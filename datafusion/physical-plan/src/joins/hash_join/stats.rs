// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: See the NOTICE file distributed with this work for additional information regarding copyright ownership
// SPDX-FileContributor: Licensed to the Apache Software Foundation (ASF) under one or more contributor license agreements

//! Opt-in workload statistics for [`super::HashJoinExec`].
//!
//! Set `DATAFUSION_HASH_JOIN_STATS=1` to print one JSON line to stderr per
//! build side (`"kind":"build"`) and per probe stream (`"kind":"probe"`).
//! The two are linked by `build_id`. Disabled by default; the hot paths only
//! pay for a cached boolean check.

use std::cell::Cell;
use std::fmt::Write;
use std::sync::OnceLock;
use std::time::Duration;

use datafusion_common::instant::Instant;

use arrow::array::{Array, ArrayRef, RecordBatch};
use datafusion_physical_expr::PhysicalExprRef;

pub(crate) fn enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| {
        std::env::var("DATAFUSION_HASH_JOIN_STATS")
            .is_ok_and(|v| v != "0" && !v.is_empty())
    })
}

thread_local! {
    /// Candidate `(build, probe)` pairs returned by hash lookups before the
    /// key equality check, used to count hash collisions.
    static CANDIDATES: Cell<u64> = const { Cell::new(0) };
}

pub(crate) fn add_candidates(n: usize) {
    if enabled() {
        CANDIDATES.with(|c| c.set(c.get() + n as u64));
    }
}

pub(crate) fn candidates() -> u64 {
    CANDIDATES.with(|c| c.get())
}

pub(crate) fn now() -> Option<Instant> {
    enabled().then(Instant::now)
}

pub(crate) fn elapsed(start: Option<Instant>) -> Duration {
    start.map(|s| s.elapsed()).unwrap_or_default()
}

fn json_str(out: &mut String, s: &str) {
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

/// Minimal JSON object writer, to avoid a serde dependency.
pub(crate) struct JsonLine(String);

impl JsonLine {
    pub(crate) fn new(kind: &str) -> Self {
        let mut s = String::from("{\"kind\":");
        json_str(&mut s, kind);
        Self(s)
    }

    pub(crate) fn num(mut self, k: &str, v: impl std::fmt::Display) -> Self {
        self.0.push(',');
        json_str(&mut self.0, k);
        let _ = write!(self.0, ":{v}");
        self
    }

    pub(crate) fn opt_num(self, k: &str, v: Option<impl std::fmt::Display>) -> Self {
        match v {
            Some(v) => self.num(k, v),
            None => self.raw(k, "null"),
        }
    }

    pub(crate) fn str(mut self, k: &str, v: &str) -> Self {
        self.0.push(',');
        json_str(&mut self.0, k);
        self.0.push(':');
        json_str(&mut self.0, v);
        self
    }

    pub(crate) fn strs<S: AsRef<str>>(
        mut self,
        k: &str,
        vs: impl IntoIterator<Item = S>,
    ) -> Self {
        self.0.push(',');
        json_str(&mut self.0, k);
        self.0.push_str(":[");
        for (i, v) in vs.into_iter().enumerate() {
            if i > 0 {
                self.0.push(',');
            }
            json_str(&mut self.0, v.as_ref());
        }
        self.0.push(']');
        self
    }

    pub(crate) fn raw(mut self, k: &str, v: &str) -> Self {
        self.0.push(',');
        json_str(&mut self.0, k);
        let _ = write!(self.0, ":{v}");
        self
    }

    pub(crate) fn ns(self, k: &str, d: Duration) -> Self {
        self.num(k, d.as_nanos())
    }

    pub(crate) fn emit(mut self) {
        self.0.push('}');
        eprintln!("HJSTATS {}", self.0);
    }
}

pub(crate) fn expr_names(exprs: &[PhysicalExprRef]) -> Vec<String> {
    exprs.iter().map(|e| e.to_string()).collect()
}

pub(crate) fn key_types(values: &[ArrayRef]) -> Vec<String> {
    values.iter().map(|v| v.data_type().to_string()).collect()
}

pub(crate) fn batch_bytes(batch: &RecordBatch) -> usize {
    batch
        .columns()
        .iter()
        .map(|c| c.get_array_memory_size())
        .sum()
}

/// Accumulated statistics of one probe stream.
#[derive(Default)]
pub(crate) struct ProbeStats {
    pub batches: u64,
    pub rows: u64,
    pub null_keys: u64,
    pub hash: Duration,
    pub lookup: Duration,
    pub candidates: u64,
    pub matched_pairs: u64,
    pub filter: Duration,
    pub filtered_pairs: u64,
    pub adjust: Duration,
    pub materialize: Duration,
    pub push: Duration,
    pub out_batches: u64,
    pub out_rows: u64,
    pub out_bytes: u64,
    pub lookup_chunks: u64,
    pub unmatched_emit: Duration,
    pub min_batch_rows: u64,
    pub max_batch_rows: u64,
    pub identity_probe_chunks: u64,
    pub identity_probe_rows: u64,
    pub contiguous_probe_chunks: u64,
    pub contiguous_probe_rows: u64,
}
