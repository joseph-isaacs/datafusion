// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: See the NOTICE file distributed with this work for additional information regarding copyright ownership
// SPDX-FileContributor: Licensed to the Apache Software Foundation (ASF) under one or more contributor license agreements

//! Hash join probe side: looking up probe rows in the build-side map.
//!
//! `hash_join_probe` probes a `JoinHashMap` with precomputed hashes, the way
//! `HashJoinStream` calls `get_matched_indices_with_limit_offset` per probe
//! batch (key verification is not included). `array_map_build`,
//! `array_map_probe` and `array_map_contains` cover the `ArrayMap` used for
//! dense integer keys. Cases vary the build size, so the map fits in, or spills
//! out of, the CPU caches, and whether build keys are unique or repeated. Every
//! probe row matches, in random order.

use std::hint::black_box;
use std::sync::Arc;

use arrow::array::{ArrayRef, Int64Array, RecordBatch};
use arrow::datatypes::{DataType, Field, Schema};
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use datafusion_common::NullEquality;
use datafusion_common::hash_utils::{RandomState, create_hashes};
use datafusion_physical_expr::expressions::col;
use datafusion_physical_plan::joins::array_map::ArrayMap;
use datafusion_physical_plan::joins::join_hash_map::{
    JoinHashMapType, JoinHashMapU32, ProbeOffset,
};
use datafusion_physical_plan::joins::utils::update_hash;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};

const BATCH_ROWS: usize = 8192;
/// Probe rows per iteration, the same for every build size.
const PROBE_ROWS: usize = 1 << 20;
const BUILD_ROWS: [usize; 3] = [65_536, 1_048_576, 8_388_608];

/// Build keys: `rows / dups` distinct keys, each repeated `dups` times, in
/// random order. Keys are multiples of 4, so an `ArrayMap` over them has a
/// density of `dups / 4`.
fn build_keys(rows: usize, dups: usize) -> Vec<i64> {
    let mut rng = StdRng::seed_from_u64(42);
    let distinct = rows / dups;
    let mut keys: Vec<i64> = (0..rows).map(|i| (i % distinct) as i64 * 4).collect();
    keys.shuffle(&mut rng);
    keys
}

/// `PROBE_ROWS` probe keys drawn from the build keys, in batches.
fn probe_batches(build_keys: &[i64]) -> Vec<ArrayRef> {
    let mut rng = StdRng::seed_from_u64(7);
    (0..PROBE_ROWS / BATCH_ROWS)
        .map(|_| {
            let keys: Vec<i64> = (0..BATCH_ROWS)
                .map(|_| build_keys[rng.random_range(0..build_keys.len())])
                .collect();
            Arc::new(Int64Array::from(keys)) as ArrayRef
        })
        .collect()
}

fn hash(array: &ArrayRef, random_state: &RandomState) -> Vec<u64> {
    let mut hashes = vec![0; array.len()];
    create_hashes([array], random_state, &mut hashes).unwrap();
    hashes
}

/// Mirrors the `HashJoinExec` build: batches inserted last batch first.
fn build_hash_map(keys: &[i64], random_state: &RandomState) -> JoinHashMapU32 {
    let schema = Arc::new(Schema::new(vec![Field::new("key", DataType::Int64, true)]));
    let on = vec![col("key", &schema).unwrap()];
    let mut map = JoinHashMapU32::with_capacity(keys.len());
    let mut hashes = vec![0; BATCH_ROWS];
    let chunks: Vec<_> = keys.chunks(BATCH_ROWS).collect();
    let mut offset = keys.len();
    for chunk in chunks.iter().rev() {
        offset -= chunk.len();
        let array: ArrayRef = Arc::new(Int64Array::from(chunk.to_vec()));
        let batch = RecordBatch::try_new(Arc::clone(&schema), vec![array]).unwrap();
        update_hash(
            &on,
            &batch,
            &mut map,
            offset,
            random_state,
            &mut hashes[..chunk.len()],
            0,
            true,
            NullEquality::NullEqualsNothing,
        )
        .unwrap();
    }
    map
}

fn bench_hash_map_probe(c: &mut Criterion) {
    let mut group = c.benchmark_group("hash_join_probe");
    group.throughput(Throughput::Elements(PROBE_ROWS as u64));
    let random_state = RandomState::with_seed(0);
    for rows in BUILD_ROWS {
        for dups in [1, 4] {
            let keys = build_keys(rows, dups);
            let map = build_hash_map(&keys, &random_state);
            let probe_hashes: Vec<Vec<u64>> = probe_batches(&keys)
                .iter()
                .map(|batch| hash(batch, &random_state))
                .collect();
            drop(keys);
            group.sample_size(if rows > 1_000_000 { 10 } else { 30 });
            let id = format!("rows={rows}/dups={dups}");
            let mut input_indices = Vec::with_capacity(BATCH_ROWS);
            let mut match_indices = Vec::with_capacity(BATCH_ROWS);
            group.bench_function(BenchmarkId::from_parameter(id), |b| {
                b.iter(|| {
                    let mut matches = 0;
                    for hashes in &probe_hashes {
                        let mut offset = Some(ProbeOffset::AtRow { row: 0 });
                        while let Some(current) = offset {
                            offset = map.get_matched_indices_with_limit_offset(
                                hashes,
                                None,
                                BATCH_ROWS,
                                current,
                                &mut input_indices,
                                &mut match_indices,
                            );
                            matches += match_indices.len();
                        }
                    }
                    black_box(matches)
                })
            });
        }
    }
    group.finish();
}

fn bench_array_map(c: &mut Criterion) {
    for rows in BUILD_ROWS {
        for dups in [1, 4] {
            let keys = build_keys(rows, dups);
            let max = keys.iter().copied().max().unwrap() as u64;
            let probes = probe_batches(&keys);
            let build: ArrayRef = Arc::new(Int64Array::from(keys));
            let id = format!("rows={rows}/dups={dups}");
            let sample_size = if rows > 1_000_000 { 10 } else { 30 };

            let mut group = c.benchmark_group("array_map_build");
            group.throughput(Throughput::Elements(rows as u64));
            group.sample_size(sample_size);
            group.bench_function(BenchmarkId::from_parameter(&id), |b| {
                b.iter(|| {
                    black_box(
                        ArrayMap::try_new(&build, 0, max)
                            .unwrap()
                            .num_of_distinct_key(),
                    )
                })
            });
            group.finish();

            let map = ArrayMap::try_new(&build, 0, max).unwrap();
            drop(build);

            let mut group = c.benchmark_group("array_map_probe");
            group.throughput(Throughput::Elements(PROBE_ROWS as u64));
            group.sample_size(sample_size);
            let mut probe_indices = Vec::with_capacity(BATCH_ROWS);
            let mut build_indices = Vec::with_capacity(BATCH_ROWS);
            group.bench_function(BenchmarkId::from_parameter(&id), |b| {
                b.iter(|| {
                    let mut matches = 0;
                    for probe in &probes {
                        let probe = std::slice::from_ref(probe);
                        let mut offset = Some(ProbeOffset::AtRow { row: 0 });
                        while let Some(current) = offset {
                            offset = map
                                .get_matched_indices_with_limit_offset(
                                    probe,
                                    BATCH_ROWS,
                                    current,
                                    &mut probe_indices,
                                    &mut build_indices,
                                )
                                .unwrap();
                            matches += build_indices.len();
                        }
                    }
                    black_box(matches)
                })
            });
            group.finish();

            let mut group = c.benchmark_group("array_map_contains");
            group.throughput(Throughput::Elements(PROBE_ROWS as u64));
            group.sample_size(sample_size);
            group.bench_function(BenchmarkId::from_parameter(&id), |b| {
                b.iter(|| {
                    let mut hits = 0;
                    for probe in &probes {
                        let found =
                            map.contain_keys(std::slice::from_ref(probe)).unwrap();
                        hits += found.true_count();
                    }
                    black_box(hits)
                })
            });
            group.finish();
        }
    }
}

criterion_group!(benches, bench_hash_map_probe, bench_array_map);
criterion_main!(benches);
