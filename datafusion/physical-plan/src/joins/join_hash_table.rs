// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: See the NOTICE file distributed with this work for additional information regarding copyright ownership
// SPDX-FileContributor: Licensed to the Apache Software Foundation (ASF) under one or more contributor license agreements

//! An open-addressing join hash table specialized for the build side of
//! [`HashJoinExec`](crate::joins::HashJoinExec).
//!
//! It implements [`JoinHashMapType`] with the same chained-row semantics as
//! [`JoinHashMapU32`](crate::joins::join_hash_map::JoinHashMapU32), but with a
//! layout tuned for the hash join:
//!
//! * The table is an array of 64-byte, cache line aligned buckets of eight
//!   slots. A slot holds the upper 32 bits of the row's hash (the *tag*, with
//!   its lowest bit forced to `1`) and the 1-based index of the newest build
//!   row with that tag; a tag of `0` is an empty slot. A row's home bucket is
//!   chosen by the lower hash bits.
//! * Slots of a bucket are filled in order, so a bucket whose last slot is
//!   empty is not full. A lookup compares all eight tags of the home bucket
//!   without branching: it either finds the tag, or proves a miss when the
//!   bucket is not full. Only rows of full buckets continue to the next
//!   bucket. A lookup therefore usually touches a single cache line, and
//!   unlike `hashbrown` there is no separate control byte array.
//! * The table has twice as many slots as build rows, so few buckets fill up.
//! * Inserts and lookups prefetch the home bucket of the row
//!   [`PREFETCH_DISTANCE`] rows ahead, overlapping the cache misses of
//!   independent rows on large build sides.
//!
//! Rows whose hashes share the tag and probe into the same bucket
//! sequence are chained together, as if their hashes were equal. Every
//! consumer of the hash join map already treats a match as a *candidate*
//! (the join keys are compared afterwards, see `equal_rows_arr`), so such a
//! merge only costs a rejected candidate, which happens with probability
//! about `2^-31` per compared slot.

use std::fmt::{self, Debug};

use arrow::array::BooleanArray;
use arrow::buffer::{BooleanBuffer, NullBuffer};

use crate::joins::chain::traverse_chain;
use crate::joins::join_hash_map::ProbeOffset::{AtRow, MidChain};
use crate::joins::join_hash_map::{JoinHashMapType, ProbeOffset};

/// Number of rows ahead whose home bucket is prefetched.
const PREFETCH_DISTANCE: usize = 16;

/// Number of slots per bucket: one 64-byte cache line of `u64` slots.
const BUCKET_SLOTS: usize = 8;

/// Hints the CPU to load `slice[index]` into the cache, if `index` is in
/// bounds. Has no other effect.
#[inline(always)]
pub(crate) fn prefetch<T>(slice: &[T], index: usize) {
    if index >= slice.len() {
        return;
    }
    #[cfg(target_arch = "x86_64")]
    {
        // SAFETY: `index < slice.len()`, and prefetching has no observable
        // effect besides cache state.
        unsafe {
            use std::arch::x86_64::{_MM_HINT_T0, _mm_prefetch};
            _mm_prefetch::<_MM_HINT_T0>(slice.as_ptr().add(index).cast());
        }
    }
}

/// Returns `n` values of type `T` whose memory is all zero bytes.
///
/// Allocates zeroed memory (`calloc`), which for a large table hands out
/// fresh pages that the OS zeroes on first touch, rather than writing every
/// element as `vec![value; n]` does for a type that is not a primitive.
///
/// # Safety
///
/// All zero bytes must be a valid value of `T`.
pub(crate) unsafe fn zeroed_vec<T>(n: usize) -> Vec<T> {
    if n == 0 || size_of::<T>() == 0 {
        return Vec::new();
    }
    let layout = std::alloc::Layout::array::<T>(n).expect("table size overflow");
    // SAFETY: `layout` has a non-zero size; the caller guarantees that the
    // zeroed memory is `n` valid values of `T`, allocated by the global
    // allocator with the layout of a `Box<[T]>` of length `n`.
    unsafe {
        let ptr = std::alloc::alloc_zeroed(layout).cast::<T>();
        if ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }
        Box::from_raw(std::ptr::slice_from_raw_parts_mut(ptr, n)).into_vec()
    }
}

/// A cache line of slots, see the [module documentation](self).
///
/// The tags and rows of the slots are stored apart, so all eight tags are
/// compared with a pair of SIMD compares.
#[derive(Clone, Copy, Default)]
#[repr(C, align(64))]
struct Bucket {
    /// Tag per slot, `0` for an empty slot (tags are never `0`).
    tags: [u32; BUCKET_SLOTS],
    /// 1-based index of the newest build row with the tag, per slot.
    rows: [u32; BUCKET_SLOTS],
}

impl Bucket {
    /// Returns a bit mask of the slots whose tag is `tag`.
    #[inline(always)]
    fn matches(&self, tag: u32) -> u32 {
        let mut mask = 0;
        for (i, &t) in self.tags.iter().enumerate() {
            mask |= ((t == tag) as u32) << i;
        }
        mask
    }

    #[inline(always)]
    fn is_full(&self) -> bool {
        self.tags[BUCKET_SLOTS - 1] != 0
    }
}

/// Bucketized join hash table with `u32` row indices, see the
/// [module documentation](self).
///
/// # Public Only for Internal Use:
///
/// This is not a public API and is for internal use only; see [API policy] for details.
///
/// [API policy]: https://datafusion.apache.org/contributor-guide/api-health.html#internal-public-apis
#[doc(hidden)]
pub struct JoinHashTableU32 {
    buckets: Vec<Bucket>,
    /// `buckets.len() - 1`
    mask: usize,
    /// Number of occupied slots, i.e. of distinct tags.
    len: usize,
    /// Chained list of build rows, see `JoinHashMapU32`.
    next: Vec<u32>,
    /// Whether any slot maps to more than one row.
    chained: bool,
}

impl JoinHashTableU32 {
    /// Creates a table that holds `cap` rows.
    pub fn with_capacity(cap: usize) -> Self {
        let num_buckets = Self::num_buckets(cap);
        Self {
            // SAFETY: a `Bucket` of zeros is a bucket of empty slots.
            buckets: unsafe { zeroed_vec(num_buckets) },
            mask: num_buckets - 1,
            len: 0,
            next: vec![0; cap],
            chained: false,
        }
    }

    /// Twice as many slots as rows, so the table is at most half full.
    fn num_buckets(cap: usize) -> usize {
        cap.saturating_mul(2)
            .div_ceil(BUCKET_SLOTS)
            .next_power_of_two()
            .max(1)
    }

    /// Estimates the memory allocated by [`Self::with_capacity`].
    pub fn estimate_memory_size(cap: usize) -> Option<usize> {
        Self::num_buckets(cap)
            .checked_mul(size_of::<Bucket>())?
            .checked_add(cap.checked_mul(size_of::<u32>())?)?
            .checked_add(size_of::<Self>())
    }

    /// The upper 32 bits of `hash`, with the lowest bit set to never be `0`.
    #[inline(always)]
    fn tag(hash: u64) -> u32 {
        ((hash >> 32) as u32) | 1
    }

    #[inline(always)]
    fn home(&self, hash: u64) -> usize {
        hash as usize & self.mask
    }

    #[inline(always)]
    fn prefetch(&self, hash: u64) {
        prefetch(&self.buckets, self.home(hash));
    }

    /// Returns the 1-based index of the newest row whose hash matches `hash`.
    #[inline(always)]
    fn find(&self, hash: u64) -> Option<u32> {
        let tag = Self::tag(hash);
        let mut pos = self.home(hash);
        loop {
            // SAFETY: `pos` is always masked by `mask`, `buckets.len() - 1`.
            let bucket = unsafe { self.buckets.get_unchecked(pos) };
            let matches = bucket.matches(tag);
            if matches != 0 {
                return Some(bucket.rows[matches.trailing_zeros() as usize]);
            }
            if !bucket.is_full() {
                return None;
            }
            pos = (pos + 1) & self.mask;
        }
    }

    /// Inserts build row `row` with hash `hash`.
    #[inline(always)]
    fn insert(&mut self, hash: u64, row: usize, deleted_offset: usize) {
        let tag = Self::tag(hash);
        let value = u32::try_from(row + 1).expect("row index fits in u32");
        let mut pos = self.home(hash);
        loop {
            // SAFETY: `pos` is always masked by `mask`, `buckets.len() - 1`.
            let bucket = unsafe { self.buckets.get_unchecked_mut(pos) };
            let matches = bucket.matches(tag);
            if matches != 0 {
                let slot = matches.trailing_zeros() as usize;
                let prev = std::mem::replace(&mut bucket.rows[slot], value);
                self.next[row - deleted_offset] = prev;
                self.chained = true;
                return;
            }
            let empty = bucket.matches(0);
            if empty != 0 {
                let slot = empty.trailing_zeros() as usize;
                bucket.tags[slot] = tag;
                bucket.rows[slot] = value;
                self.len += 1;
                return;
            }
            // The table has more slots than rows, so a free one exists.
            pos = (pos + 1) & self.mask;
        }
    }

    #[inline(always)]
    fn insert_rows(
        &mut self,
        hashes: &[u64],
        row_offset: usize,
        deleted_offset: usize,
        rows: impl Iterator<Item = usize>,
        prefetch: impl Fn(usize) -> Option<usize>,
    ) {
        for i in rows {
            if let Some(ahead) = prefetch(i) {
                self.prefetch(hashes[ahead]);
            }
            self.insert(hashes[i], row_offset + i, deleted_offset);
        }
    }
}

impl Debug for JoinHashTableU32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("JoinHashTableU32")
            .field("len", &self.len)
            .field("buckets", &self.buckets.len())
            .finish_non_exhaustive()
    }
}

impl JoinHashMapType for JoinHashTableU32 {
    fn extend_zero(&mut self, _: usize) {}

    fn update_from_iter<'a>(
        &mut self,
        iter: Box<dyn Iterator<Item = (usize, &'a u64)> + Send + 'a>,
        deleted_offset: usize,
    ) {
        for (row, &hash) in iter {
            self.insert(hash, row, deleted_offset);
        }
    }

    fn update_from_hashes(
        &mut self,
        hashes: &[u64],
        row_offset: usize,
        valid: Option<&NullBuffer>,
        deleted_offset: usize,
        reverse: bool,
    ) {
        let n = hashes.len();
        match (valid, reverse) {
            (None, true) => {
                self.insert_rows(hashes, row_offset, deleted_offset, (0..n).rev(), |i| {
                    i.checked_sub(PREFETCH_DISTANCE)
                })
            }
            (None, false) => {
                self.insert_rows(hashes, row_offset, deleted_offset, 0..n, |i| {
                    Some(i + PREFETCH_DISTANCE).filter(|&j| j < n)
                })
            }
            (Some(valid), true) => self.insert_rows(
                hashes,
                row_offset,
                deleted_offset,
                (0..n).rev().filter(|&i| valid.is_valid(i)),
                |i| i.checked_sub(PREFETCH_DISTANCE),
            ),
            (Some(valid), false) => self.insert_rows(
                hashes,
                row_offset,
                deleted_offset,
                (0..n).filter(|&i| valid.is_valid(i)),
                |i| Some(i + PREFETCH_DISTANCE).filter(|&j| j < n),
            ),
        }
    }

    fn get_matched_indices<'a>(
        &self,
        iter: Box<dyn Iterator<Item = (usize, &'a u64)> + 'a>,
        deleted_offset: Option<usize>,
    ) -> (Vec<u32>, Vec<u64>) {
        let mut input_indices = vec![];
        let mut match_indices = vec![];
        for (row_idx, &hash) in iter {
            let Some(index) = self.find(hash) else {
                continue;
            };
            let mut i = index - 1;
            loop {
                let match_row_idx = match deleted_offset {
                    Some(offset) => {
                        let offset = u32::try_from(offset).unwrap();
                        if i < offset {
                            break;
                        }
                        i - offset
                    }
                    None => i,
                };
                match_indices.push(match_row_idx as u64);
                input_indices.push(row_idx as u32);
                let next = self.next[match_row_idx as usize];
                if next == 0 {
                    break;
                }
                i = next - 1;
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
        let n = hash_values.len();

        if !self.chained {
            debug_assert!(
                matches!(offset, AtRow { .. }),
                "unique keys cannot have mid-chain"
            );
            let start = match offset {
                AtRow { row } => row,
                MidChain { row, .. } => row,
            };
            let end = (start + limit).min(n);
            for &hash in &hash_values[start..(start + PREFETCH_DISTANCE).min(end)] {
                self.prefetch(hash);
            }
            for i in start..end {
                if i + PREFETCH_DISTANCE < end {
                    self.prefetch(hash_values[i + PREFETCH_DISTANCE]);
                }
                // NULL keys cannot match any build row
                if valid_keys.is_some_and(|valid| valid.is_null(i)) {
                    continue;
                }
                if let Some(idx) = self.find(hash_values[i]) {
                    input_indices.push(i as u32);
                    match_indices.push((idx - 1) as u64);
                }
            }
            return if end == n {
                None
            } else {
                Some(AtRow { row: end })
            };
        }

        let mut remaining_output = limit;
        let to_skip = match offset {
            AtRow { row } => row,
            MidChain { row, next } => {
                let is_last = row == n - 1;
                if let Some(next_offset) = traverse_chain(
                    &self.next,
                    row,
                    next as u32,
                    &mut remaining_output,
                    input_indices,
                    match_indices,
                    is_last,
                ) {
                    return Some(next_offset);
                }
                row + 1
            }
        };

        for &hash in &hash_values[to_skip..(to_skip + PREFETCH_DISTANCE).min(n)] {
            self.prefetch(hash);
        }
        for row_idx in to_skip..n {
            if row_idx + PREFETCH_DISTANCE < n {
                self.prefetch(hash_values[row_idx + PREFETCH_DISTANCE]);
            }
            // NULL keys cannot match any build row
            if valid_keys.is_some_and(|valid| valid.is_null(row_idx)) {
                continue;
            }
            if let Some(idx) = self.find(hash_values[row_idx]) {
                let is_last = row_idx == n - 1;
                if let Some(next_offset) = traverse_chain(
                    &self.next,
                    row_idx,
                    idx,
                    &mut remaining_output,
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
        let n = hash_values.len();
        for &hash in &hash_values[..PREFETCH_DISTANCE.min(n)] {
            self.prefetch(hash);
        }
        let buffer = BooleanBuffer::collect_bool(n, |i| {
            if i + PREFETCH_DISTANCE < n {
                self.prefetch(hash_values[i + PREFETCH_DISTANCE]);
            }
            self.find(hash_values[i]).is_some()
        });
        BooleanArray::new(buffer, None)
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn len(&self) -> usize {
        self.len
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::joins::join_hash_map::JoinHashMapU32;

    /// Probes `map` in chunks of `limit` and returns all `(probe, build)` pairs.
    fn probe_all(
        map: &dyn JoinHashMapType,
        probe: &[u64],
        valid: Option<&NullBuffer>,
        limit: usize,
    ) -> Vec<(u32, u64)> {
        let mut out = vec![];
        let mut input_indices = vec![];
        let mut match_indices = vec![];
        let mut offset = AtRow { row: 0 };
        loop {
            let next = map.get_matched_indices_with_limit_offset(
                probe,
                valid,
                limit,
                offset,
                &mut input_indices,
                &mut match_indices,
            );
            out.extend(
                input_indices
                    .iter()
                    .copied()
                    .zip(match_indices.iter().copied()),
            );
            match next {
                Some(next) => offset = next,
                None => return out,
            }
        }
    }

    /// A well mixed 64-bit hash, so distinct inputs have distinct tags.
    fn mix(mut x: u64) -> u64 {
        x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
        x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        x ^ (x >> 31)
    }

    /// The table must return exactly what `JoinHashMapU32` returns.
    #[test]
    fn matches_join_hash_map_u32() {
        // Many duplicates and misses, plus hashes that share their home slot
        // (lower bits) but not their tag (upper bits).
        let key_hash = |k: u64| {
            if k.is_multiple_of(10) {
                (mix(k) & !0xFFFF_FFFF) | 5
            } else {
                mix(k)
            }
        };
        let build: Vec<u64> = (0..2000u64).map(|i| key_hash(i % 700)).collect();
        let probe: Vec<u64> = (0..3000u64).map(|i| key_hash(i % 900)).collect();
        let valid =
            NullBuffer::from((0..probe.len()).map(|i| i % 7 != 0).collect::<Vec<_>>());
        let build_valid =
            NullBuffer::from((0..build.len()).map(|i| i % 11 != 0).collect::<Vec<_>>());

        for (build_valid, reverse) in [
            (None, true),
            (None, false),
            (Some(&build_valid), true),
            (Some(&build_valid), false),
        ] {
            let mut expected = JoinHashMapU32::with_capacity(build.len());
            let mut actual = JoinHashTableU32::with_capacity(build.len());
            // Insert in two updates, as the build side does per batch.
            for (range, offset) in [(0..1000, 0), (1000..2000, 1000)] {
                let valid = build_valid.map(|v| v.slice(range.start, range.len()));
                expected.update_from_hashes(
                    &build[range.clone()],
                    offset,
                    valid.as_ref(),
                    0,
                    reverse,
                );
                actual.update_from_hashes(
                    &build[range],
                    offset,
                    valid.as_ref(),
                    0,
                    reverse,
                );
            }
            assert_eq!(expected.len(), actual.len());
            for limit in [1, 3, 64, 8192] {
                for valid in [None, Some(&valid)] {
                    assert_eq!(
                        probe_all(&expected, &probe, valid, limit),
                        probe_all(&actual, &probe, valid, limit),
                        "limit {limit}"
                    );
                }
            }
            assert_eq!(
                expected.contain_hashes(&probe),
                actual.contain_hashes(&probe)
            );
            let iter = || Box::new(probe.iter().enumerate());
            assert_eq!(
                expected.get_matched_indices(iter(), None),
                actual.get_matched_indices(iter(), None)
            );
        }
    }

    #[test]
    fn unique_and_null_build_rows() {
        let build = [10u64 << 40, 99 << 40, 20 << 40, 30 << 40];
        let valid = NullBuffer::from(vec![true, false, true, true]);
        let mut map = JoinHashTableU32::with_capacity(build.len());
        map.update_from_hashes(&build, 0, Some(&valid), 0, true);
        assert_eq!(map.len(), 3);
        let probe = [10u64 << 40, 99 << 40, 20 << 40, 30 << 40, 40 << 40];
        assert_eq!(
            probe_all(&map, &probe, None, 2),
            vec![(0, 0), (2, 2), (3, 3)]
        );
    }

    #[test]
    fn empty_table() {
        let map = JoinHashTableU32::with_capacity(0);
        assert!(map.is_empty());
        assert_eq!(probe_all(&map, &[1, 2, 3], None, 8192), vec![]);
    }
}
