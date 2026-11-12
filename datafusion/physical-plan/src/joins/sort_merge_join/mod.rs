// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: See the NOTICE file distributed with this work for additional information regarding copyright ownership
// SPDX-FileContributor: Licensed to the Apache Software Foundation (ASF) under one or more contributor license agreements

//! Sort Merge Join Execution Plan Operator

pub use exec::SortMergeJoinExec;

pub(crate) mod bitwise_stream;
mod exec;
mod filter;
pub(crate) mod materializing_stream;
mod metrics;

#[cfg(test)]
mod tests;

use std::sync::Arc;

use crate::SendableRecordBatchStream;
use crate::common::spawn_buffered;
use crate::stream::{EmptyRecordBatchStream, RecordBatchStreamAdapter};
use datafusion_execution::memory_pool::MemoryReservation;
use futures::StreamExt;

/// Moves the polling of `stream` into its own task, up to
/// [`PREFETCH_BATCHES`] batches ahead of the join (as
/// `SortPreservingMergeExec` does for its inputs, one batch ahead). A merge join
/// alternates between its two inputs, so polling them inline leaves each
/// input's scan and decode idle while the other's runs and while the merge
/// itself does; buffered, they run in parallel with both.
///
/// The streams call this lazily, on the second input only once the first
/// has produced a row, so a join that ends early still never starts reading
/// an input it does not need. A no-op outside a multi-threaded runtime.
///
/// Batches waiting in the read-ahead are counted in the join's memory
/// reservation (see [`prefetch`]); the depth is still kept small (each slot
/// holds one input batch). Four slots, over one, are 18-37% faster on the sorted
/// time-series benchmark (`dfbench smj-ts`), whose Parquet inputs decode in
/// bursts that a single slot cannot absorb, with no measurable change in
/// peak RSS.
const PREFETCH_BATCHES: usize = 4;

pub(super) fn prefetch(
    stream: &mut SendableRecordBatchStream,
    reservation: &MemoryReservation,
) {
    let schema = stream.schema();
    let input = std::mem::replace(
        stream,
        Box::pin(EmptyRecordBatchStream::new(Arc::clone(&schema))),
    );
    // Each batch read ahead is reserved from when it is produced until the
    // join takes it, so the pool sees the memory the read-ahead holds.
    // Dropping the stream releases whatever is still held.
    let held = Arc::new(reservation.new_empty());
    let produced = Arc::clone(&held);
    let input = input.inspect(move |batch| {
        if let Ok(batch) = batch {
            produced.grow(batch.get_array_memory_size());
        }
    });
    let buffered = spawn_buffered(
        Box::pin(RecordBatchStreamAdapter::new(Arc::clone(&schema), input)),
        PREFETCH_BATCHES,
    );
    let taken = buffered.inspect(move |batch| {
        if let Ok(batch) = batch {
            held.shrink(batch.get_array_memory_size());
        }
    });
    *stream = Box::pin(RecordBatchStreamAdapter::new(schema, taken));
}
