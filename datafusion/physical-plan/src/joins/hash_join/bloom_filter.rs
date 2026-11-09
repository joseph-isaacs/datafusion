// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: See the NOTICE file distributed with this work for additional information regarding copyright ownership
// SPDX-FileContributor: Licensed to the Apache Software Foundation (ASF) under one or more contributor license agreements

//! Split-block Bloom filter over hash join build keys, used as a compact
//! membership filter for predicate transfer (see
//! `datafusion.optimizer.enable_predicate_transfer`).
//!
//! The filter is built from the same hashes the join's hash table is built
//! from, so a probe key that has a match in the hash table always passes the
//! Bloom filter: it has false positives but never false negatives.

use std::fmt::{self, Display};
use std::hash::Hash;
use std::sync::Arc;

use arrow::array::{ArrayRef, BooleanArray};
use arrow::buffer::BooleanBuffer;
use arrow::datatypes::{DataType, Schema};
use arrow::record_batch::RecordBatch;
use datafusion_common::Result;
use datafusion_common::hash_utils::with_hashes;
use datafusion_expr::ColumnarValue;
use datafusion_expr_common::dyn_eq::DynHash;
use datafusion_physical_expr_common::physical_expr::{PhysicalExpr, PhysicalExprRef};

use crate::joins::hash_join::partitioned_hash_eval::SeededRandomState;
use crate::joins::key_range_bitmap::KeyRangeBitmap;

/// Bits the Bloom filter allots per build-side row, before rounding the
/// number of blocks up to a power of two. With 8 bits set per key in a
/// 256-bit block this keeps the false positive rate well under 1%.
pub(crate) const BLOOM_BITS_PER_KEY: usize = 16;

/// Largest Bloom filter built for one join, in bytes. Past this the join
/// keeps its hash-table lookup.
const MAX_BLOOM_BYTES: usize = 128 * 1024 * 1024;

/// One 256-bit block: eight 32-bit words, one bit set per word per key.
type Block = [u32; 8];

/// Odd constants spreading the low 32 hash bits over the eight words of a
/// block, as in the Parquet split-block Bloom filter specification.
const SALT: [u32; 8] = [
    0x47b6137b, 0x44974d91, 0x8824ad5b, 0xa2b7289d, 0x705495c7, 0x2df1424b, 0x9efc4947,
    0x5c6bfb31,
];

/// A split-block Bloom filter (Putze et al., as used by Parquet) over 64-bit
/// key hashes. The upper 32 bits of a hash choose the block and the lower 32
/// bits choose one bit in each of its words, so a lookup touches one cache
/// line.
///
/// The block count is a power of two and blocks are chosen by the low bits of
/// the upper hash half. A filter can therefore be [folded](Self::fold_to) to
/// any smaller power of two without losing a key, which is how filters built
/// independently by the partitions of a partitioned join are merged.
#[derive(Clone, PartialEq, Eq)]
pub struct SplitBlockBloomFilter {
    blocks: Vec<Block>,
}

impl fmt::Debug for SplitBlockBloomFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SplitBlockBloomFilter")
            .field("blocks", &self.blocks.len())
            .finish()
    }
}

#[inline]
fn block_mask(hash: u32) -> Block {
    let mut mask = [0u32; 8];
    for (word, salt) in mask.iter_mut().zip(SALT) {
        *word = 1u32 << (hash.wrapping_mul(salt) >> 27);
    }
    mask
}

impl SplitBlockBloomFilter {
    /// A filter sized for `num_keys` keys at `bits_per_key` bits each, or
    /// `None` if it would be larger than [`MAX_BLOOM_BYTES`].
    pub fn try_with_capacity(num_keys: usize, bits_per_key: usize) -> Option<Self> {
        let bits = num_keys.checked_mul(bits_per_key)?;
        let num_blocks = bits.div_ceil(256).max(1).checked_next_power_of_two()?;
        if num_blocks.checked_mul(size_of::<Block>())? > MAX_BLOOM_BYTES {
            return None;
        }
        Some(Self {
            blocks: vec![[0; 8]; num_blocks],
        })
    }

    #[inline]
    fn block_index(&self, hash: u64) -> usize {
        ((hash >> 32) as usize) & (self.blocks.len() - 1)
    }

    /// Adds the key with this hash.
    #[inline]
    pub fn insert(&mut self, hash: u64) {
        let index = self.block_index(hash);
        let mask = block_mask(hash as u32);
        let block = &mut self.blocks[index];
        for (word, bit) in block.iter_mut().zip(mask) {
            *word |= bit;
        }
    }

    /// Adds every key in `hashes`.
    pub fn insert_hashes(&mut self, hashes: &[u64]) {
        for &hash in hashes {
            self.insert(hash);
        }
    }

    /// Whether the key with this hash may have been inserted. `false` means it
    /// certainly was not.
    #[inline]
    pub fn contains(&self, hash: u64) -> bool {
        let block = &self.blocks[self.block_index(hash)];
        let mask = block_mask(hash as u32);
        // Branch-free: the key is absent if any of its bits is unset.
        block
            .iter()
            .zip(mask)
            .fold(0u32, |missing, (word, bit)| missing | (bit & !word))
            == 0
    }

    /// Number of 256-bit blocks.
    pub fn num_blocks(&self) -> usize {
        self.blocks.len()
    }

    /// Heap size in bytes.
    pub fn size(&self) -> usize {
        self.blocks.len() * size_of::<Block>()
    }

    /// This filter folded to `num_blocks` blocks, a power of two no larger
    /// than [`Self::num_blocks`]: every key the filter contains is still
    /// contained.
    pub fn fold_to(&self, num_blocks: usize) -> Self {
        debug_assert!(num_blocks.is_power_of_two() && num_blocks <= self.num_blocks());
        let mut blocks = self.blocks[..num_blocks].to_vec();
        for chunk in self.blocks[num_blocks..].chunks(num_blocks) {
            for (dst, src) in blocks.iter_mut().zip(chunk) {
                for (d, s) in dst.iter_mut().zip(src) {
                    *d |= s;
                }
            }
        }
        Self { blocks }
    }

    /// Union of `filters`, folded to the smallest of them. `None` when
    /// `filters` is empty.
    pub fn union<'a>(
        filters: impl IntoIterator<Item = &'a Self> + Clone,
    ) -> Option<Self> {
        let num_blocks = filters.clone().into_iter().map(Self::num_blocks).min()?;
        let mut iter = filters.into_iter();
        let mut merged = iter.next()?.fold_to(num_blocks);
        for filter in iter {
            let folded;
            let filter = if filter.num_blocks() == num_blocks {
                filter
            } else {
                folded = filter.fold_to(num_blocks);
                &folded
            };
            for (dst, src) in merged.blocks.iter_mut().zip(&filter.blocks) {
                for (d, s) in dst.iter_mut().zip(src) {
                    *d |= s;
                }
            }
        }
        Some(merged)
    }
}

/// Physical expression that tests join keys against a [`SplitBlockBloomFilter`]
/// built from the build side's keys.
///
/// It is the predicate transfer counterpart of [`HashTableLookupExpr`]: it
/// hashes the probe keys the way the join hashes its build keys, but checks
/// a compact Bloom filter instead of the hash table. The filter is a few bits
/// per key rather than the hash table's entries, so it stays cache resident
/// on large builds, it holds no reference to the join's hash table, and the
/// filters of a partitioned join merge into one that needs no partition
/// routing. It may keep rows without a match (false positives) but never
/// drops a row with one.
///
/// [`HashTableLookupExpr`]: super::HashTableLookupExpr
pub struct BloomFilterExpr {
    /// Probe-side join key expressions
    on_columns: Vec<PhysicalExprRef>,
    /// Hashing used to build the filter
    random_state: SeededRandomState,
    bloom: Arc<SplitBlockBloomFilter>,
    /// Key-range summary of the build side, for container pruning
    pruning_bitmap: Option<Arc<KeyRangeBitmap>>,
}

impl BloomFilterExpr {
    /// Create a Bloom membership test of `on_columns` against `bloom`, which
    /// holds the build keys hashed with `random_state`.
    pub fn new(
        on_columns: Vec<PhysicalExprRef>,
        random_state: SeededRandomState,
        bloom: Arc<SplitBlockBloomFilter>,
        pruning_bitmap: Option<Arc<KeyRangeBitmap>>,
    ) -> Self {
        Self {
            on_columns,
            random_state,
            bloom,
            pruning_bitmap,
        }
    }

    /// The Bloom filter probe keys are checked against.
    pub fn bloom(&self) -> &Arc<SplitBlockBloomFilter> {
        &self.bloom
    }

    /// Which parts of the build side's key range hold keys, for container
    /// pruning.
    pub fn pruning_bitmap(&self) -> Option<&Arc<KeyRangeBitmap>> {
        self.pruning_bitmap.as_ref()
    }

    /// Membership of each row of `keys`.
    fn contains_keys(&self, keys: &[ArrayRef]) -> Result<BooleanArray> {
        with_hashes(keys, self.random_state.random_state(), |hashes| {
            let bloom = self.bloom.as_ref();
            let bits =
                BooleanBuffer::collect_bool(hashes.len(), |i| bloom.contains(hashes[i]));
            Ok(BooleanArray::new(bits, None))
        })
    }
}

impl fmt::Debug for BloomFilterExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cols = self
            .on_columns
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        write!(
            f,
            "bloom_filter({cols}, [{}], {} blocks)",
            self.random_state.seed(),
            self.bloom.num_blocks()
        )
    }
}

impl Hash for BloomFilterExpr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.on_columns.dyn_hash(state);
        self.random_state.seed().hash(state);
        Arc::as_ptr(&self.bloom).hash(state);
    }
}

impl PartialEq for BloomFilterExpr {
    fn eq(&self, other: &Self) -> bool {
        // The filter is immutable once built, so sharing it implies equal
        // contents; distinct filters are not compared bit by bit.
        self.on_columns == other.on_columns
            && self.random_state.seed() == other.random_state.seed()
            && Arc::ptr_eq(&self.bloom, &other.bloom)
    }
}

impl Eq for BloomFilterExpr {}

impl Display for BloomFilterExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bloom_filter")
    }
}

impl PhysicalExpr for BloomFilterExpr {
    fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>> {
        self.on_columns.iter().collect()
    }

    fn with_new_children(
        self: Arc<Self>,
        children: Vec<Arc<dyn PhysicalExpr>>,
    ) -> Result<Arc<dyn PhysicalExpr>> {
        Ok(Arc::new(Self {
            on_columns: children,
            random_state: self.random_state.clone(),
            bloom: Arc::clone(&self.bloom),
            pruning_bitmap: self.pruning_bitmap.clone(),
        }))
    }

    fn data_type(&self, _input_schema: &Schema) -> Result<DataType> {
        Ok(DataType::Boolean)
    }

    fn nullable(&self, _input_schema: &Schema) -> Result<bool> {
        Ok(false)
    }

    fn evaluate(&self, batch: &RecordBatch) -> Result<ColumnarValue> {
        let num_rows = batch.num_rows();
        let keys = self
            .on_columns
            .iter()
            .map(|c| c.evaluate(batch)?.into_array(num_rows))
            .collect::<Result<Vec<_>>>()?;
        Ok(ColumnarValue::Array(Arc::new(self.contains_keys(&keys)?)))
    }

    #[cfg(feature = "proto")]
    fn try_to_proto(
        &self,
        _ctx: &datafusion_physical_expr_common::physical_expr::proto_encode::PhysicalExprEncodeCtx<'_>,
    ) -> Result<Option<datafusion_proto_models::protobuf::PhysicalExprNode>> {
        use datafusion_proto_models::protobuf;
        use datafusion_proto_models::protobuf::physical_expr_node::ExprType;

        // The protobuf schema has no Bloom filter node yet, so, like a hash
        // table lookup, the filter is replaced with `true`. This keeps every
        // row, which is always correct for a dynamic filter.
        let value = datafusion_proto_common::ScalarValue {
            value: Some(datafusion_proto_common::scalar_value::Value::BoolValue(
                true,
            )),
        };
        Ok(Some(protobuf::PhysicalExprNode {
            expr_id: None,
            expr_type: Some(ExprType::Literal(value)),
        }))
    }

    fn fmt_sql(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "bloom_filter")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::joins::hash_join::exec::HASH_JOIN_SEED;
    use arrow::array::{Array, Int64Array, StringArray};
    use arrow::datatypes::Field;
    use datafusion_common::hash_utils::create_hashes;
    use datafusion_physical_expr::expressions::Column;

    fn hashes(keys: &[ArrayRef]) -> Vec<u64> {
        let mut hashes = vec![0; keys[0].len()];
        create_hashes(keys, HASH_JOIN_SEED.random_state(), &mut hashes).unwrap();
        hashes
    }

    fn bloom_of(keys: &[ArrayRef]) -> SplitBlockBloomFilter {
        let mut bloom =
            SplitBlockBloomFilter::try_with_capacity(keys[0].len(), BLOOM_BITS_PER_KEY)
                .unwrap();
        bloom.insert_hashes(&hashes(keys));
        bloom
    }

    fn false_positive_rate(bloom: &SplitBlockBloomFilter, absent: &[u64]) -> f64 {
        absent.iter().filter(|&&h| bloom.contains(h)).count() as f64 / absent.len() as f64
    }

    #[test]
    fn no_false_negatives_and_bounded_fpr() {
        for n in [1usize, 7, 1000, 100_000, 1_000_000] {
            let present: ArrayRef =
                Arc::new(Int64Array::from_iter_values((0..n as i64).map(|i| i * 3)));
            let bloom = bloom_of(std::slice::from_ref(&present));
            let present_hashes = hashes(&[present]);
            assert!(present_hashes.iter().all(|&h| bloom.contains(h)), "n={n}");

            let absent: ArrayRef = Arc::new(Int64Array::from_iter_values(
                (0..200_000i64).map(|i| i * 3 + 1),
            ));
            let fpr = false_positive_rate(&bloom, &hashes(&[absent]));
            assert!(fpr < 0.01, "n={n} fpr={fpr}");
        }
    }

    #[test]
    fn fold_and_union_keep_every_key() {
        let a: ArrayRef = Arc::new(Int64Array::from_iter_values(0..50_000));
        let b: ArrayRef = Arc::new(Int64Array::from_iter_values(50_000..60_000));
        let bloom_a = bloom_of(std::slice::from_ref(&a));
        let bloom_b = bloom_of(std::slice::from_ref(&b));
        assert!(bloom_a.num_blocks() > bloom_b.num_blocks());

        let folded = bloom_a.fold_to(bloom_b.num_blocks());
        assert!(
            hashes(std::slice::from_ref(&a))
                .iter()
                .all(|&h| folded.contains(h))
        );

        let merged = SplitBlockBloomFilter::union([&bloom_a, &bloom_b]).unwrap();
        assert_eq!(merged.num_blocks(), bloom_b.num_blocks());
        for keys in [a, b] {
            assert!(hashes(&[keys]).iter().all(|&h| merged.contains(h)));
        }
        assert!(SplitBlockBloomFilter::union(std::iter::empty()).is_none());
    }

    #[test]
    fn capacity_is_capped() {
        assert!(SplitBlockBloomFilter::try_with_capacity(usize::MAX, 16).is_none());
        assert!(SplitBlockBloomFilter::try_with_capacity(1 << 40, 16).is_none());
        let empty = SplitBlockBloomFilter::try_with_capacity(0, 16).unwrap();
        assert_eq!(empty.num_blocks(), 1);
        assert!(!empty.contains(0));
    }

    #[test]
    fn expr_matches_build_keys_on_multiple_columns() -> Result<()> {
        let build_ids: ArrayRef = Arc::new(Int64Array::from(vec![1, 2, 3, 4]));
        let build_names: ArrayRef = Arc::new(StringArray::from(vec!["a", "b", "c", "d"]));
        let bloom = Arc::new(bloom_of(&[build_ids, build_names]));

        let schema = Arc::new(Schema::new(vec![
            Field::new("id", DataType::Int64, true),
            Field::new("name", DataType::Utf8, true),
        ]));
        let expr = Arc::new(BloomFilterExpr::new(
            vec![
                Arc::new(Column::new("id", 0)) as _,
                Arc::new(Column::new("name", 1)) as _,
            ],
            HASH_JOIN_SEED,
            Arc::clone(&bloom),
            None,
        ));
        let batch = RecordBatch::try_new(
            Arc::clone(&schema),
            vec![
                Arc::new(Int64Array::from(vec![
                    Some(1),
                    Some(2),
                    Some(3),
                    Some(4),
                    None,
                ])),
                Arc::new(StringArray::from(vec![
                    Some("a"),
                    Some("x"),
                    Some("c"),
                    Some("d"),
                    Some("a"),
                ])),
            ],
        )?;
        let result = expr.evaluate(&batch)?.into_array(batch.num_rows())?;
        let result = result.as_any().downcast_ref::<BooleanArray>().unwrap();
        // Every build key passes; rows that cannot match may or may not
        // (false positives), here they do not.
        assert_eq!(result.null_count(), 0);
        assert!(result.value(0) && result.value(2) && result.value(3));
        assert!(!result.value(1) && !result.value(4));

        // Remapped children keep sharing the same filter.
        let remapped = Arc::clone(&expr).with_new_children(vec![
            Arc::new(Column::new("id", 1)) as _,
            Arc::new(Column::new("name", 0)) as _,
        ])?;
        let remapped = remapped.downcast_ref::<BloomFilterExpr>().unwrap();
        assert!(Arc::ptr_eq(remapped.bloom(), &bloom));
        assert_eq!(expr.to_string(), "bloom_filter");
        Ok(())
    }
}
