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
use arrow::datatypes::{Schema, SchemaRef};
use datafusion_common::stats::Precision;
use datafusion_common::tree_node::TreeNodeRecursion;
use datafusion_common::{
    JoinSide, JoinType, NullEquality, Result, ScalarValue, internal_err,
};
use datafusion_datasource::PartitionedFile;
use datafusion_datasource::file::FileSource;
use datafusion_datasource::file_groups::FileGroup;
use datafusion_datasource::file_scan_config::{FileScanConfig, FileScanConfigBuilder};
use datafusion_datasource::source::DataSourceExec;
use datafusion_execution::TaskContext;
use datafusion_expr::Operator;
use datafusion_physical_expr::expressions::Column;
use datafusion_physical_expr::expressions::{BinaryExpr, Literal};
use datafusion_physical_expr::{
    EquivalenceProperties, LexOrdering, PhysicalExpr, PhysicalSortExpr,
};
use datafusion_physical_plan::execution_plan::{Boundedness, EmissionType};
use datafusion_physical_plan::joins::SortMergeJoinExec;
use datafusion_physical_plan::joins::utils::{ColumnIndex, JoinFilter};
use datafusion_physical_plan::stream::RecordBatchStreamAdapter;
use datafusion_physical_plan::{
    DisplayAs, DisplayFormatType, ExecutionPlan, ExecutionPlanProperties, Partitioning,
    PlanProperties, SendableRecordBatchStream,
};
use futures::StreamExt;

/// Splits `join`, a sort-merge join whose inputs are both file scans, into
/// `n` joins over disjoint ranges of its first key, when the scans' files
/// prove the split is exact: every file is sorted ascending on a prefix of
/// the join keys (any later keys move into the join filter), has exact
/// min/max statistics and no nulls for the first key, and
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
    // Join each range on the leading keys both inputs are ordered on, with
    // the same null placement; the rest move into the filter.
    let Some((options, ordered)) = [false, true].into_iter().find_map(|nulls_first| {
        let i = usize::from(nulls_first);
        let m = left.prefixes[i].min(right.prefixes[i]);
        (m > 0).then(|| (SortOptions::new(false, nulls_first), m))
    }) else {
        return Ok(None);
    };
    let filter = keys_into_filter(join, ordered)?;
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
    // Cuts inside files need each range's scan to skip what lies outside it.
    let cut_inside_files =
        larger.files.len() < n && left.prunes_row_groups && right.prunes_row_groups;
    // A FULL join cut inside files lost to the hash join (smj-ts as one
    // file per table, Q5: x1.06 [1.01, 1.10]); keep the planner's choice.
    if cut_inside_files && join.join_type() == JoinType::Full {
        return Ok(None);
    }
    if cut_inside_files {
        // Too few files to cut at: cut inside them, at equal widths. Each
        // range's scan skips the row groups and pages outside its range by
        // their statistics, so a file is decoded about once all the same.
        let span = (max as i128) - (min as i128) + 1;
        for i in 1..n {
            let cut = (min as i128 + span * i as i128 / n as i128) as i64;
            if cut > *bounds.last().unwrap() {
                bounds.push(cut);
            }
        }
    } else {
        let mut seen = 0;
        for file in &larger.files {
            let next = bounds.len();
            if next < n && seen * n >= total * next && file.min > *bounds.last().unwrap()
            {
                bounds.push(file.min);
            }
            seen += file.rows;
        }
    }
    // Half-open ranges covering `min..=max`.
    bounds.push(max + 1);
    if bounds.len() < 3 {
        return Ok(None);
    }
    let sort_options = vec![options; ordered];
    let ranges = bounds
        .windows(2)
        .map(|w| {
            let (lo, hi) = (w[0], w[1]);
            let joined = SortMergeJoinExec::try_new(
                left.range(lo, hi, cut_inside_files),
                right.range(lo, hi, cut_inside_files),
                join.on()[..ordered].to_vec(),
                filter.clone(),
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

/// `join`'s filter, with the equalities of its join keys from `ordered` on
/// added to it. Joining on the leading keys and filtering on the others
/// matches the same pairs, for every join type: a pair the filter rejects is
/// unmatched, as a pair whose other keys differ is.
fn keys_into_filter(
    join: &SortMergeJoinExec,
    ordered: usize,
) -> Result<Option<JoinFilter>> {
    let rest = &join.on()[ordered..];
    if rest.is_empty() {
        return Ok(join.filter().clone());
    }
    let (left_schema, right_schema) = (join.left().schema(), join.right().schema());
    let (mut expression, mut column_indices, mut fields) = match join.filter() {
        Some(f) => (
            Some(Arc::clone(f.expression())),
            f.column_indices().to_vec(),
            f.schema().fields().iter().cloned().collect::<Vec<_>>(),
        ),
        None => (None, vec![], vec![]),
    };
    let op = match join.null_equality() {
        NullEquality::NullEqualsNothing => Operator::Eq,
        NullEquality::NullEqualsNull => Operator::IsNotDistinctFrom,
    };
    for (l, r) in rest {
        let (Some(l), Some(r)) = (l.downcast_ref::<Column>(), r.downcast_ref::<Column>())
        else {
            return internal_err!("range split keys must be columns");
        };
        let mut column = |side: JoinSide, index: usize, schema: &SchemaRef| {
            column_indices.push(ColumnIndex { index, side });
            let field = schema.field(index);
            let name = format!("__range_split_key_{}", fields.len());
            fields.push(Arc::new(field.clone().with_name(name.clone())));
            Arc::new(Column::new(&name, fields.len() - 1)) as Arc<dyn PhysicalExpr>
        };
        let lhs = column(JoinSide::Left, l.index(), &left_schema);
        let rhs = column(JoinSide::Right, r.index(), &right_schema);
        let eq = Arc::new(BinaryExpr::new(lhs, op, rhs)) as Arc<dyn PhysicalExpr>;
        expression = Some(match expression {
            Some(e) => Arc::new(BinaryExpr::new(e, Operator::And, eq)),
            None => eq,
        });
    }
    Ok(expression
        .map(|e| JoinFilter::new(e, column_indices, Arc::new(Schema::new(fields)))))
}

#[cfg(feature = "parquet")]
fn is_parquet(config: &FileScanConfig) -> bool {
    config
        .file_source
        .downcast_ref::<datafusion_datasource_parquet::source::ParquetSource>()
        .is_some()
}

#[cfg(not(feature = "parquet"))]
fn is_parquet(_config: &FileScanConfig) -> bool {
    false
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
    /// How many leading join keys each file is ordered on, ascending with
    /// nulls last (`[0]`) and nulls first (`[1]`).
    prefixes: [usize; 2],
    /// In key order, non-overlapping.
    files: Vec<KeyedFile>,
    /// The first key's column in the table schema, for a range predicate.
    table_key: Column,
    key_type: DataType,
    /// Whether the source prunes row groups and pages by a predicate
    /// (Parquet), so a range need not decode the parts of a file outside it.
    prunes_row_groups: bool,
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
        // The files must each be ordered (ascending) on a prefix of the join
        // keys: the longest such prefix, for each null placement.
        let eq = EquivalenceProperties::new_with_orderings(
            Arc::clone(&projected),
            config.output_ordering.clone(),
        );
        let mut prefixes = [0; 2];
        for (nulls_first, prefix) in prefixes.iter_mut().enumerate() {
            let options = SortOptions::new(false, nulls_first == 1);
            *prefix =
                (1..=keys.len())
                    .rev()
                    .find(|&m| {
                        LexOrdering::new(keys[..m].iter().map(|k| {
                            PhysicalSortExpr::new(Arc::new((*k).clone()), options)
                        }))
                        .is_some_and(|required| {
                            eq.ordering_satisfy(required).unwrap_or(false)
                        })
                    })
                    .unwrap_or(0);
        }
        if prefixes == [0, 0] {
            return Ok(None);
        }
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
        let table_schema = config.file_source.table_schema().table_schema();
        let Ok(table_index) = table_schema.index_of(key.name()) else {
            return Ok(None);
        };
        Ok(Some(Self {
            config: config.clone(),
            key: key.index(),
            prefixes,
            files,
            table_key: Column::new(key.name(), table_index),
            key_type: projected.field(key.index()).data_type().clone(),
            prunes_row_groups: is_parquet(config),
        }))
    }

    fn rows(&self) -> usize {
        self.files.iter().map(|f| f.rows).sum()
    }

    /// The scan's Parquet source with `lo <= key < hi` added to its
    /// predicate, so the reader skips row groups and pages outside the range.
    #[cfg(feature = "parquet")]
    fn range_source(&self, lo: i64, hi: i64) -> Option<Arc<dyn FileSource>> {
        use datafusion_datasource_parquet::source::ParquetSource;
        let parquet = self.config.file_source.downcast_ref::<ParquetSource>()?;
        let bound = |v: i64| -> Option<Arc<dyn PhysicalExpr>> {
            let value = ScalarValue::Int64(Some(v)).cast_to(&self.key_type).ok()?;
            Some(Arc::new(Literal::new(value)))
        };
        let key = Arc::new(self.table_key.clone()) as Arc<dyn PhysicalExpr>;
        let range = Arc::new(BinaryExpr::new(
            Arc::new(BinaryExpr::new(
                Arc::clone(&key),
                Operator::GtEq,
                bound(lo)?,
            )),
            Operator::And,
            Arc::new(BinaryExpr::new(key, Operator::Lt, bound(hi)?)),
        )) as Arc<dyn PhysicalExpr>;
        let predicate = match parquet.filter() {
            Some(existing) => {
                Arc::new(BinaryExpr::new(existing, Operator::And, range)) as _
            }
            None => range,
        };
        Some(Arc::new(parquet.with_predicate(predicate)))
    }

    #[cfg(not(feature = "parquet"))]
    fn range_source(&self, _lo: i64, _hi: i64) -> Option<Arc<dyn FileSource>> {
        None
    }

    /// This input's rows with keys in `lo..hi`, skipping row groups and pages
    /// outside the range when `prune`.
    fn range(&self, lo: i64, hi: i64, prune: bool) -> Arc<dyn ExecutionPlan> {
        let files = self
            .files
            .iter()
            .filter(|f| f.max >= lo && f.min < hi)
            .map(|f| f.file.clone())
            .collect();
        let mut builder = FileScanConfigBuilder::from(self.config.clone());
        // Ranges cut at file starts read whole files: the predicate would
        // only cost its evaluation (smj-ts, 8 files: Q8 +5%).
        if let Some(source) = prune.then(|| self.range_source(lo, hi)).flatten() {
            builder = builder.with_source(source);
        }
        let config = builder
            .with_file_groups(vec![FileGroup::new(files)])
            // The ranges already decode in parallel; reading ahead within
            // each adds memory but no speed (smj-ts, 8 ranges: all 9
            // queries within noise; pool peak on Q4 122 MB with it, 7 MB
            // without).
            .with_lookahead(false)
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
