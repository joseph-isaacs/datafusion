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

use crate::SendableRecordBatchStream;
use crate::common::spawn_buffered;
use crate::stream::EmptyRecordBatchStream;

/// Moves the polling of `stream` into its own task, one batch ahead of the
/// join (as `SortPreservingMergeExec` does for its inputs). A merge join
/// alternates between its two inputs, so polling them inline leaves each
/// input's scan and decode idle while the other's runs and while the merge
/// itself does; buffered, they run in parallel with both.
///
/// The streams call this lazily, on the second input only once the first
/// has produced a row, so a join that ends early still never starts reading
/// an input it does not need. A no-op outside a multi-threaded runtime.
pub(super) fn prefetch(stream: &mut SendableRecordBatchStream) {
    let schema = stream.schema();
    let input = std::mem::replace(stream, Box::pin(EmptyRecordBatchStream::new(schema)));
    *stream = spawn_buffered(input, 1);
}
