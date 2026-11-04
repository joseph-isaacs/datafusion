// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: See the NOTICE file distributed with this work for additional information regarding copyright ownership
// SPDX-FileContributor: Licensed to the Apache Software Foundation (ASF) under one or more contributor license agreements

//! [`GroupTable`]: the hash table mapping the values of a single primitive
//! group by column to their group indices, see `GroupValuesPrimitive`.

use std::mem::size_of;

use crate::joins::join_hash_table::zeroed_vec;

/// Number of slots per bucket.
const BUCKET_SLOTS: usize = 8;

/// Maximum fraction of occupied slots, as `(numerator, denominator)`.
const MAX_LOAD: (usize, usize) = (3, 4);

/// Number of rows ahead whose bucket is prefetched.
pub(crate) const PREFETCH_DISTANCE: usize = 16;

/// Tags and group indices of eight slots, one 64-byte cache line.
#[derive(Clone, Copy, Default)]
#[repr(C, align(64))]
struct Bucket {
    /// Upper 32 bits of the value's hash, with the lowest bit set, `0` for
    /// an empty slot.
    tags: [u32; BUCKET_SLOTS],
    /// Group index of the slot's value.
    groups: [u32; BUCKET_SLOTS],
}

impl Bucket {
    #[inline(always)]
    fn matches(&self, tag: u32) -> u32 {
        let mut mask = 0;
        for (i, &t) in self.tags.iter().enumerate() {
            mask |= ((t == tag) as u32) << i;
        }
        mask
    }
}

/// Maps distinct values of type `K` to group indices.
///
/// Like the join hash table (`JoinHashTableU32`), it is an array of
/// cache-line buckets of eight slots compared without branches, kept at most
/// [`MAX_LOAD`] full, with the values of the slots of a bucket in a separate line
/// (`keys`). Lookups of a batch prefetch the bucket and the keys of the row
/// [`PREFETCH_DISTANCE`] rows ahead, so the cache misses of a high
/// cardinality group by overlap rather than add up.
pub(crate) struct GroupTable<K> {
    buckets: Vec<Bucket>,
    /// `keys[b][s]` is the value of slot `s` of bucket `b`.
    keys: Vec<[K; BUCKET_SLOTS]>,
    /// `buckets.len() - 1`
    mask: usize,
    /// Number of occupied slots.
    len: usize,
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

impl<K: Copy + Default> GroupTable<K> {
    pub(crate) fn with_capacity(capacity: usize) -> Self {
        let num_buckets = Self::num_buckets(capacity);
        Self {
            // SAFETY: a `Bucket` of zeros is a bucket of empty slots.
            buckets: unsafe { zeroed_vec(num_buckets) },
            keys: vec![[K::default(); BUCKET_SLOTS]; num_buckets],
            mask: num_buckets - 1,
            len: 0,
        }
    }

    /// Buckets for `capacity` values, at most [`MAX_LOAD`] full.
    fn num_buckets(capacity: usize) -> usize {
        capacity
            .saturating_mul(MAX_LOAD.1)
            .div_ceil(MAX_LOAD.0 * BUCKET_SLOTS)
            .next_power_of_two()
            .max(1)
    }

    /// Number of values the table holds before growing.
    pub(crate) fn capacity(&self) -> usize {
        self.buckets.len() * BUCKET_SLOTS * MAX_LOAD.0 / MAX_LOAD.1
    }

    /// Whether inserting one more value requires [`Self::reserve`] first.
    #[inline(always)]
    pub(crate) fn is_full(&self) -> bool {
        self.len >= self.capacity()
    }

    pub(crate) fn len(&self) -> usize {
        self.len
    }

    /// Bytes allocated by the table.
    pub(crate) fn allocation_size(&self) -> usize {
        self.buckets.capacity() * size_of::<Bucket>()
            + self.keys.capacity() * size_of::<[K; BUCKET_SLOTS]>()
    }

    #[inline(always)]
    fn tag(hash: u64) -> u32 {
        ((hash >> 32) as u32) | 1
    }

    #[inline(always)]
    fn home(&self, hash: u64) -> usize {
        hash as usize & self.mask
    }

    #[inline(always)]
    pub(crate) fn prefetch(&self, hash: u64) {
        let home = self.home(hash);
        prefetch(&self.buckets, home);
        prefetch(&self.keys, home);
    }

    /// Returns the group of `key`, inserting it as group `new_group()` if
    /// absent. The table must have room for one more value.
    #[inline(always)]
    pub(crate) fn get_or_insert(
        &mut self,
        hash: u64,
        key: K,
        is_eq: impl Fn(K, K) -> bool,
        new_group: impl FnOnce() -> u32,
    ) -> u32 {
        let tag = Self::tag(hash);
        let mut pos = self.home(hash);
        loop {
            // SAFETY: `pos` is always masked by `mask`, `buckets.len() - 1`,
            // and `keys` has as many entries as `buckets`.
            let (bucket, keys) = unsafe {
                (
                    self.buckets.get_unchecked_mut(pos),
                    self.keys.get_unchecked_mut(pos),
                )
            };
            let mut matches = bucket.matches(tag);
            while matches != 0 {
                let slot = matches.trailing_zeros() as usize;
                if is_eq(keys[slot], key) {
                    return bucket.groups[slot];
                }
                matches &= matches - 1;
            }
            let empty = bucket.matches(0);
            if empty != 0 {
                let slot = empty.trailing_zeros() as usize;
                let group = new_group();
                bucket.tags[slot] = tag;
                bucket.groups[slot] = group;
                keys[slot] = key;
                self.len += 1;
                return group;
            }
            pos = (pos + 1) & self.mask;
        }
    }

    /// Inserts `key`, known to be absent, as group `group`. The table must
    /// have room for one more value.
    fn insert_unique(&mut self, hash: u64, key: K, group: u32) {
        let tag = Self::tag(hash);
        let mut pos = self.home(hash);
        loop {
            let bucket = &mut self.buckets[pos];
            let empty = bucket.matches(0);
            if empty != 0 {
                let slot = empty.trailing_zeros() as usize;
                bucket.tags[slot] = tag;
                bucket.groups[slot] = group;
                self.keys[pos][slot] = key;
                self.len += 1;
                return;
            }
            pos = (pos + 1) & self.mask;
        }
    }

    /// Calls `f` with the value and group of every slot.
    fn for_each(&self, mut f: impl FnMut(K, u32)) {
        for (bucket, keys) in self.buckets.iter().zip(&self.keys) {
            for ((&tag, &group), &key) in bucket.tags.iter().zip(&bucket.groups).zip(keys)
            {
                if tag != 0 {
                    f(key, group);
                }
            }
        }
    }

    /// Rebuilds the table with room for `capacity` values, keeping the
    /// values for which `f` returns a new group.
    fn rebuild(
        &mut self,
        capacity: usize,
        hash: &impl Fn(K) -> u64,
        mut f: impl FnMut(u32) -> Option<u32>,
    ) {
        let mut table = Self::with_capacity(capacity);
        self.for_each(|key, group| {
            if let Some(group) = f(group) {
                table.insert_unique(hash(key), key, group);
            }
        });
        *self = table;
    }

    /// Makes room for `additional` more values.
    pub(crate) fn reserve(&mut self, additional: usize, hash: &impl Fn(K) -> u64) {
        let needed = self.len + additional;
        if needed > self.capacity() {
            let capacity = needed.max(self.capacity() * 2);
            self.rebuild(capacity, hash, Some);
        }
    }

    /// Removes the groups `0..n` and renumbers the others from `0`, keeping
    /// the allocation.
    pub(crate) fn remove_first(&mut self, n: usize, hash: &impl Fn(K) -> u64) {
        let n = u32::try_from(n).unwrap_or(u32::MAX);
        self.rebuild(self.capacity(), hash, |group| group.checked_sub(n));
    }

    pub(crate) fn clear(&mut self) {
        self.buckets.fill(Bucket::default());
        self.len = 0;
    }

    /// Shrinks the table to `capacity` values, which must hold its values.
    pub(crate) fn shrink_to(&mut self, capacity: usize, hash: &impl Fn(K) -> u64) {
        let capacity = capacity.max(self.len);
        if Self::num_buckets(capacity) < self.buckets.len() {
            self.rebuild(capacity, hash, Some);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    /// A poor hash, so that many values share a home bucket and a tag.
    fn hash(value: i64) -> u64 {
        let h = (value as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
        if value % 3 == 0 {
            h & 0xFFFF_0000_0000_0007
        } else {
            h
        }
    }

    /// Interns `values` like `GroupValuesPrimitive` does, checking every group
    /// against `model`.
    fn intern(
        table: &mut GroupTable<i64>,
        model: &mut HashMap<i64, u32>,
        values: &[i64],
    ) {
        for &value in values {
            if table.is_full() {
                table.reserve(1, &hash);
            }
            let next = model.len() as u32;
            let group = table.get_or_insert(hash(value), value, |a, b| a == b, || next);
            let expected = *model.entry(value).or_insert(next);
            assert_eq!(group, expected, "value {value}");
        }
        assert_eq!(table.len(), model.len());
    }

    #[test]
    fn matches_hash_map() {
        let mut table = GroupTable::<i64>::with_capacity(4);
        let mut model = HashMap::new();
        let values: Vec<i64> = (0..20_000).map(|i| (i * 7919) % 5003 - 2500).collect();
        intern(&mut table, &mut model, &values);
        assert!(table.capacity() >= model.len());

        // Remove the first groups and renumber the others.
        let n = 1000;
        let capacity = table.capacity();
        table.remove_first(n, &hash);
        assert_eq!(table.capacity(), capacity);
        model.retain(|_, group| *group >= n as u32);
        for group in model.values_mut() {
            *group -= n as u32;
        }
        let remaining: Vec<i64> = model.keys().copied().collect();
        intern(&mut table, &mut model, &remaining);

        // Shrinking keeps the values.
        table.shrink_to(0, &hash);
        assert!(table.capacity() <= capacity);
        intern(&mut table, &mut model, &values);

        table.clear();
        assert_eq!(table.len(), 0);
        table.shrink_to(0, &hash);
        assert!(table.capacity() < capacity);
        let mut model = HashMap::new();
        intern(&mut table, &mut model, &values[..100]);
    }
}
