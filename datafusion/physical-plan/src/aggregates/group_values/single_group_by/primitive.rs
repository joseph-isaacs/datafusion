// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: See the NOTICE file distributed with this work for additional information regarding copyright ownership
// SPDX-FileContributor: Licensed to the Apache Software Foundation (ASF) under one or more contributor license agreements

use super::group_table::{GroupTable, PREFETCH_DISTANCE};

/// Table size above which lookups prefetch, see [`GroupTable::prefetch`].
const PREFETCH_MIN_TABLE_BYTES: usize = 1 << 20;
use crate::aggregates::group_values::GroupValues;
use arrow::array::types::{IntervalDayTime, IntervalMonthDayNano};
use arrow::array::{
    ArrayRef, ArrowNativeTypeOp, ArrowPrimitiveType, NullBufferBuilder, PrimitiveArray,
    cast::AsArray,
};
use arrow::datatypes::{DataType, i256};
use datafusion_common::float_semantics::normalize_float;
use datafusion_common::hash_utils::RandomState;
use datafusion_common::utils::split_vec_min_alloc;
use datafusion_common::{Result, internal_err};
use datafusion_execution::memory_pool::proxy::VecAllocExt;
use datafusion_expr::{EmitTo, GroupSelection};
use half::f16;
#[cfg(not(feature = "force_hash_collisions"))]
use std::hash::BuildHasher;
use std::sync::Arc;

/// A trait to allow hashing of floating point numbers
pub trait HashValue {
    fn hash(&self, state: &RandomState) -> u64;

    /// Return a canonical representative whose bit pattern is identical for
    /// all values that should be grouped together. Default is the identity;
    /// floats override this with [`normalize_float`] so the bit-equal
    /// `is_eq` check used during insertion follows SQL equality.
    #[inline]
    fn normalize_key(self) -> Self
    where
        Self: Sized,
    {
        self
    }
}

macro_rules! hash_integer {
    ($($t:ty),+) => {
        $(impl HashValue for $t {
            #[cfg(not(feature = "force_hash_collisions"))]
            fn hash(&self, state: &RandomState) -> u64 {
                state.hash_one(self)
            }

            #[cfg(feature = "force_hash_collisions")]
            fn hash(&self, _state: &RandomState) -> u64 {
                0
            }
        })+
    };
}
hash_integer!(i8, i16, i32, i64, i128, i256);
hash_integer!(u8, u16, u32, u64);
hash_integer!(IntervalDayTime, IntervalMonthDayNano);

macro_rules! hash_float {
    ($($t:ty),+) => {
        $(impl HashValue for $t {
            #[cfg(not(feature = "force_hash_collisions"))]
            fn hash(&self, state: &RandomState) -> u64 {
                state.hash_one(datafusion_common::float_semantics::hash_input(*self).to_bits())
            }

            #[cfg(feature = "force_hash_collisions")]
            fn hash(&self, _state: &RandomState) -> u64 {
                0
            }

            #[inline]
            fn normalize_key(self) -> Self {
                normalize_float(self)
            }
        })+
    };
}

hash_float!(f16, f32, f64);

/// A [`GroupValues`] storing a single column of primitive values
///
/// This specialization is significantly faster than using the more general
/// purpose `Row`s format
pub struct GroupValuesPrimitive<T: ArrowPrimitiveType> {
    /// The data type of the output array
    data_type: DataType,
    /// Maps each distinct non-NULL value to its group index.
    ///
    /// The table stores the values next to the group indices, so a lookup
    /// does not read `values[group_index]`, and rehashing recomputes the
    /// hashes from those values without reading `values` either, see
    /// <https://github.com/apache/datafusion/issues/15961>.
    map: GroupTable<T::Native>,
    /// Scratch buffer for the hashes of the values of a batch.
    hashes: Vec<u64>,
    /// The group index of the null value if any
    null_group: Option<usize>,
    /// The values for each group index
    values: Vec<T::Native>,
    /// The random state used to generate hashes
    random_state: RandomState,
}

impl<T: ArrowPrimitiveType> GroupValuesPrimitive<T> {
    pub fn new(data_type: DataType) -> Self {
        assert!(PrimitiveArray::<T>::is_compatible(&data_type));
        Self {
            data_type,
            map: GroupTable::with_capacity(128),
            hashes: Vec::new(),
            values: Vec::with_capacity(128),
            null_group: None,
            random_state: crate::aggregates::AGGREGATION_HASH_SEED,
        }
    }
}

impl<T: ArrowPrimitiveType> GroupValues for GroupValuesPrimitive<T>
where
    T::Native: HashValue,
{
    fn intern(&mut self, cols: &[ArrayRef], groups: &mut Vec<usize>) -> Result<()> {
        assert_eq!(cols.len(), 1);
        groups.clear();

        let array = cols[0].as_primitive::<T>();
        let n = array.len();
        if self.values.len() + n > u32::MAX as usize {
            return internal_err!("Too many groups for a single group by column");
        }
        let state = &self.random_state;
        let hash = |value: T::Native| value.hash(state);

        // Fold equivalence-class duplicates (e.g. `-0.0` → `+0.0`) so the
        // bit-equal `is_eq` matches and the stored value is the canonical
        // representative. NULL slots hash an arbitrary value, never used.
        self.hashes.clear();
        self.hashes.extend(
            array
                .values()
                .iter()
                .map(|value| hash(value.normalize_key())),
        );
        let hashes = &self.hashes;

        // Prefetching only pays off once the table outgrows the caches.
        let prefetch = self.map.allocation_size() > PREFETCH_MIN_TABLE_BYTES;
        if prefetch {
            for &h in &hashes[..PREFETCH_DISTANCE.min(n)] {
                self.map.prefetch(h);
            }
        }
        // The previous non-NULL value and its group: runs of equal values,
        // common in sorted or clustered data, skip the table lookup.
        let mut previous: Option<(T::Native, usize)> = None;
        for (i, v) in array.iter().enumerate() {
            if prefetch && i + PREFETCH_DISTANCE < n {
                self.map.prefetch(hashes[i + PREFETCH_DISTANCE]);
            }
            let group_id = match v {
                None => *self.null_group.get_or_insert_with(|| {
                    let group_id = self.values.len();
                    self.values.push(Default::default());
                    group_id
                }),
                Some(key) => {
                    let key = key.normalize_key();
                    match previous {
                        Some((previous_key, group_id)) if previous_key.is_eq(key) => {
                            group_id
                        }
                        _ => {
                            if self.map.is_full() {
                                self.map.reserve(1, &hash);
                            }
                            let values = &mut self.values;
                            let group_id = self.map.get_or_insert(
                                hashes[i],
                                key,
                                |a, b| a.is_eq(b),
                                || {
                                    let group_id = values.len() as u32;
                                    values.push(key);
                                    group_id
                                },
                            ) as usize;
                            previous = Some((key, group_id));
                            group_id
                        }
                    }
                }
            };
            groups.push(group_id)
        }
        Ok(())
    }

    fn size(&self) -> usize {
        size_of::<Self>()
            + self.map.allocation_size()
            + self.values.allocated_size()
            + self.hashes.allocated_size()
    }

    fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    fn len(&self) -> usize {
        self.values.len()
    }

    fn emit(&mut self, emit_to: EmitTo) -> Result<Vec<ArrayRef>> {
        fn build_primitive<T: ArrowPrimitiveType>(
            values: Vec<T::Native>,
            null_idx: Option<usize>,
        ) -> PrimitiveArray<T> {
            let nulls = null_idx.map(|null_idx| {
                let mut buffer = NullBufferBuilder::new(values.len());
                buffer.append_n_non_nulls(null_idx);
                buffer.append_null();
                buffer.append_n_non_nulls(values.len() - null_idx - 1);
                // NOTE: The inner builder must be constructed as there is at least one null
                buffer.finish().unwrap()
            });
            PrimitiveArray::<T>::new(values.into(), nulls)
        }

        let array: PrimitiveArray<T> = match emit_to {
            EmitTo::All => {
                self.map.clear();
                build_primitive(std::mem::take(&mut self.values), self.null_group.take())
            }
            EmitTo::First(n) => {
                let state = &self.random_state;
                self.map
                    .remove_first(n, &|value: T::Native| value.hash(state));
                let null_group = match &mut self.null_group {
                    Some(v) if *v >= n => {
                        *v -= n;
                        None
                    }
                    Some(_) => self.null_group.take(),
                    None => None,
                };
                build_primitive(split_vec_min_alloc(&mut self.values, n), null_group)
            }
        };

        Ok(vec![Arc::new(array.with_data_type(self.data_type.clone()))])
    }

    fn values_preserving(
        &mut self,
        selection: GroupSelection<'_>,
    ) -> Result<Vec<ArrayRef>> {
        selection.validate_num_groups(self.values.len())?;
        let values: Vec<T::Native> =
            selection.iter().map(|index| self.values[index]).collect();
        let nulls = if let Some(null_group) = self.null_group {
            let mut nulls = NullBufferBuilder::new(values.len());
            for index in selection.iter() {
                if index == null_group {
                    nulls.append_null();
                } else {
                    nulls.append_non_null();
                }
            }
            nulls.finish()
        } else {
            None
        };
        let array = PrimitiveArray::<T>::new(values.into(), nulls)
            .with_data_type(self.data_type.clone());
        Ok(vec![Arc::new(array)])
    }

    fn supports_values_preserving(&self) -> bool {
        true
    }

    fn shrink_to_fit(&mut self) {
        let state = &self.random_state;
        self.map
            .shrink_to(self.map.len(), &|value: T::Native| value.hash(state));
        self.hashes = Vec::new();
        self.values.shrink_to_fit();
    }

    fn clear_shrink(&mut self, num_rows: usize) {
        self.values.clear();
        self.values.shrink_to(num_rows);
        self.map.clear();
        let state = &self.random_state;
        self.map
            .shrink_to(num_rows, &|value: T::Native| value.hash(state));
        self.hashes.clear();
        self.hashes.shrink_to(num_rows);

        // Reset the null group index
        self.null_group = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrow::array::types::Int32Type;
    use arrow::array::{ArrayRef, Int32Array};
    use arrow::datatypes::DataType;
    use datafusion_expr::EmitTo;
    use std::mem::size_of;
    use std::sync::Arc;

    /// Mirror of the `EmitTo::take_needed` regression test, applied to the
    /// concrete `GroupValuesPrimitive` accumulator.
    ///
    /// When `n` is small, the old `split_off(n) + swap` pattern used inside
    /// `emit(EmitTo::First(n))` left `self.values` with a small fresh allocation
    /// and returned the emitted prefix carrying the original large backing.
    ///
    /// With `split_vec_min_alloc` and `n * 2 <= len`, the drain branch is taken:
    /// the emitted prefix gets a compact allocation and `self.values` retains the
    /// original large one.
    #[test]
    fn emit_first_small_n_allocates_minimally() -> Result<()> {
        let mut gv = GroupValuesPrimitive::<Int32Type>::new(DataType::Int32);

        // Intern 20 distinct values; `new()` pre-allocates capacity 128 for `values`.
        let arr: ArrayRef = Arc::new(Int32Array::from_iter_values(0..20i32));
        let mut groups = vec![];
        gv.intern(&[arr], &mut groups)?;
        let capacity_before = gv.values.capacity(); // 128

        // n=4, n*2=8 <= len=20 -> drain branch
        let emitted = gv.emit(EmitTo::First(4))?;

        assert_eq!(emitted[0].len(), 4);

        // `self.values` must retain its original large allocation.
        // Old split_off+swap left it with a fresh small allocation (~16).
        assert_eq!(
            gv.values.capacity(),
            capacity_before,
            "self.values capacity {} should equal original {} after small First(n) emit",
            gv.values.capacity(),
            capacity_before,
        );

        Ok(())
    }

    #[test]
    fn test_exact_hash_table_allocation_accounting() {
        let mut group_values = GroupValuesPrimitive::<Int32Type>::new(DataType::Int32);
        let self_size = size_of::<GroupValuesPrimitive<Int32Type>>();

        // 1. Initial state: the constructor calls GroupTable::with_capacity(128),
        //    so the map already has a nonzero retained allocation. Capture it
        //    and verify that size() accounts for it correctly.
        let initial_map_bytes = group_values.map.allocation_size();
        assert!(group_values.map.capacity() >= 128);
        assert!(initial_map_bytes > 0);
        assert_eq!(
            group_values.size(),
            self_size
                + group_values.values.allocated_size()
                + group_values.hashes.allocated_size()
                + initial_map_bytes,
        );

        // 2. Insert keys and observe table allocation growth
        let num_distinct = 500_i32;
        let arr: ArrayRef = Arc::new(Int32Array::from_iter_values(0..num_distinct));
        let mut groups = vec![];
        group_values.intern(&[arr], &mut groups).unwrap();
        assert_eq!(groups.len(), num_distinct as usize);

        let table_bytes = group_values.map.allocation_size();
        assert!(group_values.map.capacity() >= num_distinct as usize);
        assert!(table_bytes > 0);

        // The table holds at least a value and a group index per value of
        // capacity, and keeps spare slots beyond that capacity.
        let entry_only_bytes =
            group_values.map.capacity() * (size_of::<i32>() + size_of::<u32>());
        assert!(table_bytes > entry_only_bytes);

        assert_eq!(
            group_values.size(),
            self_size
                + group_values.values.allocated_size()
                + group_values.hashes.allocated_size()
                + table_bytes,
        );

        // 3. Partial emit (EmitTo::First)
        let emit_count = 200;
        let emitted = group_values.emit(EmitTo::First(emit_count)).unwrap();
        assert_eq!(emitted[0].len(), emit_count);
        assert_eq!(group_values.len(), (num_distinct as usize) - emit_count);

        // Retained capacity is still charged until the table releases it
        assert_eq!(group_values.map.allocation_size(), table_bytes);
        assert_eq!(
            group_values.size(),
            self_size
                + group_values.values.allocated_size()
                + group_values.hashes.allocated_size()
                + table_bytes,
        );

        // 4. Emit all (EmitTo::All)
        let remaining_count = group_values.len();
        let emitted_all = group_values.emit(EmitTo::All).unwrap();
        assert_eq!(emitted_all[0].len(), remaining_count);
        assert_eq!(group_values.len(), 0);

        // Clear preserves table capacity
        assert_eq!(group_values.map.allocation_size(), table_bytes);

        // 5. clear_shrink: releases/shrinks map capacity
        group_values.clear_shrink(16);
        let shrunken_table_bytes = group_values.map.allocation_size();
        assert!(shrunken_table_bytes < table_bytes);
        assert_eq!(
            group_values.size(),
            self_size
                + group_values.values.allocated_size()
                + group_values.hashes.allocated_size()
                + shrunken_table_bytes,
        );

        // 6. Reinsert new distinct values into the shrunken table
        let new_arr: ArrayRef = Arc::new(Int32Array::from_iter_values(
            num_distinct..(num_distinct * 2),
        ));
        group_values.intern(&[new_arr], &mut groups).unwrap();
        assert_eq!(groups.len(), num_distinct as usize);
        assert!(group_values.map.allocation_size() > shrunken_table_bytes);
        assert_eq!(
            group_values.size(),
            self_size
                + group_values.values.allocated_size()
                + group_values.hashes.allocated_size()
                + group_values.map.allocation_size(),
        );
    }

    #[test]
    fn clear_shrink_reset_null_group() -> Result<()> {
        let mut gv = GroupValuesPrimitive::<Int32Type>::new(DataType::Int32);

        // Intern some values including a null
        let arr: ArrayRef = Arc::new(Int32Array::from(vec![Some(1), None, Some(2)]));
        let mut groups = vec![];
        gv.intern(&[arr], &mut groups)?;

        assert_eq!(groups.len(), 3);

        let null_group = groups[1];
        assert_eq!(null_group, 1);

        // Clear and shrink
        gv.clear_shrink(0);

        let arr: ArrayRef = Arc::new(Int32Array::from(vec![None::<i32>]));
        let mut groups = vec![];
        gv.intern(&[arr], &mut groups)?;
        assert_eq!(groups.len(), 1);

        let new_null_group = groups[0];
        assert_eq!(new_null_group, 0);

        Ok(())
    }
}
