// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: See the NOTICE file distributed with this work for additional information regarding copyright ownership
// SPDX-FileContributor: Licensed to the Apache Software Foundation (ASF) under one or more contributor license agreements

//! Build side cost of `HashJoinExec`.
//!
//! Each case joins a large build side against a single probe row, so the time
//! is dominated by collecting the build input and building the hash table.
//! Integer keys are scattered over the whole `i64` domain so the perfect hash
//! join (`ArrayMap`) is not selected.

use std::hint::black_box;
use std::sync::Arc;

use arrow::array::{ArrayRef, Int64Array, RecordBatch, StringArray};
use arrow::datatypes::{DataType, Field, Schema};
use criterion::{BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use datafusion_common::{JoinType, Result};
use datafusion_execution::TaskContext;
use datafusion_physical_expr::expressions::col;
use datafusion_physical_plan::joins::{HashJoinExecBuilder, PartitionMode};
use datafusion_physical_plan::test::TestMemoryExec;
use datafusion_physical_plan::{ExecutionPlan, collect};
use tokio::runtime::Builder;

const BATCH_SIZE: usize = 8192;

#[derive(Clone, Copy)]
enum Keys {
    /// One `Int64` key, every key distinct.
    Unique,
    /// One `Int64` key, each key repeated 4 times.
    Dup4,
    /// Two `Int64` keys, every pair distinct.
    TwoCols,
    /// One `Utf8` key, every key distinct.
    Utf8,
}

impl Keys {
    fn name(self) -> &'static str {
        match self {
            Keys::Unique => "i64_unique",
            Keys::Dup4 => "i64_dup4",
            Keys::TwoCols => "i64x2_unique",
            Keys::Utf8 => "utf8_unique",
        }
    }

    fn key_columns(self) -> &'static [&'static str] {
        match self {
            Keys::TwoCols => &["k0", "k1"],
            _ => &["k0"],
        }
    }

    /// Output rows of joining the single probe row, the build side's row 0.
    fn expected_matches(self) -> usize {
        match self {
            Keys::Dup4 => 4,
            _ => 1,
        }
    }
}

/// Scatters `i` over the `i64` domain without collisions.
fn scatter(i: usize) -> i64 {
    (i as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15) as i64
}

fn schema(keys: Keys) -> Arc<Schema> {
    let key_type = match keys {
        Keys::Utf8 => DataType::Utf8,
        _ => DataType::Int64,
    };
    let mut fields = vec![Field::new("k0", key_type, false)];
    if matches!(keys, Keys::TwoCols) {
        fields.push(Field::new("k1", DataType::Int64, false));
    }
    fields.push(Field::new("payload", DataType::Int64, false));
    Arc::new(Schema::new(fields))
}

fn make_batch(
    keys: Keys,
    schema: &Arc<Schema>,
    rows: std::ops::Range<usize>,
) -> RecordBatch {
    let mut columns: Vec<ArrayRef> = match keys {
        Keys::Unique => vec![Arc::new(Int64Array::from_iter_values(
            rows.clone().map(scatter),
        ))],
        Keys::Dup4 => vec![Arc::new(Int64Array::from_iter_values(
            rows.clone().map(|i| scatter(i / 4)),
        ))],
        Keys::TwoCols => vec![
            Arc::new(Int64Array::from_iter_values(
                rows.clone().map(|i| scatter(i / 16)),
            )),
            Arc::new(Int64Array::from_iter_values(
                rows.clone().map(|i| (i % 16) as i64),
            )),
        ],
        Keys::Utf8 => vec![Arc::new(StringArray::from_iter_values(
            rows.clone().map(|i| format!("key-{:016x}", scatter(i))),
        ))],
    };
    columns.push(Arc::new(Int64Array::from_iter_values(
        rows.map(|i| i as i64),
    )));
    RecordBatch::try_new(Arc::clone(schema), columns).unwrap()
}

struct Workload {
    schema: Arc<Schema>,
    keys: Keys,
    build: Vec<RecordBatch>,
    probe: RecordBatch,
}

impl Workload {
    fn new(keys: Keys, build_rows: usize) -> Self {
        let schema = schema(keys);
        let build = (0..build_rows)
            .step_by(BATCH_SIZE)
            .map(|start| {
                make_batch(keys, &schema, start..(start + BATCH_SIZE).min(build_rows))
            })
            .collect();
        let probe = make_batch(keys, &schema, 0..1);
        Self {
            schema,
            keys,
            build,
            probe,
        }
    }

    fn plan(&self) -> Result<Arc<dyn ExecutionPlan>> {
        let on = self
            .keys
            .key_columns()
            .iter()
            .map(|name| Ok((col(name, &self.schema)?, col(name, &self.schema)?)))
            .collect::<Result<Vec<_>>>()?;
        let join = HashJoinExecBuilder::new(
            TestMemoryExec::try_new_exec(
                std::slice::from_ref(&self.build),
                Arc::clone(&self.schema),
                None,
            )?,
            TestMemoryExec::try_new_exec(
                &[vec![self.probe.clone()]],
                Arc::clone(&self.schema),
                None,
            )?,
            on,
            JoinType::Inner,
        )
        .with_partition_mode(PartitionMode::CollectLeft)
        .build()?;
        Ok(Arc::new(join))
    }
}

fn bench_build(c: &mut Criterion) {
    let rt = Builder::new_current_thread().build().unwrap();
    let ctx = Arc::new(TaskContext::default());

    let mut group = c.benchmark_group("hash_join_build");
    for keys in [Keys::Unique, Keys::Dup4, Keys::TwoCols, Keys::Utf8] {
        for build_rows in [100_000, 1_000_000, 4_000_000] {
            let workload = Workload::new(keys, build_rows);
            group.throughput(Throughput::Elements(build_rows as u64));
            group.bench_function(BenchmarkId::new(keys.name(), build_rows), |b| {
                b.iter(|| {
                    let plan = workload.plan().unwrap();
                    let out = rt.block_on(collect(plan, Arc::clone(&ctx))).unwrap();
                    assert_eq!(
                        out.iter().map(|b| b.num_rows()).sum::<usize>(),
                        keys.expected_matches()
                    );
                    black_box(out)
                })
            });
        }
    }
    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = bench_build
}
criterion_main!(benches);
