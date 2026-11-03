// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: See the NOTICE file distributed with this work for additional information regarding copyright ownership
// SPDX-FileContributor: Licensed to the Apache Software Foundation (ASF) under one or more contributor license agreements

//! Hash join build side: inserting build rows into a `JoinHashMap`.
//!
//! Mirrors the `HashJoinExec` build loop: batches of 8192 rows are inserted
//! last batch first through `update_hash`. Cases vary the build size (so the
//! table fits in, or spills out of, the CPU caches), the number of duplicate
//! keys, and the fraction of NULL keys. Key hashing is included in the timing.

use std::hint::black_box;
use std::sync::Arc;

use arrow::array::{ArrayRef, Int64Array, RecordBatch};
use arrow::datatypes::{DataType, Field, Schema};
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use datafusion_common::NullEquality;
use datafusion_common::hash_utils::RandomState;
use datafusion_physical_expr::PhysicalExprRef;
use datafusion_physical_expr::expressions::col;
use datafusion_physical_plan::joins::join_hash_map::{JoinHashMapType, JoinHashMapU32};
use datafusion_physical_plan::joins::utils::update_hash;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

const BATCH_ROWS: usize = 8192;

/// Builds `rows` keys in batches. Each distinct key appears `dups` times, and
/// a `null_pct` percent of the rows are NULL.
fn build_batches(rows: usize, dups: usize, null_pct: u32) -> Vec<RecordBatch> {
    let schema = Arc::new(Schema::new(vec![Field::new("key", DataType::Int64, true)]));
    let mut rng = StdRng::seed_from_u64(42);
    let distinct = (rows / dups).max(1) as i64;
    let keys: Vec<Option<i64>> = (0..rows)
        .map(|_| {
            (rng.random_range(0..100) >= null_pct)
                .then(|| rng.random_range(0..distinct) * 7919)
        })
        .collect();
    keys.chunks(BATCH_ROWS)
        .map(|chunk| {
            let array: ArrayRef = Arc::new(Int64Array::from(chunk.to_vec()));
            RecordBatch::try_new(Arc::clone(&schema), vec![array]).unwrap()
        })
        .collect()
}

fn build(on: &[PhysicalExprRef], batches: &[RecordBatch], rows: usize) -> JoinHashMapU32 {
    let random_state = RandomState::with_seed(0);
    let mut map = JoinHashMapU32::with_capacity(rows);
    let mut hashes = Vec::with_capacity(BATCH_ROWS);
    let mut offset = rows;
    for batch in batches.iter().rev() {
        offset -= batch.num_rows();
        hashes.clear();
        hashes.resize(batch.num_rows(), 0);
        update_hash(
            on,
            batch,
            &mut map,
            offset,
            &random_state,
            &mut hashes,
            0,
            true,
            NullEquality::NullEqualsNothing,
        )
        .unwrap();
    }
    map
}

fn bench_build(c: &mut Criterion) {
    let mut group = c.benchmark_group("hash_join_build");
    for rows in [65_536, 1_048_576, 8_388_608] {
        for (dups, null_pct) in [(1, 0), (4, 0), (1, 10)] {
            let batches = build_batches(rows, dups, null_pct);
            let on = vec![col("key", &batches[0].schema()).unwrap()];
            group.throughput(Throughput::Elements(rows as u64));
            group.sample_size(if rows > 1_000_000 { 10 } else { 50 });
            let id = format!("rows={rows}/dups={dups}/nulls={null_pct}%");
            group.bench_function(BenchmarkId::from_parameter(id), |b| {
                b.iter(|| black_box(build(&on, &batches, rows).len()))
            });
        }
    }
    group.finish();
}

criterion_group!(benches, bench_build);
criterion_main!(benches);
