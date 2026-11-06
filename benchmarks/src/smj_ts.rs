// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: See the NOTICE file distributed with this work for additional information regarding copyright ownership
// SPDX-FileContributor: Licensed to the Apache Software Foundation (ASF) under one or more contributor license agreements

//! Sort-merge joins over pre-sorted time-series Parquet.
//!
//! Unlike the `smj` benchmark, whose inputs are sorted by a `SortExec` inside
//! each query, every table here is written to Parquet already sorted by its
//! timestamp and registered `WITH ORDER`, so no sort runs. Each query measures
//! the path a sorted join takes through execution: the ordered scan, any
//! order-preserving repartition, the merge join itself, and its output.
//!
//! Tables (generated once into `--path`, as several non-overlapping files):
//!
//! - `quotes`: 10M rows, one per millisecond (`ts`), `sym` in 0..16, bid/ask.
//! - `trades`: 1M rows, one per 10 ms, so it matches every 10th quote.
//! - `events`: 3M rows, three per 7 ms timestamp, a 3:1 fan-in on `quotes`.
//!
//! ```text
//! dfbench smj-ts --path /tmp/smj_ts --iterations 5
//! ```

use std::path::PathBuf;
use std::sync::Arc;

use crate::util::{BenchmarkRun, CommonOpt, QueryResult};
use arrow::compute::SortOptions;
use clap::Args;
use datafusion::physical_plan::joins::{HashJoinExec, SortMergeJoinExec};
use datafusion::physical_plan::{ExecutionPlan, displayable, execute_stream};
use datafusion::{error::Result, prelude::SessionContext};
use datafusion_common::instant::Instant;
use datafusion_common::tree_node::{Transformed, TransformedResult, TreeNode};
use datafusion_common::{exec_datafusion_err, exec_err};
use futures::StreamExt;

/// Run sort-merge joins over pre-sorted time-series Parquet.
#[derive(Debug, Args, Clone)]
#[command(verbatim_doc_comment)]
pub struct RunOpt {
    /// Query number (between 1 and the number of queries). If not specified, runs all queries
    #[arg(short, long)]
    query: Option<usize>,

    /// Run plain scans of the same Parquet (aggregate, filter, top-k) at the
    /// configured partition count instead of the joins: a check that scan
    /// changes made for the sorted join do not cost multi-partition scans.
    #[arg(long = "scan-queries")]
    scan_queries: bool,

    /// Run the join queries with DataFusion's default plan (hash joins at the
    /// configured partition count) instead of the single-partition sort-merge
    /// join: the baseline a user gets without asking for a merge join.
    #[arg(long = "hash-join")]
    hash_join: bool,

    /// Let the planner choose the sort-merge join at the configured
    /// partition count (hash-repartitioning both ordered inputs while
    /// keeping their order) instead of the single-partition plan.
    #[arg(long = "parallel")]
    parallel: bool,

    /// Directory holding (or receiving) the generated Parquet tables
    #[arg(short = 'p', long = "path", default_value = "data/smj_ts")]
    path: PathBuf,

    /// Common options
    #[command(flatten)]
    common: CommonOpt,

    /// If present, write results json here
    #[arg(short = 'o', long = "output")]
    output_path: Option<PathBuf>,
}

const FILES_PER_TABLE: u64 = 8;

/// (table name, total rows, timestamp-in-ms expression of `value`, extra columns)
const TABLES: &[(&str, u64, &str, &str)] = &[
    (
        "quotes",
        10_000_000,
        "value",
        "value % 16 AS sym, (value % 1000) * 0.01 + 100 AS bid, \
         (value % 1000) * 0.01 + 100.05 AS ask",
    ),
    (
        "trades",
        1_000_000,
        "value * 10",
        "value % 16 AS sym, (value % 997) * 0.01 + 100 AS price, value % 500 AS qty",
    ),
    (
        "events",
        3_000_000,
        "(value / 3) * 7",
        "value % 3 AS kind, value AS seq",
    ),
];

/// Each query's comment states the join type, row counts, and match shape.
const QUERIES: &[&str] = &[
    // Q1: INNER trades(1M) x quotes(10M) on ts | 1:1, 90% of quotes unmatched
    "SELECT t.ts, t.price, q.bid, q.ask FROM trades t JOIN quotes q ON t.ts = q.ts",
    // Q2: INNER quotes(10M) x quotes(10M) on ts | dense 1:1 self-join
    "SELECT a.ts, a.bid, b.ask FROM quotes a JOIN quotes b ON a.ts = b.ts",
    // Q3: LEFT quotes(10M) x trades(1M) on ts | 90% of streamed rows null-joined
    "SELECT q.ts, q.bid, t.price FROM quotes q LEFT JOIN trades t ON q.ts = t.ts",
    // Q4: INNER events(3M) x quotes(10M) on ts | 3:1 duplicate keys
    "SELECT e.ts, e.seq, q.bid FROM events e JOIN quotes q ON e.ts = q.ts",
    // Q5: FULL trades(1M) x events(3M) on ts | sparse overlap on both sides
    "SELECT t.ts, e.ts, t.price, e.seq FROM trades t FULL JOIN events e ON t.ts = e.ts",
    // Q6: INNER trades x quotes on (ts, sym) with a 50% join filter
    "SELECT t.ts, t.price, q.bid FROM trades t JOIN quotes q \
       ON t.ts = q.ts AND t.sym = q.sym AND q.bid < t.price",
    // Q7: LEFT SEMI quotes(10M) x trades(1M) on ts | 10% of streamed rows match
    "SELECT q.ts, q.bid FROM quotes q WHERE EXISTS (SELECT 1 FROM trades t WHERE t.ts = q.ts)",
    // Q8: LEFT ANTI quotes(10M) x trades(1M) on ts | 90% of streamed rows survive
    "SELECT q.ts, q.bid FROM quotes q WHERE NOT EXISTS (SELECT 1 FROM trades t WHERE t.ts = q.ts)",
    // Q9: INNER trades x quotes on ts + aggregate (join output consumed by an aggregate)
    "SELECT count(*), sum(t.price - q.bid) FROM trades t JOIN quotes q ON t.ts = q.ts",
];

/// Multi-partition scans of `quotes`, for `--scan-queries`.
const SCAN_QUERIES: &[&str] = &[
    // S1: full scan into a grouped aggregate
    "SELECT sym, count(*), sum(bid), max(ask) FROM quotes GROUP BY sym",
    // S2: full scan with a filter
    "SELECT count(*) FROM quotes WHERE bid > 105.0",
    // S3: top-k on the sort column
    "SELECT ts, bid FROM quotes ORDER BY ts DESC LIMIT 10",
];

impl RunOpt {
    pub async fn run(self) -> Result<()> {
        println!("Running sorted time-series SMJ benchmarks: {self:#?}\n");

        let queries = if self.scan_queries {
            SCAN_QUERIES
        } else {
            QUERIES
        };
        let query_range = match self.query {
            Some(q) if (1..=queries.len()).contains(&q) => q..=q,
            Some(q) => {
                return exec_err!(
                    "Query {q} not found. Available queries: 1 to {}",
                    queries.len()
                );
            }
            None => 1..=queries.len(),
        };

        let config = if self.scan_queries || self.hash_join {
            self.common.config()?
        } else if self.parallel {
            self.common
                .config()?
                .set_bool("datafusion.optimizer.prefer_hash_join", false)
                .set_bool("datafusion.optimizer.prefer_existing_sort", true)
        } else {
            self.common
                .config()?
                // One partition per input keeps each scan in file (=
                // timestamp) order; the planned hash join is swapped for a
                // sort-merge join (see `plan`). Dynamic filters belong to
                // the hash join being replaced.
                .with_target_partitions(1)
                .set_bool("datafusion.optimizer.enable_dynamic_filter_pushdown", false)
        };
        let rt = self.common.build_runtime()?;
        let ctx = SessionContext::new_with_config_rt(config, rt);

        self.generate_data(&ctx).await?;
        self.register_tables(&ctx).await?;

        let mut benchmark_run = BenchmarkRun::new();
        benchmark_run.set_memory_pool(&ctx.runtime_env().memory_pool);
        for query_id in query_range {
            let sql = queries[query_id - 1];
            benchmark_run.start_new_case(&format!("Query {query_id}"));
            for result in self.benchmark_query(sql, query_id, &ctx).await? {
                benchmark_run.write_iter(result.elapsed, result.row_count);
            }
        }
        benchmark_run.maybe_write_json(self.output_path.as_ref())?;
        Ok(())
    }

    fn table_dir(&self, table: &str) -> PathBuf {
        self.path.join(table)
    }

    /// Writes each table as `FILES_PER_TABLE` Parquet files covering
    /// consecutive, non-overlapping timestamp ranges, each sorted by `ts`.
    async fn generate_data(&self, ctx: &SessionContext) -> Result<()> {
        for (name, rows, ts_expr, extra) in TABLES {
            let dir = self.table_dir(name);
            if dir.exists() {
                continue;
            }
            std::fs::create_dir_all(&dir)?;
            println!("Generating {name} into {}", dir.display());
            let per_file = rows.div_ceil(FILES_PER_TABLE);
            for file in 0..FILES_PER_TABLE {
                let start = file * per_file;
                let end = (start + per_file).min(*rows);
                let file_path = dir.join(format!("part-{file:03}.parquet"));
                let sql = format!(
                    "COPY (SELECT to_timestamp_millis({ts_expr}) AS ts, {extra} \
                     FROM range({start}, {end}) ORDER BY value) \
                     TO '{}' STORED AS PARQUET",
                    file_path.display()
                );
                ctx.sql(&sql).await?.collect().await?;
            }
        }
        Ok(())
    }

    async fn register_tables(&self, ctx: &SessionContext) -> Result<()> {
        for (name, ..) in TABLES {
            let sql = format!(
                "CREATE EXTERNAL TABLE {name} STORED AS PARQUET LOCATION '{}/' \
                 WITH ORDER (ts ASC)",
                self.table_dir(name).display()
            );
            ctx.sql(&sql).await?.collect().await?;
        }
        Ok(())
    }

    /// Plans `sql` and replaces every `HashJoinExec` with the equivalent
    /// `SortMergeJoinExec`. Both inputs are ordered scans, so the result is
    /// the sort-free sorted-join plan: scan -> merge join -> consumer.
    async fn plan(
        &self,
        sql: &str,
        ctx: &SessionContext,
    ) -> Result<Arc<dyn ExecutionPlan>> {
        let plan = ctx.sql(sql).await?.create_physical_plan().await?;
        if self.hash_join || self.parallel {
            return Ok(plan);
        }
        plan.transform_up(|node| {
            let Some(hj) = node.downcast_ref::<HashJoinExec>() else {
                return Ok(Transformed::no(node));
            };
            let smj = SortMergeJoinExec::try_new(
                Arc::clone(hj.left()),
                Arc::clone(hj.right()),
                hj.on().to_vec(),
                hj.filter().cloned(),
                *hj.join_type(),
                vec![SortOptions::new(false, false); hj.on().len()],
                hj.null_equality(),
            )?
            .with_projection(hj.projection.as_deref().map(<[usize]>::to_vec))?;
            Ok(Transformed::yes(Arc::new(smj) as Arc<dyn ExecutionPlan>))
        })
        .data()
    }

    async fn benchmark_query(
        &self,
        sql: &str,
        query_id: usize,
        ctx: &SessionContext,
    ) -> Result<Vec<QueryResult>> {
        let plan = self.plan(sql, ctx).await?;
        let plan_string = displayable(plan.as_ref()).indent(true).to_string();
        let merge_join = !self.scan_queries && !self.hash_join;
        if self.parallel && plan_string.contains("HashJoinExec") {
            return Err(exec_datafusion_err!(
                "Query {query_id} still uses a hash join: {plan_string}"
            ));
        }
        if merge_join && !plan_string.contains("SortMergeJoinExec") {
            return Err(exec_datafusion_err!(
                "Query {query_id} does not use a sort-merge join: {plan_string}"
            ));
        }
        // The single-partition plan must not sort; the planner's parallel
        // plan sorts wherever the inputs' ordering does not cover the keys.
        if merge_join && !self.parallel && plan_string.contains("SortExec") {
            return Err(exec_datafusion_err!(
                "Query {query_id} sorts its pre-sorted input: {plan_string}"
            ));
        }
        if self.common.debug {
            println!("{}", displayable(plan.as_ref()).indent(true));
        }

        let mut results = vec![];
        for i in 0..self.common.iterations {
            let start = Instant::now();
            let plan = self.plan(sql, ctx).await?;
            let mut stream = execute_stream(plan, ctx.task_ctx())?;
            let mut row_count = 0;
            while let Some(batch) = stream.next().await {
                row_count += batch?.num_rows();
            }
            let elapsed = start.elapsed();
            println!(
                "Query {query_id} iteration {i} returned {row_count} rows in {elapsed:?}"
            );
            results.push(QueryResult { elapsed, row_count });
        }
        Ok(results)
    }
}
