// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: See the NOTICE file distributed with this work for additional information regarding copyright ownership
// SPDX-FileContributor: Licensed to the Apache Software Foundation (ASF) under one or more contributor license agreements

//! Experimental: [`JoinHashMapU32`](super::join_hash_map::JoinHashMapU32) on a
//! local fork of hashbrown's `HashTable` that adds `prefetch(hash)`.
//!
//! The layout and insert path are the same as `JoinHashMapU32`. Lookups are
//! software pipelined: while probe row `i` is looked up, the memory that the
//! lookup of row `i + PREFETCH_DISTANCE` reads first is already being loaded.
//! The control group is always prefetched; the first data slot only while at
//! least a quarter of recent lookups hit, since a miss never reads it.

use std::fmt::{self, Debug};

use arrow::array::BooleanArray;
use arrow::buffer::{BooleanBuffer, NullBuffer};
use hashbrown_pipelined::HashTable;
use hashbrown_pipelined::hash_table::Entry::{Occupied, Vacant};

use crate::joins::chain::traverse_chain;
use crate::joins::join_hash_map::JoinHashMapType;
use crate::joins::join_hash_map::ProbeOffset::{self, AtRow, MidChain};

/// How many probe rows ahead of the current lookup to prefetch.
const PREFETCH_DISTANCE: usize = 16;

/// Number of lookups over which the hit rate is measured.
const HIT_RATE_WINDOW: u32 = 64;

/// Decides from the recent hit rate whether to prefetch data slots.
struct HitRate {
    data: bool,
    lookups: u32,
    hits: u32,
}

impl HitRate {
    fn new() -> Self {
        Self {
            data: true,
            lookups: 0,
            hits: 0,
        }
    }

    #[inline(always)]
    fn record(&mut self, hit: bool) {
        self.lookups += 1;
        self.hits += hit as u32;
        if self.lookups == HIT_RATE_WINDOW {
            self.data = self.hits * 4 >= HIT_RATE_WINDOW;
            self.lookups = 0;
            self.hits = 0;
        }
    }
}

/// See the [module docs](self).
///
/// # Public Only for Internal Use:
///
/// This is not a public API and is for internal use only.
#[doc(hidden)]
pub struct PipelinedJoinHashMapU32 {
    /// Hash value to the last inserted row index + 1.
    map: HashTable<(u64, u32)>,
    /// Chain of earlier row indices + 1 with the same hash, 0 ends a chain.
    next: Vec<u32>,
    /// Whether any hash value maps to more than one row.
    chained: bool,
}

impl PipelinedJoinHashMapU32 {
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            map: HashTable::with_capacity(cap),
            next: vec![0; cap],
            chained: false,
        }
    }

    #[inline(always)]
    fn find(&self, hash: u64) -> Option<u32> {
        self.map
            .find(hash, |&(h, _)| h == hash)
            .map(|&(_, idx)| idx)
    }

    /// Prefetches the lookup of `hash_values[row]`, if it exists.
    #[inline(always)]
    fn prefetch_row(&self, hash_values: &[u64], row: usize, data: bool) {
        if let Some(&hash) = hash_values.get(row) {
            self.map.prefetch(hash, data);
        }
    }

    /// Starts the pipeline at `row`.
    #[inline(always)]
    fn prefetch_window(&self, hash_values: &[u64], row: usize, data: bool) {
        for &hash in hash_values.iter().skip(row).take(PREFETCH_DISTANCE) {
            self.map.prefetch(hash, data);
        }
    }
}

impl Debug for PipelinedJoinHashMapU32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("PipelinedJoinHashMapU32")
            .field("len", &self.map.len())
            .finish()
    }
}

impl JoinHashMapType for PipelinedJoinHashMapU32 {
    fn extend_zero(&mut self, _: usize) {}

    fn update_from_iter<'a>(
        &mut self,
        iter: Box<dyn Iterator<Item = (usize, &'a u64)> + Send + 'a>,
        deleted_offset: usize,
    ) {
        for (row, &hash) in iter {
            let entry = self.map.entry(hash, |&(h, _)| h == hash, |&(h, _)| h);
            let index = u32::try_from(row + 1).expect("row index exceeds u32");
            match entry {
                Occupied(mut occupied) => {
                    let (_, last) = occupied.get_mut();
                    self.next[row - deleted_offset] = *last;
                    *last = index;
                    self.chained = true;
                }
                Vacant(vacant) => {
                    vacant.insert((hash, index));
                }
            }
        }
    }

    fn get_matched_indices<'a>(
        &self,
        iter: Box<dyn Iterator<Item = (usize, &'a u64)> + 'a>,
        deleted_offset: Option<usize>,
    ) -> (Vec<u32>, Vec<u64>) {
        assert!(deleted_offset.unwrap_or(0) == 0, "pruning is not supported");
        let mut input_indices = vec![];
        let mut match_indices = vec![];
        for (row, &hash) in iter {
            let Some(mut idx) = self.find(hash) else {
                continue;
            };
            while idx != 0 {
                input_indices.push(row as u32);
                match_indices.push((idx - 1) as u64);
                idx = self.next[(idx - 1) as usize];
            }
        }
        (input_indices, match_indices)
    }

    fn get_matched_indices_with_limit_offset(
        &self,
        hash_values: &[u64],
        valid_keys: Option<&NullBuffer>,
        limit: usize,
        offset: ProbeOffset,
        input_indices: &mut Vec<u32>,
        match_indices: &mut Vec<u64>,
    ) -> Option<ProbeOffset> {
        input_indices.clear();
        match_indices.clear();
        let num_probe_rows = hash_values.len();

        if !self.chained {
            // Same as `JoinHashMapU32`: the limit counts probe rows.
            let start = match offset {
                AtRow { row } | MidChain { row, .. } => row,
            };
            let end = (start + limit).min(num_probe_rows);
            let hash_values = &hash_values[..end];
            let mut hit_rate = HitRate::new();
            self.prefetch_window(hash_values, start, hit_rate.data);
            for row in start..end {
                self.prefetch_row(hash_values, row + PREFETCH_DISTANCE, hit_rate.data);
                if valid_keys.is_some_and(|valid| valid.is_null(row)) {
                    continue;
                }
                let found = self.find(hash_values[row]);
                hit_rate.record(found.is_some());
                if let Some(idx) = found {
                    input_indices.push(row as u32);
                    match_indices.push((idx - 1) as u64);
                }
            }
            return (end < num_probe_rows).then_some(AtRow { row: end });
        }

        let mut remaining = limit;
        let start = match offset {
            AtRow { row } => row,
            MidChain { row, next } => {
                let is_last = row == num_probe_rows - 1;
                if let Some(next_offset) = traverse_chain(
                    &self.next,
                    row,
                    next as u32,
                    &mut remaining,
                    input_indices,
                    match_indices,
                    is_last,
                ) {
                    return Some(next_offset);
                }
                row + 1
            }
        };
        let mut hit_rate = HitRate::new();
        self.prefetch_window(hash_values, start, hit_rate.data);
        for row in start..num_probe_rows {
            self.prefetch_row(hash_values, row + PREFETCH_DISTANCE, hit_rate.data);
            if valid_keys.is_some_and(|valid| valid.is_null(row)) {
                continue;
            }
            let found = self.find(hash_values[row]);
            hit_rate.record(found.is_some());
            if let Some(idx) = found {
                let is_last = row == num_probe_rows - 1;
                if let Some(next_offset) = traverse_chain(
                    &self.next,
                    row,
                    idx,
                    &mut remaining,
                    input_indices,
                    match_indices,
                    is_last,
                ) {
                    return Some(next_offset);
                }
            }
        }
        None
    }

    fn contain_hashes(&self, hash_values: &[u64]) -> BooleanArray {
        let mut hit_rate = HitRate::new();
        self.prefetch_window(hash_values, 0, hit_rate.data);
        let buffer = BooleanBuffer::collect_bool(hash_values.len(), |i| {
            self.prefetch_row(hash_values, i + PREFETCH_DISTANCE, hit_rate.data);
            let hit = self.find(hash_values[i]).is_some();
            hit_rate.record(hit);
            hit
        });
        BooleanArray::new(buffer, None)
    }

    fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    fn len(&self) -> usize {
        self.map.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::joins::join_hash_map::JoinHashMapU32;

    fn probe_all(
        map: &dyn JoinHashMapType,
        probe: &[u64],
        limit: usize,
    ) -> Vec<(u32, u64)> {
        let (mut input, mut matched, mut out) = (vec![], vec![], vec![]);
        let mut offset = AtRow { row: 0 };
        loop {
            let next = map.get_matched_indices_with_limit_offset(
                probe,
                None,
                limit,
                offset,
                &mut input,
                &mut matched,
            );
            out.extend(input.iter().copied().zip(matched.iter().copied()));
            match next {
                Some(next) => offset = next,
                None => return out,
            }
        }
    }

    #[test]
    fn matches_join_hash_map_u32() {
        for dups in [1u64, 3] {
            let build: Vec<u64> =
                (0..2000u64).map(|i| (i / dups) * 0x9E37_79B9).collect();
            let probe: Vec<u64> = (0..500u64).map(|i| (i * 7) * 0x9E37_79B9).collect();
            let mut stock = JoinHashMapU32::with_capacity(build.len());
            stock.update_from_iter(Box::new(build.iter().enumerate().rev()), 0);
            let mut pipelined = PipelinedJoinHashMapU32::with_capacity(build.len());
            pipelined.update_from_iter(Box::new(build.iter().enumerate().rev()), 0);
            assert_eq!(
                stock.contain_hashes(&probe),
                pipelined.contain_hashes(&probe)
            );
            for limit in [1, 7, 8192] {
                assert_eq!(
                    probe_all(&stock, &probe, limit),
                    probe_all(&pipelined, &probe, limit)
                );
            }
        }
    }
}
