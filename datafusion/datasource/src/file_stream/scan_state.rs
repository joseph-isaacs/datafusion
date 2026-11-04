// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: See the NOTICE file distributed with this work for additional information regarding copyright ownership
// SPDX-FileContributor: Licensed to the Apache Software Foundation (ASF) under one or more contributor license agreements

use datafusion_common::internal_datafusion_err;
use std::collections::VecDeque;
use std::task::{Context, Poll};

use crate::morsel::{Morsel, MorselPlanner, Morselizer, PendingMorselPlanner};
use arrow::record_batch::RecordBatch;
use datafusion_common::{DataFusionError, Result};
use datafusion_common_runtime::SpawnedTask;
use datafusion_execution::memory_pool::MemoryReservation;
use datafusion_physical_plan::metrics::ScopedTimerGuard;
use futures::stream::BoxStream;
use futures::{FutureExt as _, StreamExt as _};
use std::sync::Arc;

use super::work_source::WorkSource;
use super::{FileStreamMetrics, OnError};

/// State [`FileStreamState::Scan`].
///
/// There is one `ScanState` per `FileStream`, and thus per output partition.
///
/// It groups together the lifecycle of scanning that partition's files:
/// unopened files, CPU-ready planners, pending planner I/O, ready morsels,
/// the active reader, and the metrics associated with processing that work.
///
/// # I/O
///
/// To avoid challenges controlling buffering, the ScanState only ever has a
/// single I/O outstanding at any time.
///
/// # State Transitions
///
/// ```text
/// work_source
///    |
///    v
/// morselizer.plan_file(file)
///    |
///    v
/// ready_planners ---> plan() ---> ready_morsels ---> into_stream() ---> reader ---> RecordBatches
///       ^               |
///       |               v
///       |          pending_planner
///       |               |
///       |               v
///       +-------- poll until ready
/// ```
///
/// [`FileStreamState::Scan`]: super::FileStreamState::Scan
pub(super) struct ScanState {
    /// Unopened files that still need to be planned for this stream.
    work_source: WorkSource,
    /// Remaining row limit, if any.
    remain: Option<usize>,
    /// The morselizer used to plan files.
    morselizer: Box<dyn Morselizer>,
    /// Behavior if opening or scanning a file fails.
    on_error: OnError,
    /// CPU-ready planners for the current file.
    ready_planners: VecDeque<Box<dyn MorselPlanner>>,
    /// Ready morsels for the current file.
    ready_morsels: VecDeque<Box<dyn Morsel>>,
    /// The active reader, if any.
    reader: Option<BoxStream<'static, Result<RecordBatch>>>,
    /// The single planner currently blocked on I/O, if any.
    ///
    /// Once the I/O completes, yields the next planner and is pushed back
    /// onto `ready_planners`.
    pending_planner: Option<PendingMorselPlanner>,
    /// Metrics for the active scan queues.
    metrics: FileStreamMetrics,
    /// Where batches decoded ahead by a morsel opened in advance are
    /// reserved while they wait for the consumer, if the stream has a pool.
    lookahead_reservation: Option<MemoryReservation>,
}

impl ScanState {
    pub(super) fn new(
        work_source: WorkSource,
        remain: Option<usize>,
        morselizer: Box<dyn Morselizer>,
        on_error: OnError,
        metrics: FileStreamMetrics,
        lookahead_reservation: Option<MemoryReservation>,
    ) -> Self {
        Self {
            work_source,
            remain,
            morselizer,
            on_error,
            ready_planners: Default::default(),
            ready_morsels: Default::default(),
            reader: None,
            pending_planner: None,
            metrics,
            lookahead_reservation,
        }
    }

    /// Updates how scan errors are handled while the stream is still active.
    pub(super) fn set_on_error(&mut self, on_error: OnError) {
        self.on_error = on_error;
    }

    /// Drives one iteration of the active scan state.
    ///
    /// Work is attempted in this order:
    /// 1. resolve any pending planner I/O
    /// 2. poll the active reader
    /// 3. turn a ready morsel into the active reader
    /// 4. run CPU planning on a ready planner
    /// 5. morselize the next unopened file
    ///
    /// The return [`ScanAndReturn`] tells `poll_inner` how to update the
    /// outer `FileStreamState`.
    pub(super) fn poll_scan(&mut self, cx: &mut Context<'_>) -> ScanAndReturn {
        // A clone (sharing the metric), so the guard does not borrow `self`.
        let time_processing = self.metrics.time_processing.clone();
        let _processing_timer: ScopedTimerGuard<'_> = time_processing.timer();

        // Try and resolve outstanding IO first. If it is still pending, check
        // the current reader or ready morsels before yielding. New planning
        // work must still wait for this I/O to resolve.
        if let Some(mut pending_planner) = self.pending_planner.take() {
            match pending_planner.poll_unpin(cx) {
                // IO is still pending
                Poll::Pending => {
                    self.pending_planner = Some(pending_planner);
                }
                // IO resolved, and the planner is ready for CPU work
                Poll::Ready(Ok(planner)) => {
                    self.ready_planners.push_back(planner);
                }
                // IO Error
                Poll::Ready(Err(err)) => {
                    self.metrics.file_open_errors.add(1);
                    self.metrics.time_opening.stop();
                    return match self.on_error {
                        OnError::Skip => {
                            self.metrics.files_processed.add(1);
                            ScanAndReturn::Continue
                        }
                        OnError::Fail => ScanAndReturn::Error(err),
                    };
                }
            }
        }

        // Next try and get the next batch from the active reader, if any.
        if let Some(reader) = self.reader.as_mut() {
            match reader.poll_next_unpin(cx) {
                // Morsels should ideally only expose ready-to-decode streams,
                // but tolerate pending readers here.
                Poll::Pending => return ScanAndReturn::Return(Poll::Pending),
                Poll::Ready(Some(Ok(batch))) => {
                    self.metrics.time_scanning_until_data.stop();
                    self.metrics.time_scanning_total.stop();
                    // Apply any remaining row limit.
                    let (batch, finished) = match &mut self.remain {
                        Some(remain) => {
                            if *remain > batch.num_rows() {
                                *remain -= batch.num_rows();
                                self.metrics.time_scanning_total.start();
                                (batch, false)
                            } else {
                                let batch = batch.slice(0, *remain);
                                let done = 1 + self.work_source.skipped_on_limit();
                                self.metrics.files_processed.add(done);
                                *remain = 0;
                                (batch, true)
                            }
                        }
                        None => {
                            self.metrics.time_scanning_total.start();
                            self.plan_ahead();
                            (batch, false)
                        }
                    };
                    return if finished {
                        ScanAndReturn::Done(Some(Ok(batch)))
                    } else {
                        ScanAndReturn::Return(Poll::Ready(Some(Ok(batch))))
                    };
                }
                Poll::Ready(Some(Err(err))) => {
                    self.reader = None;
                    self.metrics.file_scan_errors.add(1);
                    self.metrics.time_scanning_until_data.stop();
                    self.metrics.time_scanning_total.stop();
                    return match self.on_error {
                        OnError::Skip => {
                            self.metrics.files_processed.add(1);
                            ScanAndReturn::Continue
                        }
                        OnError::Fail => ScanAndReturn::Error(err),
                    };
                }
                Poll::Ready(None) => {
                    self.reader = None;
                    self.metrics.files_processed.add(1);
                    self.metrics.time_scanning_until_data.stop();
                    self.metrics.time_scanning_total.stop();
                    return ScanAndReturn::Continue;
                }
            }
        }

        // No active reader, but a morsel is ready to become the reader.
        if let Some(morsel) = self.ready_morsels.pop_front() {
            self.metrics.time_opening.stop();
            self.metrics.time_scanning_until_data.start();
            self.metrics.time_scanning_total.start();
            self.reader = Some(morsel.into_stream());
            return ScanAndReturn::Continue;
        }

        // Do not start CPU planning or open another file while planner I/O is
        // still outstanding because they may need additional IO and ScanState
        // currently only permits a single outstanding IO
        if self.pending_planner.is_some() {
            return ScanAndReturn::Return(Poll::Pending);
        }

        // No reader or morsel, so try to produce more work via CPU planning.
        if let Some(planner) = self.ready_planners.pop_front() {
            return match planner.plan() {
                Ok(Some(mut plan)) => {
                    // Queue any newly-ready morsels, planners, or planner I/O.
                    self.ready_morsels.extend(plan.take_morsels());
                    self.ready_planners.extend(plan.take_ready_planners());
                    if let Some(pending_planner) = plan.take_pending_planner() {
                        // should not have planned if we have outstanding I/O
                        if self.pending_planner.is_some() {
                            return ScanAndReturn::Error(internal_datafusion_err!(
                                "Conflicting pending planner state in FileStream ScanState"
                            ));
                        }
                        self.pending_planner = Some(pending_planner);
                    }
                    ScanAndReturn::Continue
                }
                Ok(None) => {
                    self.metrics.files_processed.add(1);
                    self.metrics.time_opening.stop();
                    ScanAndReturn::Continue
                }
                Err(err) => {
                    self.metrics.file_open_errors.add(1);
                    self.metrics.time_opening.stop();
                    match self.on_error {
                        OnError::Skip => {
                            self.metrics.files_processed.add(1);
                            ScanAndReturn::Continue
                        }
                        OnError::Fail => ScanAndReturn::Error(err),
                    }
                }
            };
        }

        // No outstanding work remains, so begin planning the next unopened file.
        let Some(part_file) = self.work_source.pop_front() else {
            return ScanAndReturn::Done(None);
        };

        self.metrics.time_opening.start();
        match self.morselizer.plan_file(part_file) {
            Ok(planner) => {
                self.metrics.files_opened.add(1);
                self.ready_planners.push_back(planner);
                ScanAndReturn::Continue
            }
            Err(err) => match self.on_error {
                OnError::Skip => {
                    self.metrics.file_open_errors.add(1);
                    self.metrics.time_opening.stop();
                    self.metrics.files_processed.add(1);
                    ScanAndReturn::Continue
                }
                OnError::Fail => ScanAndReturn::Error(err),
            },
        }
    }
}

impl ScanState {
    /// While the active reader still has batches, take one step towards the
    /// next file's morsel, and start decoding it in the background.
    ///
    /// A partition reads its files one after another, and a file's reader
    /// decodes only when polled, so without this the next file is not even
    /// opened until the current one is drained, and a partition never has
    /// more than one file decoding at once. That leaves the scan of an
    /// ordered partition, which has to read its files in order on a single
    /// stream, as one serial chain of decode work. Each step here runs only
    /// when nothing else is queued or in flight (keeping the single
    /// outstanding I/O), so at most one file is ever opened ahead, and its
    /// stream is spawned with [`spawn_morsel`], which reads at most
    /// [`LOOKAHEAD_BATCHES`] batches ahead. Files still start in order, so
    /// the partition's batches keep their order.
    ///
    /// Not used with a row limit (extra files may never be needed) or
    /// outside a multi-threaded runtime (nowhere to run the decode).
    fn plan_ahead(&mut self) {
        if !multi_thread_runtime()
            || !self.ready_morsels.is_empty()
            || self.pending_planner.is_some()
        {
            return;
        }
        if let Some(planner) = self.ready_planners.pop_front() {
            match planner.plan() {
                Ok(Some(mut plan)) => {
                    let reservation = self.lookahead_reservation.as_ref();
                    self.ready_morsels.extend(
                        plan.take_morsels()
                            .into_iter()
                            .map(|morsel| spawn_morsel(morsel, reservation)),
                    );
                    self.ready_planners.extend(plan.take_ready_planners());
                    self.pending_planner = plan.take_pending_planner();
                }
                Ok(None) => self.metrics.files_processed.add(1),
                // Leave the error to the regular path: re-plan the file then.
                Err(_) => {}
            }
            return;
        }
        let Some(part_file) = self.work_source.pop_front() else {
            return;
        };
        match self.morselizer.plan_file(part_file) {
            Ok(planner) => {
                self.metrics.files_opened.add(1);
                self.ready_planners.push_back(planner);
            }
            // Surface the error as the regular path would, after the active
            // reader: queue a planner that fails when planned.
            Err(err) => self
                .ready_planners
                .push_back(Box::new(FailedPlanner(Some(err)))),
        }
    }
}

/// Batches a morsel opened ahead may decode before the scan reaches it. They
/// are reserved in the stream's memory pool while they wait; the depth stays
/// small all the same.
const LOOKAHEAD_BATCHES: usize = 2;

fn multi_thread_runtime() -> bool {
    tokio::runtime::Handle::try_current().is_ok_and(|handle| {
        handle.runtime_flavor() == tokio::runtime::RuntimeFlavor::MultiThread
    })
}

/// Decodes `morsel` in its own task, up to [`LOOKAHEAD_BATCHES`] ahead of its
/// consumer. Dropping the returned morsel, or its stream, stops the task.
///
/// With a `reservation`, each batch decoded ahead is reserved from it until
/// the consumer takes the batch, so the memory pool sees what the lookahead
/// holds; dropping the morsel's stream releases what is still waiting.
fn spawn_morsel(
    morsel: Box<dyn Morsel>,
    reservation: Option<&MemoryReservation>,
) -> Box<dyn Morsel> {
    let (tx, rx) = tokio::sync::mpsc::channel(LOOKAHEAD_BATCHES);
    let mut stream = morsel.into_stream();
    let held = reservation.map(|r| Arc::new(r.new_empty()));
    let produced = held.clone();
    let task = SpawnedTask::spawn(async move {
        // Reserve a slot before decoding, so at most `LOOKAHEAD_BATCHES`
        // batches exist ahead of the consumer.
        while let Ok(permit) = tx.reserve().await {
            match stream.next().await {
                Some(item) => {
                    if let (Some(held), Ok(batch)) = (&produced, &item) {
                        held.grow(batch.get_array_memory_size());
                    }
                    permit.send(item)
                }
                None => break,
            }
        }
    });
    let stream =
        futures::stream::unfold((rx, task, held), |(mut rx, task, held)| async move {
            let item = rx.recv().await?;
            if let (Some(held), Ok(batch)) = (&held, &item) {
                held.shrink(batch.get_array_memory_size());
            }
            Some((item, (rx, task, held)))
        });
    Box::new(SpawnedMorsel(stream.boxed()))
}

struct SpawnedMorsel(BoxStream<'static, Result<RecordBatch>>);

impl std::fmt::Debug for SpawnedMorsel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SpawnedMorsel").finish_non_exhaustive()
    }
}

impl Morsel for SpawnedMorsel {
    fn into_stream(self: Box<Self>) -> BoxStream<'static, Result<RecordBatch>> {
        self.0
    }
}

/// A planner standing in for a file whose planning failed while it was
/// being opened ahead, so the error surfaces in order.
#[derive(Debug)]
struct FailedPlanner(Option<DataFusionError>);

impl MorselPlanner for FailedPlanner {
    fn plan(mut self: Box<Self>) -> Result<Option<crate::morsel::MorselPlan>> {
        Err(self
            .0
            .take()
            .unwrap_or_else(|| internal_datafusion_err!("FailedPlanner planned twice")))
    }
}

/// What should be done on the next iteration of [`ScanState::poll_scan`]?
pub(super) enum ScanAndReturn {
    /// Poll again.
    Continue,
    /// Return the provided result without changing the outer state.
    Return(Poll<Option<Result<RecordBatch>>>),
    /// Update the outer `FileStreamState` to `Done` and return the provided result.
    Done(Option<Result<RecordBatch>>),
    /// Update the outer `FileStreamState` to `Error` and return the provided error.
    Error(DataFusionError),
}
