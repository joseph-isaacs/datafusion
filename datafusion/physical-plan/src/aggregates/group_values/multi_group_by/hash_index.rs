// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: See the NOTICE file distributed with this work for additional information regarding copyright ownership
// SPDX-FileContributor: Licensed to the Apache Software Foundation (ASF) under one or more contributor license agreements

//! [`HashIndex`]: the hash table of `GroupValuesColumn`, mapping the hash of
//! a group's values to the group (or groups) with that hash.

use std::mem::size_of;

/// Number of slots per bucket.
const BUCKET_SLOTS: usize = 8;

/// Maximum fraction of occupied slots, as `(numerator, denominator)`.
const MAX_LOAD: (usize, usize) = (3, 4);

/// Number of rows ahead whose bucket is prefetched by callers.
pub(crate) const PREFETCH_DISTANCE: usize = 16;

/// Tags of eight slots, half a cache line, with the slots' hashes and values
/// stored apart (see [`HashIndex`]).
#[derive(Clone, Copy, Default)]
#[repr(C, align(32))]
struct Tags([u32; BUCKET_SLOTS]);

impl Tags {
    #[inline(always)]
    fn matches(&self, tag: u32) -> u32 {
        let mut mask = 0;
        for (i, &t) in self.0.iter().enumerate() {
            mask |= ((t == tag) as u32) << i;
        }
        mask
    }
}

#[inline(always)]
fn prefetch<T>(slice: &[T], index: usize) {
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

/// A multimap from 64-bit hashes to values of type `V`.
///
/// Same semantics as `hashbrown::HashTable<(u64, V)>` searched by hash
/// equality: entries are found by their full hash, and several entries may
/// share a hash. The layout is that of the other hash tables of the hash
/// join and aggregation (`JoinHashTableU32`, `GroupTable`): buckets of eight
/// slots whose tags (upper hash bits) are compared without branches, with
/// the full hashes and the values in separate arrays, kept at most
/// [`MAX_LOAD`] full. Callers [`Self::prefetch`] the bucket of the row
/// [`PREFETCH_DISTANCE`] rows ahead, so that the cache misses of a high
/// cardinality group by overlap.
pub(crate) struct HashIndex<V> {
    tags: Vec<Tags>,
    hashes: Vec<[u64; BUCKET_SLOTS]>,
    values: Vec<[V; BUCKET_SLOTS]>,
    /// `tags.len() - 1`, or `0` for a table without buckets
    mask: usize,
    /// Number of occupied slots.
    len: usize,
}

impl<V: Copy + Default> Default for HashIndex<V> {
    fn default() -> Self {
        Self::with_capacity(0)
    }
}

impl<V: Copy + Default> HashIndex<V> {
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        let num_buckets = Self::num_buckets(capacity);
        Self {
            tags: vec![Tags::default(); num_buckets],
            hashes: vec![[0; BUCKET_SLOTS]; num_buckets],
            values: vec![[V::default(); BUCKET_SLOTS]; num_buckets],
            mask: num_buckets.saturating_sub(1),
            len: 0,
        }
    }

    /// Buckets for `capacity` entries, none for `0`, so that an empty table
    /// allocates nothing.
    fn num_buckets(capacity: usize) -> usize {
        if capacity == 0 {
            return 0;
        }
        capacity
            .saturating_mul(MAX_LOAD.1)
            .div_ceil(MAX_LOAD.0 * BUCKET_SLOTS)
            .next_power_of_two()
            .max(1)
    }

    /// Number of entries the table holds before growing.
    pub(crate) fn capacity(&self) -> usize {
        self.tags.len() * BUCKET_SLOTS * MAX_LOAD.0 / MAX_LOAD.1
    }

    /// Number of entries.
    #[cfg(test)]
    pub(crate) fn len(&self) -> usize {
        self.len
    }

    #[cfg(test)]
    pub(crate) fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Bytes allocated by the table.
    pub(crate) fn allocation_size(&self) -> usize {
        self.tags.capacity() * size_of::<Tags>()
            + self.hashes.capacity() * size_of::<[u64; BUCKET_SLOTS]>()
            + self.values.capacity() * size_of::<[V; BUCKET_SLOTS]>()
    }

    #[inline(always)]
    fn tag(hash: u64) -> u32 {
        ((hash >> 32) as u32) | 1
    }

    #[inline(always)]
    fn home(&self, hash: u64) -> usize {
        hash as usize & self.mask
    }

    /// Hints the CPU to load the home bucket of `hash`.
    #[inline(always)]
    pub(crate) fn prefetch(&self, hash: u64) {
        let home = self.home(hash);
        prefetch(&self.tags, home);
        prefetch(&self.hashes, home);
    }

    /// Returns the first entry with hash `hash` for which `eq` holds.
    #[inline(always)]
    pub(crate) fn find_mut(
        &mut self,
        hash: u64,
        mut eq: impl FnMut(&V) -> bool,
    ) -> Option<&mut V> {
        if self.tags.is_empty() {
            return None;
        }
        let tag = Self::tag(hash);
        let mut pos = self.home(hash);
        loop {
            let tags = &self.tags[pos];
            let mut matches = tags.matches(tag);
            while matches != 0 {
                let slot = matches.trailing_zeros() as usize;
                if self.hashes[pos][slot] == hash && eq(&self.values[pos][slot]) {
                    return Some(&mut self.values[pos][slot]);
                }
                matches &= matches - 1;
            }
            if tags.0[BUCKET_SLOTS - 1] == 0 {
                return None;
            }
            pos = (pos + 1) & self.mask;
        }
    }

    /// Returns the first entry with hash `hash`.
    #[inline(always)]
    pub(crate) fn find(&self, hash: u64) -> Option<&V> {
        if self.tags.is_empty() {
            return None;
        }
        let tag = Self::tag(hash);
        let mut pos = self.home(hash);
        loop {
            let tags = &self.tags[pos];
            let mut matches = tags.matches(tag);
            while matches != 0 {
                let slot = matches.trailing_zeros() as usize;
                if self.hashes[pos][slot] == hash {
                    return Some(&self.values[pos][slot]);
                }
                matches &= matches - 1;
            }
            if tags.0[BUCKET_SLOTS - 1] == 0 {
                return None;
            }
            pos = (pos + 1) & self.mask;
        }
    }

    /// Inserts an entry, growing the table if needed. Returns the change in
    /// [`Self::allocation_size`].
    pub(crate) fn insert(&mut self, hash: u64, value: V) -> isize {
        let mut grown = 0;
        if self.len >= self.capacity() {
            let before = self.allocation_size() as isize;
            self.rebuild(self.capacity().max(BUCKET_SLOTS) * 2, |_| true);
            grown = self.allocation_size() as isize - before;
        }
        self.insert_unchecked(hash, value);
        grown
    }

    fn insert_unchecked(&mut self, hash: u64, value: V) {
        let tag = Self::tag(hash);
        let mut pos = self.home(hash);
        loop {
            let empty = self.tags[pos].matches(0);
            if empty != 0 {
                let slot = empty.trailing_zeros() as usize;
                self.tags[pos].0[slot] = tag;
                self.hashes[pos][slot] = hash;
                self.values[pos][slot] = value;
                self.len += 1;
                return;
            }
            pos = (pos + 1) & self.mask;
        }
    }

    /// Rebuilds the table with room for `capacity` entries, keeping the
    /// entries for which `keep` returns `true`.
    fn rebuild(&mut self, capacity: usize, mut keep: impl FnMut(&mut V) -> bool) {
        let mut table = Self::with_capacity(capacity);
        for ((tags, hashes), values) in self
            .tags
            .iter()
            .zip(&self.hashes)
            .zip(self.values.iter_mut())
        {
            for ((&tag, &hash), value) in tags.0.iter().zip(hashes).zip(values) {
                if tag != 0 && keep(value) {
                    table.insert_unchecked(hash, *value);
                }
            }
        }
        *self = table;
    }

    /// Keeps the entries for which `keep` returns `true`, keeping the
    /// allocation.
    pub(crate) fn retain(&mut self, keep: impl FnMut(&mut V) -> bool) {
        self.rebuild(self.capacity(), keep);
    }

    pub(crate) fn clear(&mut self) {
        self.tags.fill(Tags::default());
        self.len = 0;
    }

    /// Shrinks the table to `capacity` entries, but at least its entries.
    pub(crate) fn shrink_to(&mut self, capacity: usize) {
        let capacity = capacity.max(self.len);
        if Self::num_buckets(capacity) < self.tags.len() {
            self.rebuild(capacity, |_| true);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn matches_multimap() {
        let mut index = HashIndex::<u64>::with_capacity(0);
        let mut model: HashMap<u64, Vec<u64>> = HashMap::new();
        // Poor hashes: many share their home bucket, and some their tag.
        let hash = |i: u64| ((i % 997) << 40) | (i % 13);
        for i in 0..10_000u64 {
            let h = hash(i);
            index.insert(h, i);
            model.entry(h).or_default().push(i);
        }
        assert!(index.capacity() >= 10_000);
        for (&h, values) in &model {
            assert!(values.contains(index.find(h).unwrap()));
            for v in values {
                assert_eq!(index.find_mut(h, |x| x == v).copied(), Some(*v));
            }
        }
        assert!(index.find(12345 << 40).is_none());

        let capacity = index.capacity();
        index.retain(|v| *v % 2 == 0);
        assert_eq!(index.capacity(), capacity);
        for (&h, values) in &model {
            for v in values {
                let found = index.find_mut(h, |x| x == v).copied();
                assert_eq!(found, (v % 2 == 0).then_some(*v));
            }
        }

        index.clear();
        assert!(index.find(hash(0)).is_none());
        index.shrink_to(0);
        assert!(index.capacity() < capacity);
    }
}
