// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: See the NOTICE file distributed with this work for additional information regarding copyright ownership
// SPDX-FileContributor: Licensed to the Apache Software Foundation (ASF) under one or more contributor license agreements

//! An "unchained" hash table for the build side of a hash join.
//!
//! Based on Birler, Schmidt, Fent and Neumann, *Simple, Efficient, and Robust
//! Hash Tables for Join Processing* (DaMoN 2024). A hash join collects its
//! whole build side before probing starts, so the table does not need to
//! support interleaved inserts and lookups. It is built in three passes once
//! all rows are known:
//!
//! 1. **Count**: count the rows of each bucket, selected by the top bits of
//!    the hash, and take a prefix sum.
//! 2. **Scatter**: write each row's hash and index into dense arrays grouped
//!    by bucket, so the candidates of a probe are contiguous in memory rather
//!    than spread over a linked chain.
//! 3. **Directory**: for each bucket, store its start offset in the upper 48
//!    bits of a `u64` and a 16 bit Bloom filter of the bucket's hashes in the
//!    lower 16 bits. Most probes that find no match are rejected by the filter
//!    without touching the dense arrays.
//!
//! ```text
//!  directory (2^k + 1 entries)        hashes / rows (one entry per build row)
//!  ┌───────────────┬───────┐          ┌──────┬──────┬──────┬──────┬──────┐
//!  │ start = 0     │ tag   │ ───────▶ │ h0   │ h0   │ h3   │ h7   │ ...  │
//!  ├───────────────┼───────┤          ├──────┼──────┼──────┼──────┼──────┤
//!  │ start = 3     │ tag   │ ───┐     │ r2   │ r5   │ r1   │ r4   │ ...  │
//!  ├───────────────┼───────┤    │     └──────┴──────┴──────┴──────┴──────┘
//!  │ start = 3     │ 0     │    └────────────────────────▲
//!  └───────────────┴───────┘   (empty bucket: start == next start, tag 0)
//! ```
//!
//! Inserts are buffered by [`JoinHashMapType::update_from_iter`] and the table
//! is built by [`JoinHashMapType::finalize`], which must be called before the
//! first lookup.
//!
//! Within a bucket, rows are stored in reverse insertion order. Lookups
//! therefore return the rows of a hash value in the same order as the chained
//! [`JoinHashMapU32`](super::join_hash_map::JoinHashMapU32), which emits the
//! most recently inserted row first.

use std::fmt::{self, Debug};

use arrow::array::BooleanArray;
use arrow::buffer::{BooleanBuffer, NullBuffer};
use datafusion_common::{Result, exec_datafusion_err};

use crate::joins::join_hash_map::JoinHashMapType;
use crate::joins::join_hash_map::ProbeOffset::{self, AtRow, MidChain};

/// Number of directory bits that hold the Bloom filter tag.
const TAG_BITS: u32 = 16;
const TAG_MASK: u64 = (1 << TAG_BITS) - 1;

/// Row index stored in an [`UnchainedJoinHashMap`].
///
/// # Public Only for Internal Use:
///
/// This is not a public API and is for internal use only.
#[doc(hidden)]
pub trait RowIndex: Copy + Default + Send + Sync + 'static {
    fn from_usize(value: usize) -> Self;
    fn to_u64(self) -> u64;
}

impl RowIndex for u32 {
    #[inline(always)]
    fn from_usize(value: usize) -> Self {
        u32::try_from(value).expect("row index exceeds u32")
    }

    #[inline(always)]
    fn to_u64(self) -> u64 {
        self as u64
    }
}

impl RowIndex for u64 {
    #[inline(always)]
    fn from_usize(value: usize) -> Self {
        value as u64
    }

    #[inline(always)]
    fn to_u64(self) -> u64 {
        self
    }
}

/// The Bloom filter tag of `hash`: up to four of 16 bits, chosen by the low
/// 16 bits of the hash. The bucket is chosen by the high bits, so the two
/// are independent for any table smaller than 2^48 buckets.
#[inline(always)]
fn tag(hash: u64) -> u64 {
    (1 << (hash & 15))
        | (1 << ((hash >> 4) & 15))
        | (1 << ((hash >> 8) & 15))
        | (1 << ((hash >> 12) & 15))
}

/// Number of directory buckets for `num_rows` rows: the next power of two, at
/// least 2 so the bucket shift stays below 64.
fn num_buckets(num_rows: usize) -> Option<usize> {
    num_rows.max(2).checked_next_power_of_two()
}

/// Unchained build side hash table, see the [module docs](self).
///
/// # Public Only for Internal Use:
///
/// This is not a public API and is for internal use only; see [API policy] for details.
///
/// [API policy]: https://datafusion.apache.org/contributor-guide/api-health.html#internal-public-apis
#[doc(hidden)]
pub struct UnchainedJoinHashMap<T: RowIndex> {
    /// Hashes of the inserted rows in insertion order, emptied by `finalize`.
    staged_hashes: Vec<u64>,
    /// Indices of the inserted rows in insertion order, emptied by `finalize`.
    staged_rows: Vec<T>,
    /// Bytes of staging memory sized by `with_capacity`, released by `finalize`.
    staging_size: usize,
    /// `directory[b] >> 16` is the offset of bucket `b` in `hashes` and `rows`,
    /// the low 16 bits are the Bloom filter of the bucket's hashes. The last
    /// entry holds the total row count, so bucket `b` ends at the offset of
    /// `directory[b + 1]`.
    directory: Vec<u64>,
    /// Row hashes grouped by bucket.
    hashes: Vec<u64>,
    /// Row indices, parallel to `hashes`.
    rows: Vec<T>,
    /// `hash >> shift` is the bucket of `hash`.
    shift: u32,
    /// Number of distinct hash values.
    distinct: usize,
    finalized: bool,
}

/// [`UnchainedJoinHashMap`] with `u32` row indices.
pub type UnchainedJoinHashMapU32 = UnchainedJoinHashMap<u32>;
/// [`UnchainedJoinHashMap`] with `u64` row indices.
pub type UnchainedJoinHashMapU64 = UnchainedJoinHashMap<u64>;

impl<T: RowIndex> UnchainedJoinHashMap<T> {
    const ROW_SIZE: usize = size_of::<u64>() + size_of::<T>();

    /// Creates a map that buffers up to `cap` rows without reallocating.
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            staged_hashes: Vec::with_capacity(cap),
            staged_rows: Vec::with_capacity(cap),
            staging_size: cap * Self::ROW_SIZE,
            directory: Vec::new(),
            hashes: Vec::new(),
            rows: Vec::new(),
            shift: 63,
            distinct: 0,
            finalized: false,
        }
    }

    /// Peak memory of a map for `num_rows` rows: the staging buffers plus the
    /// finished table, which coexist while `finalize` scatters the rows.
    pub fn estimate_memory_size(num_rows: usize) -> Result<usize> {
        let overflow = || exec_datafusion_err!("Hash join table size overflow");
        let buckets = num_buckets(num_rows).ok_or_else(overflow)?;
        let directory = buckets
            .checked_add(1)
            .and_then(|entries| entries.checked_mul(size_of::<u64>()))
            .ok_or_else(overflow)?;
        num_rows
            .checked_mul(Self::ROW_SIZE)
            .and_then(|rows| rows.checked_mul(2))
            .and_then(|rows| rows.checked_add(directory))
            .and_then(|total| total.checked_add(size_of::<Self>()))
            .ok_or_else(overflow)
    }

    /// Returns the range of `hashes` and `rows` that may contain `hash`, or
    /// `None` if the Bloom filter rules it out.
    #[inline(always)]
    fn candidates(&self, hash: u64) -> Option<(usize, usize)> {
        let bucket = (hash >> self.shift) as usize;
        let entry = self.directory[bucket];
        let tag = tag(hash);
        if entry & tag != tag {
            return None;
        }
        let start = (entry >> TAG_BITS) as usize;
        let end = (self.directory[bucket + 1] >> TAG_BITS) as usize;
        Some((start, end))
    }

    #[inline(always)]
    fn assert_finalized(&self) {
        assert!(
            self.finalized,
            "UnchainedJoinHashMap must be finalized before lookups"
        );
    }

    fn build(&mut self) {
        let num_rows = self.staged_hashes.len();
        let buckets = num_buckets(num_rows).expect("bucket count fits in usize");
        self.shift = 64 - buckets.trailing_zeros();
        let shift = self.shift;

        // Count: `directory[b + 1]` counts bucket `b`, so the inclusive prefix
        // sum leaves the end of bucket `b` there.
        let mut directory = vec![0u64; buckets + 1];
        for &hash in &self.staged_hashes {
            directory[(hash >> shift) as usize + 1] += 1;
        }
        for b in 1..=buckets {
            directory[b] += directory[b - 1];
        }

        // Scatter: fill each bucket from its end in insertion order, which
        // leaves the rows in reverse insertion order and `directory[b + 1]`
        // at the start of bucket `b`.
        let mut hashes = vec![0u64; num_rows];
        let mut rows = vec![T::default(); num_rows];
        for (&hash, &row) in self.staged_hashes.iter().zip(&self.staged_rows) {
            let end = &mut directory[(hash >> shift) as usize + 1];
            *end -= 1;
            hashes[*end as usize] = hash;
            rows[*end as usize] = row;
        }
        self.staged_hashes = Vec::new();
        self.staged_rows = Vec::new();

        // Directory: shift the bucket starts into place and add the tags.
        directory.rotate_left(1);
        directory[buckets] = num_rows as u64;
        let mut distinct = 0;
        for b in 0..buckets {
            let start = directory[b] as usize;
            let end = directory[b + 1] as usize;
            let mut bloom = 0;
            for i in start..end {
                bloom |= tag(hashes[i]);
                if !hashes[start..i].contains(&hashes[i]) {
                    distinct += 1;
                }
            }
            directory[b] = ((start as u64) << TAG_BITS) | bloom;
        }
        directory[buckets] <<= TAG_BITS;
        debug_assert!(directory.iter().all(|e| e >> TAG_BITS <= num_rows as u64));
        debug_assert_eq!(directory[buckets] & TAG_MASK, 0);

        self.directory = directory;
        self.hashes = hashes;
        self.rows = rows;
        self.distinct = distinct;
    }
}

impl<T: RowIndex> Debug for UnchainedJoinHashMap<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("UnchainedJoinHashMap")
            .field("rows", &self.rows.len())
            .field("buckets", &self.directory.len().saturating_sub(1))
            .field("distinct", &self.distinct)
            .finish()
    }
}

impl<T: RowIndex> JoinHashMapType for UnchainedJoinHashMap<T> {
    fn extend_zero(&mut self, _: usize) {}

    fn update_from_iter<'a>(
        &mut self,
        iter: Box<dyn Iterator<Item = (usize, &'a u64)> + Send + 'a>,
        deleted_offset: usize,
    ) {
        assert!(
            !self.finalized,
            "UnchainedJoinHashMap cannot be updated after finalize"
        );
        assert_eq!(
            deleted_offset, 0,
            "UnchainedJoinHashMap does not support pruning"
        );
        for (row, &hash) in iter {
            self.staged_hashes.push(hash);
            self.staged_rows.push(T::from_usize(row));
        }
    }

    fn finalize(&mut self) -> usize {
        if self.finalized {
            return 0;
        }
        self.build();
        self.finalized = true;
        std::mem::take(&mut self.staging_size)
    }

    fn get_matched_indices<'a>(
        &self,
        iter: Box<dyn Iterator<Item = (usize, &'a u64)> + 'a>,
        deleted_offset: Option<usize>,
    ) -> (Vec<u32>, Vec<u64>) {
        self.assert_finalized();
        assert!(
            deleted_offset.unwrap_or(0) == 0,
            "UnchainedJoinHashMap does not support pruning"
        );
        let mut input_indices = vec![];
        let mut match_indices = vec![];
        for (row, &hash) in iter {
            if let Some((start, end)) = self.candidates(hash) {
                for i in start..end {
                    if self.hashes[i] == hash {
                        input_indices.push(row as u32);
                        match_indices.push(self.rows[i].to_u64());
                    }
                }
            }
        }
        (input_indices, match_indices)
    }

    /// A `MidChain { next, .. }` offset is the 1-based position in the dense
    /// row arrays at which the scan of the probe row's bucket resumes.
    fn get_matched_indices_with_limit_offset(
        &self,
        hash_values: &[u64],
        valid_keys: Option<&NullBuffer>,
        limit: usize,
        offset: ProbeOffset,
        input_indices: &mut Vec<u32>,
        match_indices: &mut Vec<u64>,
    ) -> Option<ProbeOffset> {
        self.assert_finalized();
        input_indices.clear();
        match_indices.clear();

        let mut remaining = limit;
        let (mut row, mut resume) = match offset {
            AtRow { row } => (row, None),
            MidChain { row, next } => (row, Some(next as usize - 1)),
        };
        let num_probe_rows = hash_values.len();
        while row < num_probe_rows {
            // NULL keys cannot match any build row. A resumed row has already
            // passed this check.
            let skip =
                resume.is_none() && valid_keys.is_some_and(|valid| valid.is_null(row));
            if !skip {
                let hash = hash_values[row];
                if let Some((start, end)) = self.candidates(hash) {
                    let mut i = resume.unwrap_or(start);
                    while i < end {
                        if self.hashes[i] == hash {
                            if remaining == 0 {
                                return Some(MidChain {
                                    row,
                                    next: i as u64 + 1,
                                });
                            }
                            input_indices.push(row as u32);
                            match_indices.push(self.rows[i].to_u64());
                            remaining -= 1;
                        }
                        i += 1;
                    }
                }
            }
            resume = None;
            row += 1;
            if remaining == 0 {
                return (row < num_probe_rows).then_some(AtRow { row });
            }
        }
        None
    }

    fn contain_hashes(&self, hash_values: &[u64]) -> BooleanArray {
        self.assert_finalized();
        let buffer = BooleanBuffer::collect_bool(hash_values.len(), |i| {
            let hash = hash_values[i];
            self.candidates(hash)
                .is_some_and(|(start, end)| self.hashes[start..end].contains(&hash))
        });
        BooleanArray::new(buffer, None)
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn len(&self) -> usize {
        self.assert_finalized();
        self.distinct
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::joins::join_hash_map::JoinHashMapU32;

    fn build<T: RowIndex>(hashes: &[u64]) -> UnchainedJoinHashMap<T> {
        let mut map = UnchainedJoinHashMap::<T>::with_capacity(hashes.len());
        map.update_from_iter(Box::new(hashes.iter().enumerate().rev()), 0);
        map.finalize();
        map
    }

    /// Collects every match with the given chunk `limit`, resuming from each
    /// returned offset.
    fn probe_all(
        map: &dyn JoinHashMapType,
        probe: &[u64],
        valid_keys: Option<&NullBuffer>,
        limit: usize,
    ) -> Vec<(u32, u64)> {
        let mut input_indices = vec![];
        let mut match_indices = vec![];
        let mut out = vec![];
        let mut offset = AtRow { row: 0 };
        loop {
            let next = map.get_matched_indices_with_limit_offset(
                probe,
                valid_keys,
                limit,
                offset,
                &mut input_indices,
                &mut match_indices,
            );
            assert!(input_indices.len() <= limit);
            out.extend(
                input_indices
                    .iter()
                    .copied()
                    .zip(match_indices.iter().copied()),
            );
            match next {
                Some(next) => {
                    assert_ne!(next, AtRow { row: 0 });
                    offset = next;
                }
                None => return out,
            }
        }
    }

    fn pseudo_random_hashes(n: usize, distinct: u64, seed: u64) -> Vec<u64> {
        let mut state = seed;
        (0..n)
            .map(|_| {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                // Spread the key over all 64 bits like a real hash function.
                (state % distinct).wrapping_mul(0x9E37_79B9_7F4A_7C15)
            })
            .collect()
    }

    #[test]
    fn empty_map() {
        let map = build::<u32>(&[]);
        assert!(map.is_empty());
        assert_eq!(map.len(), 0);
        assert_eq!(probe_all(&map, &[1, 2, 3], None, 2), vec![]);
        assert!(!map.contain_hashes(&[0, 1]).value(0));
    }

    #[test]
    fn duplicates_are_returned_in_build_order() {
        let map = build::<u32>(&[10, 20, 10, 30, 10]);
        assert_eq!(map.len(), 3);
        assert_eq!(
            probe_all(&map, &[10, 40, 30], None, 100),
            vec![(0, 0), (0, 2), (0, 4), (2, 3)]
        );
        let contains = map.contain_hashes(&[10, 11, 20, 30, 40]);
        assert_eq!(
            contains.iter().flatten().collect::<Vec<_>>(),
            vec![true, false, true, true, false]
        );
    }

    #[test]
    fn all_hashes_collide() {
        // `force_hash_collisions` maps every key to hash 0.
        let map = build::<u64>(&[0; 5]);
        assert_eq!(map.len(), 1);
        assert_eq!(
            probe_all(&map, &[0, 0], None, 3),
            vec![
                (0, 0),
                (0, 1),
                (0, 2),
                (0, 3),
                (0, 4),
                (1, 0),
                (1, 1),
                (1, 2),
                (1, 3),
                (1, 4)
            ]
        );
    }

    #[test]
    fn skips_invalid_keys() {
        let map = build::<u32>(&[10, 20, 10, 20]);
        let valid_keys = NullBuffer::from(vec![false, true]);
        assert_eq!(
            probe_all(&map, &[10, 20], Some(&valid_keys), 1),
            vec![(1, 1), (1, 3)]
        );
    }

    #[test]
    fn finalize_releases_staging_once() {
        let mut map = UnchainedJoinHashMapU32::with_capacity(4);
        map.update_from_iter(Box::new([1u64, 2].iter().enumerate()), 0);
        assert_eq!(map.finalize(), 4 * 12);
        assert_eq!(map.finalize(), 0);
    }

    #[test]
    #[should_panic(expected = "must be finalized")]
    fn lookup_before_finalize_panics() {
        let mut map = UnchainedJoinHashMapU32::with_capacity(1);
        map.update_from_iter(Box::new(std::iter::once((0, &1u64))), 0);
        map.contain_hashes(&[1]);
    }

    /// The unchained map must return exactly what the chained map returns,
    /// in the same order, for every chunk size.
    #[test]
    fn matches_chained_map() {
        for (n, distinct) in [(1, 1), (7, 3), (100, 100), (1000, 37), (5000, 4000)] {
            let build_hashes = pseudo_random_hashes(n, distinct, n as u64);
            let mut probe = pseudo_random_hashes(300, distinct * 2, 7);
            probe.extend(&build_hashes[..n.min(50)]);
            let valid: Vec<bool> = (0..probe.len()).map(|i| i % 11 != 3).collect();
            let valid_keys = NullBuffer::from(valid);

            let unchained = build::<u32>(&build_hashes);
            let mut chained = JoinHashMapU32::with_capacity(n);
            chained.update_from_iter(Box::new(build_hashes.iter().enumerate().rev()), 0);

            assert_eq!(unchained.len(), chained.len());
            assert_eq!(
                unchained.contain_hashes(&probe),
                chained.contain_hashes(&probe)
            );
            let (a, b) =
                unchained.get_matched_indices(Box::new(probe.iter().enumerate()), None);
            let (c, d) =
                chained.get_matched_indices(Box::new(probe.iter().enumerate()), None);
            assert_eq!((a, b), (c, d));

            for limit in [1, 2, 3, 64, 100_000] {
                for valid_keys in [None, Some(&valid_keys)] {
                    assert_eq!(
                        probe_all(&unchained, &probe, valid_keys, limit),
                        probe_all(&chained, &probe, valid_keys, 100_000),
                        "n={n} distinct={distinct} limit={limit}"
                    );
                }
            }
        }
    }
}
