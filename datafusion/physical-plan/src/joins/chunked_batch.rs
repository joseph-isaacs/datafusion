// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: See the NOTICE file distributed with this work for additional information regarding copyright ownership
// SPDX-FileContributor: Licensed to the Apache Software Foundation (ASF) under one or more contributor license agreements

//! [`ChunkedBatch`]: the rows of several record batches, addressed as if they
//! were concatenated, without copying them into a single batch.

use std::sync::{Arc, OnceLock};

use arrow::array::{
    Array, ArrayRef, ArrowPrimitiveType, AsArray, GenericByteViewArray, PrimitiveArray,
    RecordBatch, UInt64Array, downcast_primitive_array, new_null_array,
};
use arrow::buffer::{BooleanBuffer, Buffer, NullBuffer, ScalarBuffer};
use arrow::compute::{interleave, take};
use arrow::datatypes::{
    BinaryViewType, ByteViewType, DataType, SchemaRef, StringViewType,
};
use datafusion_common::Result;

/// Rows per block of [`ChunkedBatch::block_chunks`], as a power of two.
const BLOCK_SHIFT: u32 = 10;

/// Row `i` of a [`ChunkedBatch`] is row `i - starts[c]` of chunk `c`, where
/// chunk `c` is the last one with `starts[c] <= i`: the rows are numbered as
/// if the chunks were concatenated.
///
/// Gathering rows by index (see [`Self::take_column`]) reads them straight
/// from their chunk, so the build side of a hash join can be indexed like a
/// single batch without first copying every build row into one, as
/// `concat_batches` would.
#[derive(Debug, Clone)]
pub(crate) struct ChunkedBatch {
    schema: SchemaRef,
    /// The non-empty input batches.
    chunks: Vec<RecordBatch>,
    /// `starts[c]` is the index of the first row of chunk `c`, and
    /// `starts[chunks.len()]` the number of rows.
    starts: Vec<u64>,
    /// `block_chunks[b]` is the first chunk holding a row of block `b`, rows
    /// `b << BLOCK_SHIFT..(b + 1) << BLOCK_SHIFT`.
    block_chunks: Vec<u32>,
    /// Per column of a view type, created on first use: the data buffers of
    /// every chunk, and the position of each chunk's first buffer in them.
    view_buffers: Arc<[OnceLock<ViewBuffers>]>,
}

/// See [`ChunkedBatch::view_buffers`].
#[derive(Debug)]
struct ViewBuffers {
    buffers: Arc<[Buffer]>,
    starts: Vec<u128>,
}

impl ChunkedBatch {
    /// Keeps `batches` as chunks. They must all have schema `schema`.
    pub(crate) fn new(schema: SchemaRef, batches: Vec<RecordBatch>) -> Self {
        let chunks: Vec<RecordBatch> =
            batches.into_iter().filter(|b| b.num_rows() > 0).collect();
        let mut starts = Vec::with_capacity(chunks.len() + 1);
        let mut num_rows = 0u64;
        for chunk in &chunks {
            starts.push(num_rows);
            num_rows += chunk.num_rows() as u64;
        }
        starts.push(num_rows);

        let num_blocks = num_rows.div_ceil(1 << BLOCK_SHIFT) as usize;
        let mut block_chunks = Vec::with_capacity(num_blocks);
        let mut chunk = 0;
        for block in 0..num_blocks as u64 {
            let first_row = block << BLOCK_SHIFT;
            while starts[chunk + 1] <= first_row {
                chunk += 1;
            }
            block_chunks.push(chunk as u32);
        }

        let view_buffers = (0..schema.fields().len())
            .map(|_| OnceLock::new())
            .collect();
        Self {
            schema,
            chunks,
            starts,
            block_chunks,
            view_buffers,
        }
    }

    /// Whether gathering from `batches` as chunks avoids a copy at a small
    /// cost per gathered row: there is more than one batch and they are not
    /// too small on average, so that locating a row stays cheap.
    pub(crate) fn worth_chunking(batches: &[RecordBatch]) -> bool {
        let num_rows: usize = batches.iter().map(|b| b.num_rows()).sum();
        batches.len() > 1 && num_rows >= batches.len() << BLOCK_SHIFT
    }

    pub(crate) fn schema(&self) -> &SchemaRef {
        &self.schema
    }

    pub(crate) fn num_rows(&self) -> usize {
        *self.starts.last().unwrap_or(&0) as usize
    }

    /// The single batch holding every row, unless the rows are split over
    /// several chunks.
    pub(crate) fn as_single(&self) -> Option<RecordBatch> {
        match self.chunks.as_slice() {
            [] => Some(RecordBatch::new_empty(Arc::clone(&self.schema))),
            [batch] => Some(batch.clone()),
            _ => None,
        }
    }

    /// The chunks, in row order.
    pub(crate) fn chunks(&self) -> &[RecordBatch] {
        &self.chunks
    }

    /// Returns `(chunk, row in chunk)` of row `index < num_rows()`.
    #[inline(always)]
    fn locate(&self, index: u64) -> (usize, usize) {
        let mut chunk = self.block_chunks[(index >> BLOCK_SHIFT) as usize] as usize;
        while self.starts[chunk + 1] <= index {
            chunk += 1;
        }
        (chunk, (index - self.starts[chunk]) as usize)
    }

    /// Gathers column `column` at `indices`, like [`take`] on the column of
    /// the concatenated batch. NULL indices produce NULLs.
    pub(crate) fn take_column(
        &self,
        column: usize,
        indices: &UInt64Array,
    ) -> Result<ArrayRef> {
        if let [chunk] = self.chunks.as_slice() {
            return Ok(take(chunk.column(column), indices, None)?);
        }
        let data_type = self.schema.field(column).data_type();
        if self.chunks.is_empty() || indices.null_count() == indices.len() {
            return Ok(new_null_array(data_type, indices.len()));
        }
        let arrays: Vec<&ArrayRef> =
            self.chunks.iter().map(|c| c.column(column)).collect();

        let array: ArrayRef = match data_type {
            DataType::Utf8View => {
                Arc::new(self.take_views::<StringViewType>(column, &arrays, indices))
            }
            DataType::BinaryView => {
                Arc::new(self.take_views::<BinaryViewType>(column, &arrays, indices))
            }
            _ => {
                let first = arrays[0];
                downcast_primitive_array!(
                    first => Arc::new(self.take_primitive(first, &arrays, indices)),
                    _ => self.take_interleave(data_type, &arrays, indices)?
                )
            }
        };
        Ok(array)
    }

    /// The validity of the gathered rows, `None` if all are valid.
    fn gather_nulls(
        &self,
        arrays: &[&ArrayRef],
        indices: &UInt64Array,
    ) -> Option<NullBuffer> {
        let chunk_nulls: Vec<Option<&NullBuffer>> =
            arrays.iter().map(|a| a.nulls()).collect();
        let index_nulls = indices.nulls();
        if index_nulls.is_none() && chunk_nulls.iter().all(Option::is_none) {
            return None;
        }
        let values = indices.values();
        let buffer = BooleanBuffer::collect_bool(indices.len(), |i| {
            if index_nulls.is_some_and(|n| n.is_null(i)) {
                return false;
            }
            let (chunk, row) = self.locate(values[i]);
            chunk_nulls[chunk].is_none_or(|n| n.is_valid(row))
        });
        Some(NullBuffer::new(buffer)).filter(|n| n.null_count() > 0)
    }

    /// Gathers the native values at `indices`, `T::Native::default()` for
    /// NULL indices.
    #[inline(always)]
    fn gather_values<T: Copy + Default>(
        &self,
        values: &[&[T]],
        indices: &UInt64Array,
    ) -> Vec<T> {
        let num_rows = self.num_rows() as u64;
        let index_nulls = indices.nulls();
        let mut out = Vec::with_capacity(indices.len());
        out.extend(indices.values().iter().enumerate().map(|(i, &index)| {
            // NULL indices may hold any value
            if index >= num_rows || index_nulls.is_some_and(|n| n.is_null(i)) {
                return T::default();
            }
            let (chunk, row) = self.locate(index);
            values[chunk][row]
        }));
        out
    }

    fn take_primitive<T: ArrowPrimitiveType>(
        &self,
        first: &PrimitiveArray<T>,
        arrays: &[&ArrayRef],
        indices: &UInt64Array,
    ) -> PrimitiveArray<T> {
        let values: Vec<&[T::Native]> = arrays
            .iter()
            .map(|a| a.as_primitive::<T>().values().as_ref())
            .collect();
        let out = self.gather_values(&values, indices);
        let nulls = self.gather_nulls(arrays, indices);
        PrimitiveArray::<T>::new(ScalarBuffer::from(out), nulls)
            .with_data_type(first.data_type().clone())
    }

    fn take_views<T: ByteViewType>(
        &self,
        column: usize,
        arrays: &[&ArrayRef],
        indices: &UInt64Array,
    ) -> GenericByteViewArray<T> {
        let nulls = self.gather_nulls(arrays, indices);
        let arrays: Vec<&GenericByteViewArray<T>> =
            arrays.iter().map(|a| a.as_byte_view::<T>()).collect();
        // The data buffers of chunk `c` are numbered from `buffer_starts[c]`
        // in the output.
        let ViewBuffers {
            buffers,
            starts: buffer_starts,
        } = self.view_buffers[column].get_or_init(|| {
            let mut starts = Vec::with_capacity(arrays.len());
            let mut buffers: Vec<Buffer> = Vec::new();
            for array in &arrays {
                starts.push(buffers.len() as u128);
                buffers.extend(array.data_buffers().iter().cloned());
            }
            ViewBuffers {
                buffers: buffers.into(),
                starts,
            }
        });
        let num_rows = self.num_rows() as u64;
        let index_nulls = indices.nulls();
        let mut views = Vec::with_capacity(indices.len());
        views.extend(indices.values().iter().enumerate().map(|(i, &index)| {
            if index >= num_rows || index_nulls.is_some_and(|n| n.is_null(i)) {
                return 0u128;
            }
            let (chunk, row) = self.locate(index);
            let view = arrays[chunk].views()[row];
            if (view as u32) > 12 {
                // Bits 64..96 hold the buffer index of a non-inlined view.
                view + (buffer_starts[chunk] << 64)
            } else {
                view
            }
        }));
        // SAFETY: every view is either NULL (zeroed, a valid empty view) or
        // a view of an input array whose buffer index was shifted to the
        // position of that array's buffers in `buffers`.
        unsafe {
            GenericByteViewArray::<T>::new_unchecked(
                ScalarBuffer::from(views),
                Arc::clone(buffers),
                nulls,
            )
        }
    }

    fn take_interleave(
        &self,
        data_type: &DataType,
        arrays: &[&ArrayRef],
        indices: &UInt64Array,
    ) -> Result<ArrayRef> {
        // NULL indices take the single row of an extra all-NULL source.
        let null_source = new_null_array(data_type, 1);
        let mut sources: Vec<&dyn Array> = arrays.iter().map(|a| a.as_ref()).collect();
        let null_chunk = sources.len();
        sources.push(null_source.as_ref());
        let num_rows = self.num_rows() as u64;
        let index_nulls = indices.nulls();
        let positions: Vec<(usize, usize)> = indices
            .values()
            .iter()
            .enumerate()
            .map(|(i, &index)| {
                if index >= num_rows || index_nulls.is_some_and(|n| n.is_null(i)) {
                    (null_chunk, 0)
                } else {
                    self.locate(index)
                }
            })
            .collect();
        Ok(interleave(&sources, &positions)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use arrow::array::{
        BooleanArray, Decimal128Array, Int32Array, StringArray, StringViewArray,
    };
    use arrow::compute::concat_batches;
    use arrow::datatypes::{Field, Schema};

    fn batch(schema: &SchemaRef, start: i32, len: i32) -> RecordBatch {
        let ints = Int32Array::from_iter(
            (start..start + len).map(|v| (v % 7 != 0).then_some(v)),
        );
        let strs =
            StringViewArray::from_iter((start..start + len).map(|v| {
                (v % 5 != 0).then(|| format!("a string longer than twelve {v}"))
            }));
        let short = StringViewArray::from_iter_values(
            (start..start + len).map(|v| format!("{v}")),
        );
        let utf8 =
            StringArray::from_iter_values((start..start + len).map(|v| format!("s{v}")));
        let bools =
            BooleanArray::from_iter((start..start + len).map(|v| Some(v % 2 == 0)));
        let decimals =
            Decimal128Array::from_iter_values((start..start + len).map(i128::from))
                .with_precision_and_scale(20, 2)
                .unwrap();
        RecordBatch::try_new(
            Arc::clone(schema),
            vec![
                Arc::new(ints),
                Arc::new(strs),
                Arc::new(short),
                Arc::new(utf8),
                Arc::new(bools),
                Arc::new(decimals),
            ],
        )
        .unwrap()
    }

    #[test]
    fn take_column_matches_concat() -> Result<()> {
        let schema = Arc::new(Schema::new(vec![
            Field::new("i", DataType::Int32, true),
            Field::new("s", DataType::Utf8View, true),
            Field::new("short", DataType::Utf8View, false),
            Field::new("u", DataType::Utf8, false),
            Field::new("b", DataType::Boolean, true),
            Field::new("d", DataType::Decimal128(20, 2), false),
        ]));
        // Chunks smaller and larger than a block, and an empty one.
        let lens = [3000, 5, 0, 1024, 2500, 1];
        let mut start = 0;
        let batches: Vec<RecordBatch> = lens
            .iter()
            .map(|&len| {
                let b = batch(&schema, start, len);
                start += len;
                b
            })
            .collect();
        let num_rows = start as u64;
        let chunked = ChunkedBatch::new(Arc::clone(&schema), batches.clone());
        let single = concat_batches(&schema, &batches)?;
        assert_eq!(chunked.num_rows(), single.num_rows());
        assert!(chunked.as_single().is_none());

        let indices = UInt64Array::from_iter((0..3 * num_rows).map(|i| {
            let index = (i * 7919) % num_rows;
            (i % 13 != 0).then_some(index)
        }));
        for column in 0..schema.fields().len() {
            let expected = take(single.column(column), &indices, None)?;
            let actual = chunked.take_column(column, &indices)?;
            assert_eq!(&expected, &actual, "column {column}");
        }
        // All-NULL indices.
        let nulls = UInt64Array::new_null(4);
        for column in 0..schema.fields().len() {
            let expected = take(single.column(column), &nulls, None)?;
            assert_eq!(&expected, &chunked.take_column(column, &nulls)?);
        }
        Ok(())
    }

    #[test]
    fn worth_chunking() {
        let schema = Arc::new(Schema::new(vec![
            Field::new("i", DataType::Int32, true),
            Field::new("s", DataType::Utf8View, true),
            Field::new("short", DataType::Utf8View, false),
            Field::new("u", DataType::Utf8, false),
            Field::new("b", DataType::Boolean, true),
            Field::new("d", DataType::Decimal128(20, 2), false),
        ]));
        let big = batch(&schema, 0, 2048);
        let small = batch(&schema, 0, 10);
        let small_empty = |schema: &SchemaRef| batch(schema, 0, 0);
        assert!(!ChunkedBatch::worth_chunking(std::slice::from_ref(&big)));
        assert!(ChunkedBatch::worth_chunking(&[big.clone(), big.clone()]));
        assert!(!ChunkedBatch::worth_chunking(&[small.clone(), small]));
        let single = ChunkedBatch::new(
            Arc::clone(&schema),
            vec![big.clone(), small_empty(&schema)],
        );
        assert_eq!(single.as_single().unwrap().num_rows(), 2048);
    }
}
