// SPDX-License-Identifier: Apache-2.0
// SPDX-FileCopyrightText: See the NOTICE file distributed with this work for additional information regarding copyright ownership
// SPDX-FileContributor: Licensed to the Apache Software Foundation (ASF) under one or more contributor license agreements

//! A sort-merge join over two sorted file scans, split into independent
//! joins over disjoint ranges of the first join key.
//!
//! See `datafusion.optimizer.range_split_sort_merge_join`.

use std::sync::Arc;

use arrow::array::RecordBatch;
use arrow::buffer::ScalarBuffer;
use arrow::compute::SortOptions;
use arrow::datatypes::DataType;
use datafusion_common::stats::Precision;
use datafusion_common::tree_node::TreeNodeRecursion;
use datafusion_common::{Result, ScalarValue};
use datafusion_datasource::PartitionedFile;
use datafusion_datasource::file_groups::FileGroup;
use datafusion_datasource::file_scan_config::{FileScanConfig, FileScanConfigBuilder};
use datafusion_datasource::source::DataSourceExec;
use datafusion_execution::TaskContext;
use datafusion_physical_expr::expressions::Column;
use datafusion_physical_expr::{
    EquivalenceProperties, LexOrdering, PhysicalExpr, PhysicalSortExpr,
};
use datafusion_physical_plan::execution_plan::{Boundedness, EmissionType};
use datafusion_physical_plan::joins::SortMergeJoinExec;
use datafusion_physical_plan::stream::RecordBatchStreamAdapter;
use datafusion_physical_plan::{
    DisplayAs, DisplayFormatType, ExecutionPlan, ExecutionPlanProperties, Partitioning,
    PlanProperties, SendableRecordBatchStream,
};
use futures::StreamExt;

/// Splits `join`, a sort-merge join whose inputs are both file scans, into
/// `n` joins over disjoint ranges of its first key, when the scans' files
/// prove the split is exact: every file is sorted ascending on the join
/// keys, has exact min/max statistics and no nulls for the first key, and
/// each side's files cover non-overlapping key ranges. Returns `None`
/// otherwise.
///
/// Rows with equal keys always fall in the same range, so each range joins
/// independently, for every join type. A range scans only the files that
/// overlap it (one ordered file group, so it reads ahead), slices the
/// batches at its bounds and stops once past them.
pub(crate) fn try_range_split(
    join: &SortMergeJoinExec,
    n: usize,
) -> Result<Option<Arc<dyn ExecutionPlan>>> {
    if n < 2 {
        return Ok(None);
    }
    let (Some(left), Some(right)) = (
        SortedScan::try_new(join.left(), join.on().iter().map(|(l, _)| l))?,
        SortedScan::try_new(join.right(), join.on().iter().map(|(_, r)| r))?,
    ) else {
        return Ok(None);
    };
    if left.options != right.options {
        return Ok(None);
    }
    let (min, max) = left
        .files
        .iter()
        .chain(&right.files)
        .fold((i64::MAX, i64::MIN), |(lo, hi), f| {
            (lo.min(f.min), hi.max(f.max))
        });
    if max == i64::MAX {
        return Ok(None);
    }
    // Cut the ranges where the larger input's files start, so that input
    // decodes each file once: a range boundary inside a file makes both
    // ranges decode the part of it before the boundary.
    let larger = if left.rows() >= right.rows() {
        &left
    } else {
        &right
    };
    let total = larger.rows().max(1);
    let mut bounds = vec![min];
    let mut seen = 0;
    for file in &larger.files {
        let next = bounds.len();
        if next < n && seen * n >= total * next && file.min > *bounds.last().unwrap() {
            bounds.push(file.min);
        }
        seen += file.rows;
    }
    // Half-open ranges covering `min..=max`.
    bounds.push(max + 1);
    if bounds.len() < 3 {
        return Ok(None);
    }
    let sort_options = vec![left.options; join.on().len()];
    let ranges = (0..n)
        .map(|i| {
            let (lo, hi) = (bounds[i], bounds[i + 1]);
            let joined = SortMergeJoinExec::try_new(
                left.range(lo, hi),
                right.range(lo, hi),
                join.on().to_vec(),
                join.filter().clone(),
                join.join_type(),
                sort_options.clone(),
                join.null_equality(),
            )?
            .with_projection(join.projection.clone())?;
            Ok(Arc::new(joined) as Arc<dyn ExecutionPlan>)
        })
        .collect::<Result<Vec<_>>>()?;
    Ok(Some(Arc::new(RangeSplitJoinExec::new(ranges, bounds))))
}

/// A file with the exact min and max of the first join key.
struct KeyedFile {
    file: PartitionedFile,
    min: i64,
    max: i64,
    /// Its row count, 1 when the statistics do not say.
    rows: usize,
}

/// One join input: a file scan whose files are each sorted on the join keys.
struct SortedScan {
    config: FileScanConfig,
    /// The first key's column index in the scan's output.
    key: usize,
    options: SortOptions,
    /// In key order, non-overlapping.
    files: Vec<KeyedFile>,
}

impl SortedScan {
    fn try_new<'a>(
        plan: &Arc<dyn ExecutionPlan>,
        keys: impl Iterator<Item = &'a Arc<dyn PhysicalExpr>>,
    ) -> Result<Option<Self>> {
        let Some(config) = plan
            .downcast_ref::<DataSourceExec>()
            .and_then(|exec| exec.data_source().downcast_ref::<FileScanConfig>())
        else {
            return Ok(None);
        };
        if config.limit.is_some() {
            return Ok(None);
        }
        let keys: Vec<&Column> = match keys.map(|k| k.downcast_ref::<Column>()).collect()
        {
            Some(keys) => keys,
            None => return Ok(None),
        };
        let key = keys[0];
        let projected = config.projected_schema()?;
        if !matches!(
            projected.field(key.index()).data_type(),
            DataType::Int64
                | DataType::Date64
                | DataType::Time64(_)
                | DataType::Timestamp(..)
                | DataType::Duration(_)
        ) {
            return Ok(None);
        }
        // The files must each be ordered on every join key (ascending).
        let Some(options) = [false, true].into_iter().find_map(|nulls_first| {
            let options = SortOptions::new(false, nulls_first);
            let required = LexOrdering::new(
                keys.iter()
                    .map(|k| PhysicalSortExpr::new(Arc::new((*k).clone()), options)),
            )?;
            let eq = EquivalenceProperties::new_with_orderings(
                Arc::clone(&projected),
                config.output_ordering.clone(),
            );
            eq.ordering_satisfy(required).ok()?.then_some(options)
        }) else {
            return Ok(None);
        };
        let Ok(stats_index) = config.file_schema().index_of(key.name()) else {
            return Ok(None);
        };
        let exact = |p: &Precision<ScalarValue>| match p {
            Precision::Exact(v) => match v.cast_to(&DataType::Int64) {
                Ok(ScalarValue::Int64(Some(v))) => Some(v),
                _ => None,
            },
            _ => None,
        };
        let mut files = Vec::new();
        for file in config.file_groups.iter().flat_map(|g| g.files()) {
            // A file split into byte ranges is covered whole, once.
            if file.range.is_some() {
                return Ok(None);
            }
            let Some(col) = file
                .statistics
                .as_ref()
                .and_then(|s| s.column_statistics.get(stats_index))
            else {
                return Ok(None);
            };
            let (Some(min), Some(max)) = (exact(&col.min_value), exact(&col.max_value))
            else {
                return Ok(None);
            };
            if col.null_count != Precision::Exact(0) {
                return Ok(None);
            }
            let rows = file
                .statistics
                .as_ref()
                .and_then(|s| s.num_rows.get_value().copied())
                .unwrap_or(1);
            files.push(KeyedFile {
                file: file.clone(),
                min,
                max,
                rows,
            });
        }
        files.sort_by_key(|f| (f.min, f.max));
        if files.is_empty() || files.windows(2).any(|w| w[1].min < w[0].max) {
            return Ok(None);
        }
        Ok(Some(Self {
            config: config.clone(),
            key: key.index(),
            options,
            files,
        }))
    }

    fn rows(&self) -> usize {
        self.files.iter().map(|f| f.rows).sum()
    }

    /// This input's rows with keys in `lo..hi`.
    fn range(&self, lo: i64, hi: i64) -> Arc<dyn ExecutionPlan> {
        let files = self
            .files
            .iter()
            .filter(|f| f.max >= lo && f.min < hi)
            .map(|f| f.file.clone())
            .collect();
        let config = FileScanConfigBuilder::from(self.config.clone())
            .with_file_groups(vec![FileGroup::new(files)])
            .build();
        Arc::new(RangeSliceExec::new(
            DataSourceExec::from_data_source(config),
            self.key,
            lo,
            hi,
        ))
    }
}

/// The rows of its single-partition input, sorted ascending on a null-free
/// 64-bit `key`, that lie in `lo..hi`. Stops reading
/// its input at the first batch past the range.
#[derive(Debug)]
struct RangeSliceExec {
    input: Arc<dyn ExecutionPlan>,
    key: usize,
    lo: i64,
    hi: i64,
    properties: Arc<PlanProperties>,
}

impl RangeSliceExec {
    fn new(input: Arc<dyn ExecutionPlan>, key: usize, lo: i64, hi: i64) -> Self {
        let properties = PlanProperties::new(
            input.equivalence_properties().clone(),
            Partitioning::UnknownPartitioning(1),
            EmissionType::Incremental,
            Boundedness::Bounded,
        );
        Self {
            input,
            key,
            lo,
            hi,
            properties: Arc::new(properties),
        }
    }
}

impl DisplayAs for RangeSliceExec {
    fn fmt_as(
        &self,
        _: DisplayFormatType,
        f: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        write!(
            f,
            "RangeSliceExec: key@{} in [{}, {})",
            self.key, self.lo, self.hi
        )
    }
}

impl ExecutionPlan for RangeSliceExec {
    fn name(&self) -> &str {
        "RangeSliceExec"
    }

    fn properties(&self) -> &Arc<PlanProperties> {
        &self.properties
    }

    fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>> {
        vec![&self.input]
    }

    fn apply_expressions(
        &self,
        _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>,
    ) -> Result<TreeNodeRecursion> {
        Ok(TreeNodeRecursion::Continue)
    }

    fn with_new_children(
        self: Arc<Self>,
        children: Vec<Arc<dyn ExecutionPlan>>,
    ) -> Result<Arc<dyn ExecutionPlan>> {
        Ok(Arc::new(Self::new(
            Arc::clone(&children[0]),
            self.key,
            self.lo,
            self.hi,
        )))
    }

    fn execute(
        &self,
        partition: usize,
        context: Arc<TaskContext>,
    ) -> Result<SendableRecordBatchStream> {
        let input = self.input.execute(partition, context)?;
        let schema = input.schema();
        let (key, lo, hi) = (self.key, self.lo, self.hi);
        let stream = input
            .map(move |batch| {
                let batch = batch?;
                let data = batch.column(key).to_data();
                let keys = ScalarBuffer::<i64>::new(
                    data.buffers()[0].clone(),
                    data.offset(),
                    data.len(),
                );
                let start = keys.partition_point(|k| *k < lo);
                let end = keys.partition_point(|k| *k < hi);
                // `None`: the batch starts past the range, and so does the
                // rest of the (sorted) input.
                Ok(if end == 0 && !keys.is_empty() {
                    None
                } else {
                    Some(batch.slice(start, end - start))
                })
            })
            .take_while(|b: &Result<Option<RecordBatch>>| {
                futures::future::ready(!matches!(b, Ok(None)))
            })
            .filter_map(|b| {
                futures::future::ready(match b {
                    Ok(Some(b)) if b.num_rows() == 0 => None,
                    Ok(Some(b)) => Some(Ok(b)),
                    Ok(None) => None,
                    Err(e) => Some(Err(e)),
                })
            });
        Ok(Box::pin(RecordBatchStreamAdapter::new(schema, stream)))
    }
}

/// `n` sort-merge joins, one per range of the first join key, each its own
/// output partition. The joins are complete plans of their own, so the
/// optimizer does not see into them (or re-partition their scans).
#[derive(Debug)]
pub(crate) struct RangeSplitJoinExec {
    ranges: Vec<Arc<dyn ExecutionPlan>>,
    bounds: Vec<i64>,
    properties: Arc<PlanProperties>,
}

impl RangeSplitJoinExec {
    fn new(ranges: Vec<Arc<dyn ExecutionPlan>>, bounds: Vec<i64>) -> Self {
        let properties = PlanProperties::new(
            EquivalenceProperties::new(ranges[0].schema()),
            Partitioning::UnknownPartitioning(ranges.len()),
            EmissionType::Incremental,
            Boundedness::Bounded,
        );
        Self {
            ranges,
            bounds,
            properties: Arc::new(properties),
        }
    }
}

impl DisplayAs for RangeSplitJoinExec {
    fn fmt_as(
        &self,
        t: DisplayFormatType,
        f: &mut std::fmt::Formatter,
    ) -> std::fmt::Result {
        write!(
            f,
            "RangeSplitJoinExec: ranges={}, bounds={:?}, join=",
            self.ranges.len(),
            self.bounds
        )?;
        self.ranges[0].fmt_as(t, f)
    }
}

impl ExecutionPlan for RangeSplitJoinExec {
    fn name(&self) -> &str {
        "RangeSplitJoinExec"
    }

    fn properties(&self) -> &Arc<PlanProperties> {
        &self.properties
    }

    fn children(&self) -> Vec<&Arc<dyn ExecutionPlan>> {
        vec![]
    }

    fn apply_expressions(
        &self,
        _f: &mut dyn FnMut(&Arc<dyn PhysicalExpr>) -> Result<TreeNodeRecursion>,
    ) -> Result<TreeNodeRecursion> {
        Ok(TreeNodeRecursion::Continue)
    }

    fn with_new_children(
        self: Arc<Self>,
        _children: Vec<Arc<dyn ExecutionPlan>>,
    ) -> Result<Arc<dyn ExecutionPlan>> {
        Ok(self)
    }

    fn execute(
        &self,
        partition: usize,
        context: Arc<TaskContext>,
    ) -> Result<SendableRecordBatchStream> {
        self.ranges[partition].execute(0, context)
    }
}
