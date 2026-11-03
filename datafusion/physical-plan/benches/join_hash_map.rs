// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: See the NOTICE file distributed with this work for additional information regarding copyright ownership
// SPDX-FileContributor: Licensed to the Apache Software Foundation (ASF) under one or more contributor license agreements

//! Compares the chained `JoinHashMapU32` with the `UnchainedJoinHashMapU32`
//! used by the hash join build side, building from and probing with
//! precomputed hashes.

use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use datafusion_physical_plan::joins::join_hash_map::{
    JoinHashMapType, JoinHashMapU32, ProbeOffset,
};
use datafusion_physical_plan::joins::pipelined_hash_map::PipelinedJoinHashMapU32;
use datafusion_physical_plan::joins::unchained_hash_map::UnchainedJoinHashMapU32;
use std::hint::black_box;

const PROBE_ROWS: usize = 1 << 20;
const BATCH_SIZE: usize = 8192;

fn splitmix(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^ (x >> 31)
}

/// `build_rows` hashes over `build_rows / dups` distinct keys, and probe
/// hashes of which `hit_percent` are build keys.
fn hashes(build_rows: usize, dups: usize, hit_percent: u64) -> (Vec<u64>, Vec<u64>) {
    let distinct = (build_rows / dups) as u64;
    let build = (0..build_rows as u64)
        .map(|i| splitmix(i % distinct))
        .collect();
    let probe = (0..PROBE_ROWS as u64)
        .map(|i| {
            let r = splitmix(i ^ 0xABCD);
            if r % 100 < hit_percent {
                splitmix((r >> 8) % distinct)
            } else {
                splitmix(distinct + r)
            }
        })
        .collect();
    (build, probe)
}

fn build_chained(build: &[u64]) -> JoinHashMapU32 {
    let mut map = JoinHashMapU32::with_capacity(build.len());
    map.update_from_iter(Box::new(build.iter().enumerate().rev()), 0);
    map
}

fn build_pipelined(build: &[u64]) -> PipelinedJoinHashMapU32 {
    let mut map = PipelinedJoinHashMapU32::with_capacity(build.len());
    map.update_from_iter(Box::new(build.iter().enumerate().rev()), 0);
    map
}

fn build_unchained(build: &[u64]) -> UnchainedJoinHashMapU32 {
    let mut map = UnchainedJoinHashMapU32::with_capacity(build.len());
    map.update_from_iter(Box::new(build.iter().enumerate().rev()), 0);
    map.finalize();
    map
}

fn probe(map: &dyn JoinHashMapType, probe: &[u64]) -> usize {
    let mut input_indices = Vec::with_capacity(BATCH_SIZE);
    let mut match_indices = Vec::with_capacity(BATCH_SIZE);
    let mut matches = 0;
    for batch in probe.chunks(BATCH_SIZE) {
        let mut offset = ProbeOffset::AtRow { row: 0 };
        loop {
            let next = map.get_matched_indices_with_limit_offset(
                batch,
                None,
                BATCH_SIZE,
                offset,
                &mut input_indices,
                &mut match_indices,
            );
            matches += match_indices.len();
            match next {
                Some(next) => offset = next,
                None => break,
            }
        }
    }
    matches
}

fn bench(c: &mut Criterion) {
    for (build_rows, dups, hit) in [
        (1 << 16, 1, 100),
        (1 << 16, 1, 10),
        (1 << 20, 1, 100),
        (1 << 20, 1, 10),
        (1 << 20, 4, 100),
        (1 << 23, 1, 100),
        (1 << 23, 1, 10),
    ] {
        let (build, probe_hashes) = hashes(build_rows, dups, hit);
        let name = format!("rows={build_rows}/dups={dups}/hit={hit}");

        let mut group = c.benchmark_group("join_hash_map_build");
        group.throughput(Throughput::Elements(build_rows as u64));
        group.sample_size(10);
        if hit == 100 {
            group.bench_function(BenchmarkId::new("chained", &name), |b| {
                b.iter(|| black_box(build_chained(&build)))
            });
            group.bench_function(BenchmarkId::new("unchained", &name), |b| {
                b.iter(|| black_box(build_unchained(&build)))
            });
        }
        group.finish();

        let chained = build_chained(&build);
        let unchained = build_unchained(&build);
        let pipelined = build_pipelined(&build);
        assert_eq!(
            probe(&chained, &probe_hashes),
            probe(&pipelined, &probe_hashes)
        );
        assert_eq!(
            probe(&chained, &probe_hashes),
            probe(&unchained, &probe_hashes)
        );

        let mut group = c.benchmark_group("join_hash_map_probe");
        group.throughput(Throughput::Elements(PROBE_ROWS as u64));
        group.sample_size(10);
        group.bench_function(BenchmarkId::new("chained", &name), |b| {
            b.iter(|| probe(&chained, black_box(&probe_hashes)))
        });
        group.bench_function(BenchmarkId::new("pipelined", &name), |b| {
            b.iter(|| probe(&pipelined, black_box(&probe_hashes)))
        });
        group.bench_function(BenchmarkId::new("unchained", &name), |b| {
            b.iter(|| probe(&unchained, black_box(&probe_hashes)))
        });
        group.bench_function(BenchmarkId::new("chained_contains", &name), |b| {
            b.iter(|| chained.contain_hashes(black_box(&probe_hashes)))
        });
        group.bench_function(BenchmarkId::new("pipelined_contains", &name), |b| {
            b.iter(|| pipelined.contain_hashes(black_box(&probe_hashes)))
        });
        group.bench_function(BenchmarkId::new("unchained_contains", &name), |b| {
            b.iter(|| unchained.contain_hashes(black_box(&probe_hashes)))
        });
        group.finish();
    }
}

criterion_group!(benches, bench);
criterion_main!(benches);
