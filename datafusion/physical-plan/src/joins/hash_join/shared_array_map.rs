// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: See the NOTICE file distributed with this work for additional information regarding copyright ownership
// SPDX-FileContributor: Licensed to the Apache Software Foundation (ASF) under one or more contributor license agreements

//! One [`ArrayMap`] slot array shared by all build partitions of a hash
//! partitioned join.
//!
//! Hash repartitioning on the join key spreads the keys of every build
//! partition over (nearly) the whole key range, so the keys of one partition
//! look `partitions` times sparser than those of the whole build side, and
//! an [`ArrayMap`] per partition would need one array over the whole range per
//! partition. But every key value is routed to exactly one partition, so the
//! partitions can instead fill disjoint slots of a single array over the whole
//! range, concurrently, and each probe row, routed the same way, only meets
//! slots of its own partition.
//!
//! The array can only be sized once the key range of every partition is
//! known, so each partition reports its range once it has collected its build
//! side and waits for the others: [`SharedArrayMapBuild`] is that barrier. It is
//! only created alongside a dynamic filter [`SharedBuildAccumulator`], whose
//! probe side already waits for every build partition, so all partitions are
//! known to be executed concurrently.
//!
//! [`ArrayMap`]: crate::joins::array_map::ArrayMap
//! [`SharedBuildAccumulator`]: super::shared_bounds::SharedBuildAccumulator

use std::sync::Arc;

use datafusion_common::ScalarValue;
use datafusion_execution::memory_pool::MemoryReservation;
use parking_lot::Mutex;
use tokio::sync::Notify;

use crate::joins::array_map::{ArrayMap, ArrayMapSlots};

/// The join keys of one build partition, as far as the shared decision is
/// concerned.
#[derive(Debug, Clone)]
pub(super) enum PartitionKeys {
    /// No non-null key (an empty partition, or only NULL keys that are never
    /// matched).
    None,
    /// The smallest and largest non-null key.
    Range { min: ScalarValue, max: ScalarValue },
    /// The partition cannot be joined with an `ArrayMap`, so no partition shares
    /// one (e.g. NULL keys that must match under `NullEqualsNull`).
    Ineligible,
}

#[derive(Debug, Clone)]
enum Report {
    Keys {
        keys: PartitionKeys,
        num_rows: usize,
    },
    /// The partition stopped before reporting its keys.
    Canceled,
}

/// What every partition builds once all have reported.
#[derive(Debug, Clone)]
enum Decision {
    /// Every partition fills `slots`, which cover the keys from `offset` on.
    /// Partition `p` owns the row ids `bases[p] + 1..=bases[p] + rows`.
    Shared {
        slots: Arc<ArrayMapSlots>,
        offset: u64,
        bases: Arc<[u32]>,
        /// The partition whose metrics account for the slots.
        charged: usize,
    },
    /// Every partition builds its own map, as without sharing.
    NotShared,
}

/// One partition's share of a shared [`ArrayMap`] build, see
/// [`ArrayMap::try_new_in_slots`].
#[derive(Debug)]
pub(super) struct SharedArrayMapPart {
    pub(super) slots: Arc<ArrayMapSlots>,
    pub(super) offset: u64,
    pub(super) base: u32,
    /// Whether this partition's metrics account for the memory of `slots`.
    pub(super) charge_slots: bool,
}

struct State {
    reports: Vec<Option<Report>>,
    reported: usize,
    decision: Option<Decision>,
}

/// Decides, once every build partition of a hash partitioned join has
/// reported its key range, whether all of them fill one shared [`ArrayMap`]
/// slot array, and allocates it.
pub(crate) struct SharedArrayMapBuild {
    perfect_hash_join_small_build_threshold: usize,
    perfect_hash_join_min_key_density: f64,
    state: Mutex<State>,
    notify: Notify,
}

impl std::fmt::Debug for SharedArrayMapBuild {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SharedArrayMapBuild")
            .finish_non_exhaustive()
    }
}

impl SharedArrayMapBuild {
    /// A barrier for `partitions` build partitions, deciding with the same
    /// thresholds as a single build side.
    pub(crate) fn new(
        partitions: usize,
        perfect_hash_join_small_build_threshold: usize,
        perfect_hash_join_min_key_density: f64,
    ) -> Self {
        Self {
            perfect_hash_join_small_build_threshold,
            perfect_hash_join_min_key_density,
            state: Mutex::new(State {
                reports: vec![None; partitions],
                reported: 0,
                decision: None,
            }),
            notify: Notify::new(),
        }
    }

    /// Records the report of `partition`, deciding if it is the last one.
    /// `reservation` is used for the shared slots if this call decides.
    /// Returns whether this call decided and woke the other partitions.
    fn report(
        &self,
        partition: usize,
        report: Report,
        reservation: MemoryReservation,
    ) -> bool {
        let mut state = self.state.lock();
        if state.decision.is_some() || state.reports[partition].is_some() {
            return false;
        }
        state.reports[partition] = Some(report);
        state.reported += 1;
        if state.reported < state.reports.len() {
            return false;
        }
        let reports = std::mem::take(&mut state.reports);
        // Allocating zeroed slots is cheap: their pages are only touched by
        // the partitions filling them.
        state.decision = Some(self.decide(reports, partition, reservation));
        drop(state);
        self.notify.notify_waiters();
        true
    }

    async fn wait(&self) -> Decision {
        loop {
            let notified = {
                let state = self.state.lock();
                if let Some(decision) = &state.decision {
                    return decision.clone();
                }
                // Registered before the lock is released, so the decision
                // cannot be published unnoticed in between.
                self.notify.notified()
            };
            notified.await;
        }
    }

    fn decide(
        &self,
        reports: Vec<Option<Report>>,
        decider: usize,
        reservation: MemoryReservation,
    ) -> Decision {
        let mut total_rows = 0usize;
        let mut bases = Vec::with_capacity(reports.len());
        let mut range: Option<(ScalarValue, ScalarValue)> = None;
        let mut partitions_with_keys = 0;
        for report in reports {
            let Some(Report::Keys { keys, num_rows }) = report else {
                return Decision::NotShared;
            };
            let Ok(base) = u32::try_from(total_rows) else {
                return Decision::NotShared;
            };
            bases.push(base);
            total_rows += num_rows;
            match keys {
                PartitionKeys::None => {}
                PartitionKeys::Ineligible => return Decision::NotShared,
                PartitionKeys::Range { min, max } => {
                    partitions_with_keys += 1;
                    range = Some(match range {
                        None => (min, max),
                        Some((lo, hi)) => (
                            if min < lo { min } else { lo },
                            if max > hi { max } else { hi },
                        ),
                    });
                }
            }
        }
        // Row ids are `base + row + 1` and must stay below `u32::MAX`.
        if total_rows >= u32::MAX as usize {
            return Decision::NotShared;
        }
        // A single partition with keys gains nothing from sharing.
        if partitions_with_keys < 2 {
            return Decision::NotShared;
        }
        let Some((min, max)) = range else {
            return Decision::NotShared;
        };
        let Some((min, max)) = ArrayMap::key_to_u64(&min).zip(ArrayMap::key_to_u64(&max))
        else {
            return Decision::NotShared;
        };
        let range = ArrayMap::calculate_range(min, max);
        if range >= usize::MAX as u64 {
            return Decision::NotShared;
        }
        let dense_ratio = total_rows as f64 / (range as f64 + 1.0);
        if range >= self.perfect_hash_join_small_build_threshold as u64
            && dense_ratio <= self.perfect_hash_join_min_key_density
        {
            return Decision::NotShared;
        }
        let len = (range + 1) as usize;
        let Some(size) = len.checked_mul(size_of::<u32>()) else {
            return Decision::NotShared;
        };
        // Without the memory, each partition falls back to its own map.
        if reservation.try_grow(size).is_err() {
            return Decision::NotShared;
        }
        Decision::Shared {
            slots: Arc::new(ArrayMapSlots::zeroed(len, Some(reservation))),
            offset: min,
            bases: bases.into(),
            charged: decider,
        }
    }
}

/// One build partition's handle on a [`SharedArrayMapBuild`]. Reports the
/// partition as canceled if it is dropped before reporting, so that the other
/// partitions never wait for it.
pub(super) struct SharedArrayMapHandle {
    build: Arc<SharedArrayMapBuild>,
    partition: usize,
    reported: bool,
}

impl SharedArrayMapHandle {
    pub(super) fn new(build: Arc<SharedArrayMapBuild>, partition: usize) -> Self {
        Self {
            build,
            partition,
            reported: false,
        }
    }

    /// Reports this partition's `keys` and `num_rows` build rows, waits for every
    /// other partition, and returns this partition's share of the shared map,
    /// or `None` when every partition builds its own map.
    pub(super) async fn report_and_wait(
        mut self,
        keys: PartitionKeys,
        num_rows: usize,
        reservation: &MemoryReservation,
    ) -> Option<SharedArrayMapPart> {
        self.reported = true;
        let decided = self.build.report(
            self.partition,
            Report::Keys { keys, num_rows },
            reservation.new_empty(),
        );
        if decided {
            // Every partition now inserts its rows without yielding. Tokio
            // runs the last partition woken by this worker next on this same
            // worker, where no idle worker can steal it, so it would wait for
            // this partition's whole insertion: yield to let it start first,
            // putting this partition where an idle worker can pick it up.
            tokio::task::yield_now().await;
        }
        match self.build.wait().await {
            Decision::Shared {
                slots,
                offset,
                bases,
                charged,
            } => Some(SharedArrayMapPart {
                slots,
                offset,
                base: bases[self.partition],
                charge_slots: charged == self.partition,
            }),
            Decision::NotShared => None,
        }
    }
}

impl Drop for SharedArrayMapHandle {
    fn drop(&mut self) {
        if !self.reported {
            self.build.report_canceled(self.partition);
        }
    }
}

impl SharedArrayMapBuild {
    fn report_canceled(&self, partition: usize) {
        let mut state = self.state.lock();
        if state.decision.is_some() || state.reports[partition].is_some() {
            return;
        }
        state.reports[partition] = Some(Report::Canceled);
        state.reported += 1;
        if state.reported == state.reports.len() {
            // A canceled partition rules sharing out: decide without allocating.
            state.reports.clear();
            state.decision = Some(Decision::NotShared);
            drop(state);
            self.notify.notify_waiters();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use datafusion_common_runtime::SpawnedTask;
    use datafusion_execution::memory_pool::{
        GreedyMemoryPool, MemoryConsumer, MemoryPool, UnboundedMemoryPool,
    };

    fn reservation(pool: &Arc<dyn MemoryPool>) -> MemoryReservation {
        MemoryConsumer::new("test").register(pool)
    }

    fn range(min: i32, max: i32) -> PartitionKeys {
        PartitionKeys::Range {
            min: ScalarValue::Int32(Some(min)),
            max: ScalarValue::Int32(Some(max)),
        }
    }

    /// Reports `reports` (in order, from partitions `0..`) concurrently and
    /// returns what each partition got.
    async fn run(
        reports: Vec<(PartitionKeys, usize)>,
        pool: &Arc<dyn MemoryPool>,
    ) -> Vec<Option<SharedArrayMapPart>> {
        let build = Arc::new(SharedArrayMapBuild::new(reports.len(), 1024, 0.15));
        let tasks = reports
            .into_iter()
            .enumerate()
            .map(|(partition, (keys, rows))| {
                let handle = SharedArrayMapHandle::new(Arc::clone(&build), partition);
                let reservation = reservation(pool);
                SpawnedTask::spawn(async move {
                    handle.report_and_wait(keys, rows, &reservation).await
                })
            })
            .collect::<Vec<_>>();
        let mut parts = vec![];
        for task in tasks {
            parts.push(task.join().await.unwrap());
        }
        parts
    }

    #[tokio::test]
    async fn dense_overall_is_shared() {
        let pool: Arc<dyn MemoryPool> = Arc::new(UnboundedMemoryPool::default());
        // 4 x 1000 rows over 0..=9999: 0.4 dense overall, 0.1 per partition.
        let parts = run(
            vec![
                (range(0, 9999), 1000),
                (range(3, 9990), 1000),
                (PartitionKeys::None, 0),
                (range(-5, 9000), 1000),
                (range(7, 9998), 1000),
            ],
            &pool,
        )
        .await;
        let parts = parts.into_iter().map(Option::unwrap).collect::<Vec<_>>();
        assert_eq!(
            parts.iter().map(|p| p.base).collect::<Vec<_>>(),
            vec![0, 1000, 2000, 2000, 3000]
        );
        assert!(parts.iter().all(|p| Arc::ptr_eq(&p.slots, &parts[0].slots)));
        assert!(parts.iter().all(|p| p.offset == -5i64 as u64));
        assert_eq!(parts.iter().filter(|p| p.charge_slots).count(), 1);
        // Signed range -5..=9999.
        assert_eq!(parts[0].slots.len(), 10005);
        // The slots are reserved once.
        assert_eq!(pool.reserved(), 10005 * 4);
        drop(parts);
        assert_eq!(pool.reserved(), 0);
    }

    #[tokio::test]
    async fn not_shared() {
        let pool: Arc<dyn MemoryPool> = Arc::new(UnboundedMemoryPool::default());
        // Sparse overall.
        let sparse = run(
            vec![(range(0, 99_999), 1000), (range(1, 99_998), 1000)],
            &pool,
        )
        .await;
        // A partition that cannot use an ArrayMap.
        let ineligible = run(
            vec![(range(0, 99), 50), (PartitionKeys::Ineligible, 50)],
            &pool,
        )
        .await;
        // Only one partition holds keys.
        let single =
            run(vec![(range(0, 99), 50), (PartitionKeys::None, 50)], &pool).await;
        for parts in [sparse, ineligible, single] {
            assert!(parts.iter().all(Option::is_none));
        }
        assert_eq!(pool.reserved(), 0);
    }

    #[tokio::test]
    async fn memory_limit_falls_back() {
        let pool: Arc<dyn MemoryPool> = Arc::new(GreedyMemoryPool::new(1000));
        let parts = run(vec![(range(0, 999), 500), (range(1, 998), 500)], &pool).await;
        assert!(parts.iter().all(Option::is_none));
        assert_eq!(pool.reserved(), 0);
    }

    #[tokio::test]
    async fn canceled_partition_releases_waiters() {
        let pool: Arc<dyn MemoryPool> = Arc::new(UnboundedMemoryPool::default());
        let build = Arc::new(SharedArrayMapBuild::new(2, 1024, 0.15));
        let handle = SharedArrayMapHandle::new(Arc::clone(&build), 0);
        let reservation = reservation(&pool);
        let waiter = SpawnedTask::spawn(async move {
            handle.report_and_wait(range(0, 9), 10, &reservation).await
        });
        drop(SharedArrayMapHandle::new(Arc::clone(&build), 1));
        assert!(waiter.join().await.unwrap().is_none());
    }
}
