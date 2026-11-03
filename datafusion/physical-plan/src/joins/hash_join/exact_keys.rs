// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: See the NOTICE file distributed with this work for additional information regarding copyright ownership
// SPDX-FileContributor: Licensed to the Apache Software Foundation (ASF) under one or more contributor license agreements

//! Exact hashing of single integer join keys.
//!
//! A hash join on one integer key of at most 64 bits can hash each key with a
//! bijection of its value widened to `u64`. Two keys then have equal hashes
//! exactly when they are equal, so every candidate the hash table returns is a
//! match and the per-pair key comparison, a random read of the build-side key
//! for every candidate, can be skipped.
//!
//! The build side, the probe side and the dynamic filter's hash table lookup
//! must all hash the same way, so they ask the table whether it was built with
//! exact keys (see [`JoinHashMapType::exact_keys`]).
//!
//! [`JoinHashMapType::exact_keys`]: crate::joins::join_hash_map::JoinHashMapType::exact_keys

use arrow::array::{Array, ArrayRef, ArrowPrimitiveType, AsArray};
use arrow::datatypes::{
    DataType, Int8Type, Int16Type, Int32Type, Int64Type, UInt8Type, UInt16Type,
    UInt32Type, UInt64Type,
};
use datafusion_common::hash_utils::{RandomState, create_hashes};
use datafusion_common::{NullEquality, Result, internal_err};
use num_traits::AsPrimitive;

/// Whether a join on these key types under `null_equality` can use exact
/// hashing: a single key of an integer type of at most 64 bits, where NULL
/// keys never match (they are skipped, so their hash is irrelevant).
pub(crate) fn supports_exact_keys(
    key_types: &[DataType],
    null_equality: NullEquality,
) -> bool {
    null_equality == NullEquality::NullEqualsNothing
        && matches!(
            key_types,
            [DataType::Int8
                | DataType::Int16
                | DataType::Int32
                | DataType::Int64
                | DataType::UInt8
                | DataType::UInt16
                | DataType::UInt32
                | DataType::UInt64]
        )
}

/// The murmur3 64-bit finalizer: xor-shifts and multiplications by odd
/// constants, each invertible, so it is a bijection on `u64` that still mixes
/// the low bits of sequential keys into the high bits the hash table uses.
#[inline(always)]
fn mix(mut k: u64) -> u64 {
    k ^= k >> 33;
    k = k.wrapping_mul(0xff51_afd7_ed55_8ccd);
    k ^= k >> 33;
    k = k.wrapping_mul(0xc4ce_b9fe_1a85_ec53);
    k ^ (k >> 33)
}

fn fill<T: ArrowPrimitiveType>(array: &ArrayRef, out: &mut [u64])
where
    T::Native: AsPrimitive<u64>,
{
    // NULL slots hash their (arbitrary) value: callers skip NULL keys.
    for (o, v) in out.iter_mut().zip(array.as_primitive::<T>().values()) {
        *o = mix(v.as_());
    }
}

/// Writes the exact hash of every row of the single integer key in `keys`
/// into `out`, which must have one slot per row.
pub(crate) fn exact_key_hashes(keys: &[ArrayRef], out: &mut [u64]) -> Result<()> {
    let [array] = keys else {
        return internal_err!("exact join key hashing expects 1 key, got {}", keys.len());
    };
    if array.len() != out.len() {
        return internal_err!(
            "exact join key hashing got {} rows for {} hash slots",
            array.len(),
            out.len()
        );
    }
    match array.data_type() {
        DataType::Int8 => fill::<Int8Type>(array, out),
        DataType::Int16 => fill::<Int16Type>(array, out),
        DataType::Int32 => fill::<Int32Type>(array, out),
        DataType::Int64 => fill::<Int64Type>(array, out),
        DataType::UInt8 => fill::<UInt8Type>(array, out),
        DataType::UInt16 => fill::<UInt16Type>(array, out),
        DataType::UInt32 => fill::<UInt32Type>(array, out),
        DataType::UInt64 => fill::<UInt64Type>(array, out),
        other => {
            return internal_err!("exact join key hashing does not support {other}");
        }
    }
    Ok(())
}

/// Hashes join keys into `out` the way a hash table built with
/// `exact` keys expects them.
pub(crate) fn join_key_hashes(
    keys: &[ArrayRef],
    random_state: &RandomState,
    exact: bool,
    out: &mut [u64],
) -> Result<()> {
    if exact {
        exact_key_hashes(keys, out)
    } else {
        create_hashes(keys, random_state, out).map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrow::array::{Int8Array, Int32Array, Int64Array, UInt64Array};
    use std::collections::HashSet;
    use std::sync::Arc;

    #[test]
    fn mix_is_injective_on_a_dense_range_and_extremes() {
        let mut seen = HashSet::new();
        for v in (-100_000i64..100_000).chain([i64::MIN, i64::MAX]) {
            assert!(seen.insert(mix(v as u64)), "collision at {v}");
        }
    }

    #[test]
    fn signed_keys_hash_like_their_value() -> Result<()> {
        let narrow: ArrayRef = Arc::new(Int8Array::from(vec![-1, 0, 5]));
        let wide: ArrayRef = Arc::new(Int64Array::from(vec![-1, 0, 5]));
        let (mut a, mut b) = (vec![0; 3], vec![0; 3]);
        exact_key_hashes(&[narrow], &mut a)?;
        exact_key_hashes(&[wide], &mut b)?;
        assert_eq!(a, b);
        Ok(())
    }

    #[test]
    fn supports_only_single_integer_keys_without_null_matching() {
        let eq = NullEquality::NullEqualsNothing;
        assert!(supports_exact_keys(&[DataType::Int32], eq));
        assert!(supports_exact_keys(&[DataType::UInt64], eq));
        assert!(!supports_exact_keys(
            &[DataType::Int32, DataType::Int32],
            eq
        ));
        assert!(!supports_exact_keys(&[DataType::Float64], eq));
        assert!(!supports_exact_keys(&[DataType::Utf8View], eq));
        assert!(!supports_exact_keys(
            &[DataType::Int32],
            NullEquality::NullEqualsNull
        ));
    }

    #[test]
    fn rejects_unsupported_input() {
        let ints: ArrayRef = Arc::new(Int32Array::from(vec![1, 2]));
        let mut out = vec![0; 2];
        assert!(
            exact_key_hashes(&[Arc::clone(&ints), Arc::clone(&ints)], &mut out).is_err()
        );
        assert!(exact_key_hashes(&[ints], &mut [0; 3]).is_err());
        let wide: ArrayRef = Arc::new(UInt64Array::from(vec![u64::MAX, 0]));
        assert!(exact_key_hashes(&[wide], &mut out).is_ok());
    }
}
