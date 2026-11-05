// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: See the NOTICE file distributed with this work for additional information regarding copyright ownership
// SPDX-FileContributor: Licensed to the Apache Software Foundation (ASF) under one or more contributor license agreements

//! Exact-key hashing of integer join keys packed into one `u64`.
//!
//! When every join key column is a fixed-width integer and the build side's
//! key ranges fit together in 64 bits, each build key maps injectively to
//!
//! ```text
//! packed = OR_i ((v_i - min_i) << shift_i)
//! ```
//!
//! [`KeyPacker`] hashes `packed` with a bijective mixer, so two keys share a
//! hash only if they are equal. A [`PackedHashMap`] therefore needs no key
//! comparison after a hash table lookup, and hashes all key columns in one
//! fused pass instead of rehashing once per column.
//!
//! A probe value outside its column's build range cannot match any build row.
//! Such rows, like rows with a NULL key, are reported as invalid rather than
//! packed, so they can never alias the packed key of another build row.

use std::fmt::{self, Debug};

use arrow::array::{Array, ArrayRef, AsArray, BooleanArray};
use arrow::buffer::{BooleanBuffer, NullBuffer};
use arrow::compute::cast;
use arrow::datatypes::{
    ArrowPrimitiveType, DataType, Int8Type, Int16Type, Int32Type, Int64Type, UInt8Type,
    UInt16Type, UInt32Type, UInt64Type,
};
use datafusion_common::{NullEquality, Result, ScalarValue, internal_err};
use num_traits::AsPrimitive;

use crate::joins::array_map::ArrayMap;
use crate::joins::join_hash_map::JoinHashMapType;
use crate::joins::utils::matchable_join_keys;

/// XORed into the packed key before mixing, so packed hashes do not coincide
/// with small integers. Any constant keeps the hash a bijection.
const PACKED_HASH_SEED: u64 = 0x9E37_79B9_7F4A_7C15;

/// The murmur3 64-bit finalizer, a bijection on `u64`.
#[inline(always)]
fn mix(packed: u64) -> u64 {
    let mut k = packed ^ PACKED_HASH_SEED;
    k ^= k >> 33;
    k = k.wrapping_mul(0xff51_afd7_ed55_8ccd);
    k ^= k >> 33;
    k = k.wrapping_mul(0xc4ce_b9fe_1a85_ec53);
    k ^= k >> 33;
    k
}

/// Where one key column lives in the packed key.
#[derive(Debug, Clone)]
struct PackedColumn {
    /// The build key type. Probe keys of another type are cast to it.
    data_type: DataType,
    /// The build side minimum, sign or zero extended to `u64`.
    min: u64,
    /// `max - min` of the build side: offsets above it are out of range.
    range: u64,
    /// Position of the column's lowest bit in the packed key.
    shift: u32,
}

/// Packs integer join keys into one `u64` relative to the build side's
/// per-column minimum, and hashes the packed key bijectively.
///
/// Values are sign (signed types) or zero (unsigned types) extended to `u64`,
/// and `offset = v - min` is computed with wrapping arithmetic. In that domain
/// `offset <= max - min` holds exactly when `min <= v <= max`: for `v < min`
/// the offset is `2^64 - (min - v)`, and `(min - v) + (max - min) = max - v`
/// is below `2^64`.
#[derive(Clone)]
pub struct KeyPacker {
    columns: Vec<PackedColumn>,
}

impl Debug for KeyPacker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries(
                self.columns
                    .iter()
                    .map(|c| (c.data_type.to_string(), c.shift, c.range)),
            )
            .finish()
    }
}

impl KeyPacker {
    /// Whether keys of `data_type` can be packed.
    pub(crate) fn is_supported_type(data_type: &DataType) -> bool {
        ArrayMap::is_supported_type(data_type)
    }

    /// Creates a packer for keys of `data_types` with the build side bounds
    /// `bounds` (`(min, max)` per column), or `None` if a type is not
    /// supported, a bound is missing or NULL, or the key ranges need more than
    /// 64 bits together.
    pub(crate) fn try_new<'a>(
        data_types: &[DataType],
        bounds: impl IntoIterator<Item = Option<(&'a ScalarValue, &'a ScalarValue)>>,
    ) -> Option<Self> {
        let mut columns = Vec::with_capacity(data_types.len());
        let mut shift = 0u32;
        let mut bounds = bounds.into_iter();
        for data_type in data_types {
            if !Self::is_supported_type(data_type) {
                return None;
            }
            let (min, max) = bounds.next()??;
            let min = ArrayMap::key_to_u64(min)?;
            let max = ArrayMap::key_to_u64(max)?;
            let range = max.wrapping_sub(min);
            let bits = u64::BITS - range.leading_zeros();
            if shift + bits > u64::BITS {
                return None;
            }
            columns.push(PackedColumn {
                data_type: data_type.clone(),
                min,
                range,
                shift,
            });
            shift += bits;
        }
        (!columns.is_empty()).then_some(Self { columns })
    }

    /// Total number of bits of the packed key.
    #[cfg(test)]
    pub(crate) fn bits(&self) -> u32 {
        self.columns
            .last()
            .map(|c| c.shift + (u64::BITS - c.range.leading_zeros()))
            .unwrap_or(0)
    }

    /// Writes the packed hash of each build row of `keys` to `hashes`.
    ///
    /// Every non-NULL build value must lie within the bounds the packer was
    /// created with. The hashes of rows with a NULL key are meaningless.
    pub(crate) fn hash_build(&self, keys: &[ArrayRef], hashes: &mut [u64]) -> Result<()> {
        self.check_arity(keys)?;
        hashes.fill(0);
        for (column, array) in self.columns.iter().zip(keys) {
            if array.data_type() != &column.data_type {
                return internal_err!(
                    "packed build key has type {}, expected {}",
                    array.data_type(),
                    column.data_type
                );
            }
            dispatch_pack(array.as_ref(), column, hashes, false);
        }
        for h in hashes.iter_mut() {
            *h = mix(*h);
        }
        Ok(())
    }

    /// Writes the packed hash of each probe row of `keys` to `hashes`, and
    /// returns which rows can match a build row: those without a NULL key
    /// and with every value within the build bounds. `None` means every row
    /// can. The hashes of the other rows are meaningless.
    ///
    /// No build row with a NULL key is ever packed (see
    /// [`PackedHashMap`]), so NULL probe keys are invalid under either
    /// [`NullEquality`].
    pub(crate) fn hash_probe(
        &self,
        keys: &[ArrayRef],
        hashes: &mut [u64],
    ) -> Result<Option<NullBuffer>> {
        self.check_arity(keys)?;
        hashes.fill(0);
        let mut valid: Option<BooleanBuffer> = None;
        for (column, array) in self.columns.iter().zip(keys) {
            let cast_array;
            let array = if array.data_type() == &column.data_type {
                array
            } else {
                // A value the build type cannot hold becomes NULL, and cannot
                // match any build value.
                cast_array = cast(array, &column.data_type)?;
                &cast_array
            };
            let in_range = dispatch_pack(array.as_ref(), column, hashes, true)
                .expect("range checked packing returns a bitmap");
            let column_valid = match array.logical_nulls() {
                Some(nulls) => &in_range & nulls.inner(),
                None => in_range,
            };
            valid = Some(match valid {
                None => column_valid,
                Some(valid) => &valid & &column_valid,
            });
        }
        for h in hashes.iter_mut() {
            *h = mix(*h);
        }
        Ok(valid
            .map(NullBuffer::new)
            .filter(|valid| valid.null_count() > 0))
    }

    fn check_arity(&self, keys: &[ArrayRef]) -> Result<()> {
        if keys.len() != self.columns.len() {
            return internal_err!(
                "expected {} packed join keys, got {}",
                self.columns.len(),
                keys.len()
            );
        }
        Ok(())
    }
}

/// ORs the offsets of `array`'s values into `packed`, returning which values
/// lie within the column's range when `check_range` is set.
fn dispatch_pack(
    array: &dyn Array,
    column: &PackedColumn,
    packed: &mut [u64],
    check_range: bool,
) -> Option<BooleanBuffer> {
    match column.data_type {
        DataType::Int8 => pack_column::<Int8Type>(array, column, packed, check_range),
        DataType::Int16 => pack_column::<Int16Type>(array, column, packed, check_range),
        DataType::Int32 => pack_column::<Int32Type>(array, column, packed, check_range),
        DataType::Int64 => pack_column::<Int64Type>(array, column, packed, check_range),
        DataType::UInt8 => pack_column::<UInt8Type>(array, column, packed, check_range),
        DataType::UInt16 => pack_column::<UInt16Type>(array, column, packed, check_range),
        DataType::UInt32 => pack_column::<UInt32Type>(array, column, packed, check_range),
        DataType::UInt64 => pack_column::<UInt64Type>(array, column, packed, check_range),
        // `KeyPacker::try_new` only admits the types above.
        _ => unreachable!("unsupported packed key type {}", column.data_type),
    }
}

#[inline]
fn pack_column<T: ArrowPrimitiveType>(
    array: &dyn Array,
    column: &PackedColumn,
    packed: &mut [u64],
    check_range: bool,
) -> Option<BooleanBuffer>
where
    T::Native: AsPrimitive<u64>,
{
    let values = array.as_primitive::<T>().values();
    let (min, range, shift) = (column.min, column.range, column.shift);
    debug_assert_eq!(values.len(), packed.len());
    if check_range {
        Some(BooleanBuffer::collect_bool(values.len(), |i| {
            let offset = values[i].as_().wrapping_sub(min);
            // An out of range offset may spill into other columns' bits; the
            // row is reported invalid, so its hash is never looked up. The
            // shift can be 64 after a full-width column, where every valid
            // offset is 0.
            packed[i] |= offset.wrapping_shl(shift);
            offset <= range
        }))
    } else {
        for (p, v) in packed.iter_mut().zip(values.iter()) {
            *p |= v.as_().wrapping_sub(min).wrapping_shl(shift);
        }
        None
    }
}

/// A [`JoinHashMapType`] keyed by the packed hashes of a [`KeyPacker`].
///
/// The packed hash is a bijection of the key, so a hash match is a key match:
/// probing needs no key comparison. Probe keys must be hashed with
/// [`Self::packer`], not with the join's `RandomState`.
///
/// Rows with a NULL key are never inserted: under
/// [`NullEquality::NullEqualsNull`] a packed map is only built when no build
/// key is NULL.
pub struct PackedHashMap {
    map: Box<dyn JoinHashMapType>,
    packer: KeyPacker,
}

impl Debug for PackedHashMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PackedHashMap")
            .field("packer", &self.packer)
            .field("len", &self.map.len())
            .finish()
    }
}

impl PackedHashMap {
    pub(crate) fn new(map: Box<dyn JoinHashMapType>, packer: KeyPacker) -> Self {
        Self { map, packer }
    }

    /// The underlying hash table, keyed by packed hashes.
    pub(crate) fn map(&self) -> &dyn JoinHashMapType {
        self.map.as_ref()
    }

    /// The packer the probe keys must be hashed with.
    pub fn packer(&self) -> &KeyPacker {
        &self.packer
    }

    /// Number of distinct keys in the map.
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Whether the map contains no keys.
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Which rows of `keys` (one array per join key) are in the map.
    pub(crate) fn contain_keys(&self, keys: &[ArrayRef]) -> Result<BooleanArray> {
        let num_rows = keys.first().map(|k| k.len()).unwrap_or(0);
        let mut hashes = vec![0; num_rows];
        let valid = self.packer.hash_probe(keys, &mut hashes)?;
        let found = self.map.contain_hashes(&hashes);
        Ok(match valid {
            Some(valid) => BooleanArray::new(found.values() & valid.inner(), None),
            None => found,
        })
    }
}

/// Inserts the build rows of `keys`, indexed from `offset`, into `map` keyed by
/// their packed hash, keeping the first row of each key at the head of its
/// chain. Rows with a NULL key are skipped under
/// [`NullEquality::NullEqualsNothing`]; under [`NullEquality::NullEqualsNull`]
/// the caller guarantees there are none.
pub(crate) fn update_packed_hash(
    keys: &[ArrayRef],
    packer: &KeyPacker,
    map: &mut dyn JoinHashMapType,
    offset: usize,
    hashes_buffer: &mut [u64],
    null_equality: NullEquality,
) -> Result<()> {
    packer.hash_build(keys, hashes_buffer)?;
    let valid_keys = matchable_join_keys(keys, null_equality);
    map.update_from_hashes(hashes_buffer, valid_keys.as_ref(), offset, 0, true);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::joins::join_hash_map::JoinHashMapU32;
    use arrow::array::{Int8Array, Int32Array, Int64Array, UInt8Array, UInt64Array};
    use std::collections::HashSet;
    use std::sync::Arc;

    fn packer(types: &[DataType], bounds: &[(ScalarValue, ScalarValue)]) -> KeyPacker {
        KeyPacker::try_new(types, bounds.iter().map(|(a, b)| Some((a, b)))).unwrap()
    }

    #[test]
    fn rejects_unsupported_or_too_wide() {
        let b = (
            ScalarValue::Int64(Some(0)),
            ScalarValue::Int64(Some(1 << 40)),
        );
        let i64s = [DataType::Int64, DataType::Int64];
        assert!(
            KeyPacker::try_new(
                &i64s,
                [b.clone(), b.clone()].iter().map(|(a, b)| Some((a, b)))
            )
            .is_none()
        );
        let one = [b.clone()];
        assert_eq!(packer(&[DataType::Int64], &one).bits(), 41);
        assert!(
            KeyPacker::try_new(
                &[DataType::Utf8],
                [Some((&ScalarValue::from("a"), &ScalarValue::from("b")))]
            )
            .is_none()
        );
        let null = (ScalarValue::Int64(None), ScalarValue::Int64(None));
        assert!(
            KeyPacker::try_new(&[DataType::Int64], [Some((&null.0, &null.1))]).is_none()
        );
        assert!(KeyPacker::try_new(&[DataType::Int64], [None]).is_none());
        // A full-width column followed by a constant column still fits.
        let full = (
            ScalarValue::Int64(Some(i64::MIN)),
            ScalarValue::Int64(Some(i64::MAX)),
        );
        let constant = (ScalarValue::Int8(Some(-3)), ScalarValue::Int8(Some(-3)));
        let p = packer(&[DataType::Int64, DataType::Int8], &[full, constant]);
        assert_eq!(p.bits(), 64);
    }

    #[test]
    fn packed_hashes_are_unique_and_range_checked() -> Result<()> {
        // Signed column with negative values, unsigned column.
        let p = packer(
            &[DataType::Int8, DataType::UInt8],
            &[
                (ScalarValue::Int8(Some(-5)), ScalarValue::Int8(Some(5))),
                (ScalarValue::UInt8(Some(10)), ScalarValue::UInt8(Some(20))),
            ],
        );
        assert_eq!(p.bits(), 4 + 4);
        let mut a = vec![];
        let mut b = vec![];
        for x in -5i8..=5 {
            for y in 10u8..=20 {
                a.push(x);
                b.push(y);
            }
        }
        let keys: Vec<ArrayRef> =
            vec![Arc::new(Int8Array::from(a)), Arc::new(UInt8Array::from(b))];
        let mut hashes = vec![0; keys[0].len()];
        p.hash_build(&keys, &mut hashes)?;
        assert_eq!(hashes.iter().collect::<HashSet<_>>().len(), hashes.len());

        let mut probe_hashes = vec![0; hashes.len()];
        assert!(p.hash_probe(&keys, &mut probe_hashes)?.is_none());
        assert_eq!(hashes, probe_hashes);

        // Out of range in either direction, at either end, and NULL.
        let probe: Vec<ArrayRef> = vec![
            Arc::new(Int8Array::from(vec![
                Some(-6),
                Some(6),
                Some(i8::MIN),
                Some(i8::MAX),
                Some(0),
                None,
                Some(-5),
                Some(5),
            ])),
            Arc::new(UInt8Array::from(vec![
                Some(10),
                Some(10),
                Some(10),
                Some(10),
                Some(255),
                Some(10),
                Some(10),
                Some(20),
            ])),
        ];
        let mut probe_hashes = vec![0; 8];
        let valid = p.hash_probe(&probe, &mut probe_hashes)?.unwrap();
        let valid: Vec<bool> = (0..8).map(|i| valid.is_valid(i)).collect();
        assert_eq!(
            valid,
            vec![false, false, false, false, false, false, true, true]
        );
        assert_eq!(probe_hashes[6], hashes[0]);
        assert_eq!(probe_hashes[7], *hashes.last().unwrap());
        Ok(())
    }

    #[test]
    fn full_width_columns() -> Result<()> {
        let p = packer(
            &[DataType::UInt64],
            &[(
                ScalarValue::UInt64(Some(0)),
                ScalarValue::UInt64(Some(u64::MAX)),
            )],
        );
        let keys: Vec<ArrayRef> = vec![Arc::new(UInt64Array::from(vec![0, u64::MAX, 7]))];
        let mut hashes = vec![0; 3];
        assert!(p.hash_probe(&keys, &mut hashes)?.is_none());
        assert_eq!(hashes.iter().collect::<HashSet<_>>().len(), 3);

        let p = packer(
            &[DataType::Int64],
            &[(ScalarValue::Int64(Some(-10)), ScalarValue::Int64(Some(10)))],
        );
        let keys: Vec<ArrayRef> = vec![Arc::new(Int64Array::from(vec![
            i64::MIN,
            i64::MAX,
            -10,
            10,
            -11,
        ]))];
        let mut hashes = vec![0; 5];
        let valid = p.hash_probe(&keys, &mut hashes)?.unwrap();
        let valid: Vec<bool> = (0..5).map(|i| valid.is_valid(i)).collect();
        assert_eq!(valid, vec![false, false, true, true, false]);
        Ok(())
    }

    #[test]
    fn probe_keys_of_another_type_are_cast() -> Result<()> {
        let p = packer(
            &[DataType::Int32],
            &[(ScalarValue::Int32(Some(1)), ScalarValue::Int32(Some(3)))],
        );
        let build: Vec<ArrayRef> = vec![Arc::new(Int32Array::from(vec![1, 2, 3]))];
        let mut map = JoinHashMapU32::with_capacity(3);
        let mut hashes = vec![0; 3];
        update_packed_hash(
            &build,
            &p,
            &mut map,
            0,
            &mut hashes,
            NullEquality::NullEqualsNothing,
        )?;
        let map = PackedHashMap::new(Box::new(map), p);
        // Int64 probe values, one beyond the Int32 domain.
        let probe: Vec<ArrayRef> =
            vec![Arc::new(Int64Array::from(vec![2, 4, 1 << 33, 3]))];
        let found = map.contain_keys(&probe)?;
        assert_eq!(
            found.iter().collect::<Vec<_>>(),
            vec![Some(true), Some(false), Some(false), Some(true)]
        );
        Ok(())
    }

    #[test]
    fn null_build_keys_are_not_inserted() -> Result<()> {
        let p = packer(
            &[DataType::Int32, DataType::Int32],
            &[
                (ScalarValue::Int32(Some(1)), ScalarValue::Int32(Some(3))),
                (ScalarValue::Int32(Some(1)), ScalarValue::Int32(Some(3))),
            ],
        );
        let build: Vec<ArrayRef> = vec![
            Arc::new(Int32Array::from(vec![Some(1), None, Some(3)])),
            Arc::new(Int32Array::from(vec![Some(1), Some(2), Some(3)])),
        ];
        let mut map = JoinHashMapU32::with_capacity(3);
        let mut hashes = vec![0; 3];
        update_packed_hash(
            &build,
            &p,
            &mut map,
            0,
            &mut hashes,
            NullEquality::NullEqualsNothing,
        )?;
        let map = PackedHashMap::new(Box::new(map), p);
        assert_eq!(map.len(), 2);
        let found = map.contain_keys(&build)?;
        assert_eq!(
            found.iter().collect::<Vec<_>>(),
            vec![Some(true), Some(false), Some(true)]
        );
        Ok(())
    }
}
