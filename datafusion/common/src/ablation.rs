// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: See the NOTICE file distributed with this work for additional information regarding copyright ownership
// SPDX-FileContributor: Licensed to the Apache Software Foundation (ASF) under one or more contributor license agreements

//! EXPERIMENT ONLY (ablation branch): runtime switches for each sorted-join
//! optimization, so one binary can benchmark any combination.
//!
//! `SMJ_OFF` lists the changes to disable, e.g. `SMJ_OFF=1,8,11`; `all`
//! disables every one. Read once per process.

use std::sync::LazyLock;

/// Bits of disabled changes.
static OFF: LazyLock<u64> = LazyLock::new(|| {
    let Ok(spec) = std::env::var("SMJ_OFF") else {
        return 0;
    };
    if spec.trim() == "all" {
        return u64::MAX;
    }
    spec.split(',')
        .filter_map(|s| s.trim().parse::<u32>().ok())
        .fold(0, |mask, n| mask | (1u64 << n))
});

/// Whether change `n` is enabled.
#[inline(always)]
pub fn on(n: u32) -> bool {
    *OFF & (1u64 << n) == 0
}
