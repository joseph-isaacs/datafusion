// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: See the NOTICE file distributed with this work for additional information regarding copyright ownership
// SPDX-FileContributor: Licensed to the Apache Software Foundation (ASF) under one or more contributor license agreements

//! Criterion benchmarks for inner hash joins on integer keys that do not
//! qualify for the `ArrayMap`, with and without
//! `datafusion.execution.hash_join_key_packing`.
//!
//! - `two_i32`: two Int32 keys, unique build keys spread over a sparse
//!   `1000 x 1000` grid (like TPC-H `(ps_partkey, ps_suppkey)`).
//! - `one_i64_sparse`: one sparse Int64 key (like TPC-H `o_orderkey`).
//!
//! Every benchmark builds 1M rows and probes 4M rows, half of which match.

use std::sync::Arc;

use arrow::array::{Int32Array, Int64Array, RecordBatch};
use arrow::datatypes::{DataType, Field, Schema, SchemaRef};
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use datafusion_common::{JoinType, NullEquality};
use datafusion_execution::TaskContext;
use datafusion_execution::config::SessionConfig;
use datafusion_physical_expr::expressions::col;
use datafusion_physical_plan::joins::{HashJoinExec, PartitionMode, utils::JoinOn};
use datafusion_physical_plan::test::TestMemoryExec;
use datafusion_physical_plan::{ExecutionPlan, collect};
use tokio::runtime::Runtime;

const BUILD_ROWS: usize = 1_000_000;
const PROBE_ROWS: usize = 4_000_000;
const BATCH_SIZE: usize = 8192;

fn split(batch: &RecordBatch) -> Vec<RecordBatch> {
    (0..batch.num_rows())
        .step_by(BATCH_SIZE)
        .map(|offset| batch.slice(offset, BATCH_SIZE.min(batch.num_rows() - offset)))
        .collect()
}

/// `(a, b)` keys of row `i`: build key `i` (of `BUILD_ROWS`) maps to a unique
/// point on a sparse grid. Keys at or past `BUILD_ROWS` never match but stay
/// within the build key range, as most probe misses do.
fn two_i32_key(i: usize) -> (i32, i32) {
    if i < BUILD_ROWS {
        ((i % 1000) as i32 * 7, (i / 1000) as i32 * 13)
    } else {
        let i = i - BUILD_ROWS;
        ((i % 999) as i32 * 7 + 3, (i / 1000) as i32 * 13)
    }
}

fn two_i32_batches(keys: impl Iterator<Item = usize>) -> (SchemaRef, Vec<RecordBatch>) {
    let schema = Arc::new(Schema::new(vec![
        Field::new("a", DataType::Int32, false),
        Field::new("b", DataType::Int32, false),
        Field::new("payload", DataType::Int64, false),
    ]));
    let (a, b): (Vec<i32>, Vec<i32>) = keys.map(two_i32_key).unzip();
    let payload = Int64Array::from_iter_values(0..a.len() as i64);
    let batch = RecordBatch::try_new(
        Arc::clone(&schema),
        vec![
            Arc::new(Int32Array::from(a)),
            Arc::new(Int32Array::from(b)),
            Arc::new(payload),
        ],
    )
    .unwrap();
    (schema, split(&batch))
}

fn one_i64_batches(keys: impl Iterator<Item = usize>) -> (SchemaRef, Vec<RecordBatch>) {
    let schema = Arc::new(Schema::new(vec![
        Field::new("a", DataType::Int64, false),
        Field::new("payload", DataType::Int64, false),
    ]));
    // Keys 37 apart: far too sparse for an `ArrayMap`. Misses stay in range.
    let a = Int64Array::from_iter_values(keys.map(|i| {
        if i < BUILD_ROWS {
            i as i64 * 37
        } else {
            (i - BUILD_ROWS) as i64 * 37 + 5
        }
    }));
    let payload = Int64Array::from_iter_values(0..a.len() as i64);
    let batch =
        RecordBatch::try_new(Arc::clone(&schema), vec![Arc::new(a), Arc::new(payload)])
            .unwrap();
    (schema, split(&batch))
}

/// Probe keys: every other row hits a build key, the rest miss.
fn probe_keys() -> impl Iterator<Item = usize> {
    (0..PROBE_ROWS).map(|i| {
        let key = (i / 2).wrapping_mul(7919) % BUILD_ROWS;
        if i % 2 == 0 { key } else { BUILD_ROWS + key }
    })
}

fn do_join(
    schema: &SchemaRef,
    build: &[RecordBatch],
    probe: &[RecordBatch],
    keys: &[&str],
    key_packing: bool,
    rt: &Runtime,
) -> usize {
    let left = TestMemoryExec::try_new_exec(&[build.to_vec()], Arc::clone(schema), None)
        .unwrap();
    let right = TestMemoryExec::try_new_exec(&[probe.to_vec()], Arc::clone(schema), None)
        .unwrap();
    let on: JoinOn = keys
        .iter()
        .map(|k| {
            (
                col(k, &left.schema()).unwrap(),
                col(k, &right.schema()).unwrap(),
            )
        })
        .collect();
    let join = HashJoinExec::try_new(
        left,
        right,
        on,
        None,
        &JoinType::Inner,
        None,
        PartitionMode::CollectLeft,
        NullEquality::NullEqualsNothing,
        false,
    )
    .unwrap();

    let mut config = SessionConfig::new().with_batch_size(BATCH_SIZE);
    config.options_mut().execution.hash_join_key_packing = key_packing;
    let task_ctx = Arc::new(TaskContext::default().with_session_config(config));
    rt.block_on(async {
        let batches = collect(Arc::new(join), task_ctx).await.unwrap();
        batches.iter().map(|b| b.num_rows()).sum()
    })
}

fn bench_hash_join_multi_key(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let mut group = c.benchmark_group("hash_join_multi_key");
    group.sample_size(10);

    let (schema, build) = two_i32_batches(0..BUILD_ROWS);
    let (_, probe) = two_i32_batches(probe_keys());
    for key_packing in [false, true] {
        let name = if key_packing { "packed" } else { "hashed" };
        group.bench_function(BenchmarkId::new("two_i32", name), |b| {
            b.iter(|| {
                let rows =
                    do_join(&schema, &build, &probe, &["a", "b"], key_packing, &rt);
                assert_eq!(rows, PROBE_ROWS / 2);
            })
        });
    }

    let (schema, build) = one_i64_batches(0..BUILD_ROWS);
    let (_, probe) = one_i64_batches(probe_keys());
    for key_packing in [false, true] {
        let name = if key_packing { "packed" } else { "hashed" };
        group.bench_function(BenchmarkId::new("one_i64_sparse", name), |b| {
            b.iter(|| {
                let rows = do_join(&schema, &build, &probe, &["a"], key_packing, &rt);
                assert_eq!(rows, PROBE_ROWS / 2);
            })
        });
    }
    group.finish();
}

criterion_group!(benches, bench_hash_join_multi_key);
criterion_main!(benches);
