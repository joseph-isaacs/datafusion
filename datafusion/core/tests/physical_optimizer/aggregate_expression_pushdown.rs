// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

//! Tests for the aggregate expression pushdown performed by the
//! [`ProjectionPushdown`] physical optimizer rule.
//!
//! Note these tests are not in the same module as the optimizer pass because
//! they rely on `DataSourceExec` which is in the core crate.

use std::sync::Arc;

use arrow::datatypes::{DataType, Field, Schema, SchemaRef};
use datafusion::datasource::listing::PartitionedFile;
use datafusion::datasource::memory::MemorySourceConfig;
use datafusion::datasource::physical_plan::CsvSource;
use datafusion::datasource::source::DataSourceExec;
use datafusion_common::config::{ConfigOptions, CsvOptions};
use datafusion_common::{Result, ScalarValue};
use datafusion_datasource::file_scan_config::FileScanConfigBuilder;
use datafusion_execution::object_store::ObjectStoreUrl;
use datafusion_expr::Operator;
use datafusion_functions::math::random;
use datafusion_functions_aggregate::count::count_udaf;
use datafusion_functions_aggregate::sum::sum_udaf;
use datafusion_physical_expr::Partitioning;
use datafusion_physical_expr::ScalarFunctionExpr;
use datafusion_physical_expr::aggregate::{AggregateExprBuilder, AggregateFunctionExpr};
use datafusion_physical_expr::expressions::{binary, col, lit};
use datafusion_physical_expr_common::physical_expr::PhysicalExpr;
use datafusion_physical_optimizer::PhysicalOptimizerRule;
use datafusion_physical_optimizer::projection_pushdown::ProjectionPushdown;
use datafusion_physical_plan::aggregates::{
    AggregateExec, AggregateMode, PhysicalGroupBy,
};
use datafusion_physical_plan::coop::CooperativeExec;
use datafusion_physical_plan::filter::FilterExec;
use datafusion_physical_plan::repartition::RepartitionExec;
use datafusion_physical_plan::{ExecutionPlan, displayable};
use insta::assert_snapshot;

fn schema() -> SchemaRef {
    Arc::new(Schema::new(vec![
        Field::new("a", DataType::Int64, true),
        Field::new("b", DataType::Int64, true),
        Field::new("c", DataType::Int64, true),
    ]))
}

/// A source that absorbs arbitrary projection expressions.
fn csv_exec(schema: &SchemaRef) -> Arc<dyn ExecutionPlan> {
    let options = CsvOptions {
        has_header: Some(false),
        delimiter: 0,
        quote: 0,
        ..Default::default()
    };
    let config = FileScanConfigBuilder::new(
        ObjectStoreUrl::parse("test:///").unwrap(),
        Arc::new(CsvSource::new(Arc::clone(schema)).with_csv_options(options)),
    )
    .with_file(PartitionedFile::new("x", 100))
    .build();
    DataSourceExec::from_data_source(config)
}

/// A source that only absorbs plain column projections.
fn memory_exec(schema: &SchemaRef) -> Arc<dyn ExecutionPlan> {
    MemorySourceConfig::try_new_exec(&[], Arc::clone(schema), None).unwrap()
}

/// `a + 1`
fn a_plus_one(schema: &Schema) -> Arc<dyn PhysicalExpr> {
    binary(col("a", schema).unwrap(), Operator::Plus, lit(1i64), schema).unwrap()
}

/// `b * 2`
fn b_times_two(schema: &Schema) -> Arc<dyn PhysicalExpr> {
    binary(
        col("b", schema).unwrap(),
        Operator::Multiply,
        lit(2i64),
        schema,
    )
    .unwrap()
}

fn sum_of(
    expr: Arc<dyn PhysicalExpr>,
    name: &str,
    schema: &SchemaRef,
) -> Arc<AggregateFunctionExpr> {
    Arc::new(
        AggregateExprBuilder::new(sum_udaf(), vec![expr])
            .schema(Arc::clone(schema))
            .alias(name)
            .build()
            .unwrap(),
    )
}

fn count_of(
    expr: Arc<dyn PhysicalExpr>,
    name: &str,
    schema: &SchemaRef,
) -> Arc<AggregateFunctionExpr> {
    Arc::new(
        AggregateExprBuilder::new(count_udaf(), vec![expr])
            .schema(Arc::clone(schema))
            .alias(name)
            .build()
            .unwrap(),
    )
}

/// `AggregateExec: gby=[a + 1 as k], aggr=[sum(b * 2), count(c)]`
fn aggregate(
    mode: AggregateMode,
    input: Arc<dyn ExecutionPlan>,
    filter_expr: Option<Arc<dyn PhysicalExpr>>,
) -> Result<Arc<dyn ExecutionPlan>> {
    let schema = input.schema();
    let group_by =
        PhysicalGroupBy::new_single(vec![(a_plus_one(&schema), "k".to_string())]);
    let aggr_expr = vec![
        sum_of(b_times_two(&schema), "sum(b * 2)", &schema),
        count_of(col("c", &schema)?, "count(c)", &schema),
    ];
    Ok(Arc::new(AggregateExec::try_new(
        mode,
        group_by,
        aggr_expr,
        vec![filter_expr, None],
        input,
        schema,
    )?))
}

fn optimize(plan: Arc<dyn ExecutionPlan>, enabled: bool) -> Result<String> {
    let mut config = ConfigOptions::new();
    config.optimizer.enable_aggregate_expression_pushdown = enabled;
    let optimized = ProjectionPushdown::new().optimize(plan, &config)?;
    Ok(displayable(optimized.as_ref())
        .indent(true)
        .to_string()
        .trim()
        .to_string())
}

#[test]
fn partial_aggregate_over_absorbing_source() -> Result<()> {
    let schema = schema();
    let plan = aggregate(AggregateMode::Partial, csv_exec(&schema), None)?;
    let original_schema = plan.schema();

    assert_snapshot!(optimize(Arc::clone(&plan), false)?, @r"
    AggregateExec: mode=Partial, gby=[a@0 + 1 as k], aggr=[sum(b * 2), count(c)]
      DataSourceExec: file_groups={1 group: [[x]]}, projection=[a, b, c], file_type=csv, has_header=false
    ");

    let optimized = ProjectionPushdown::new().optimize(plan, &{
        let mut config = ConfigOptions::new();
        config.optimizer.enable_aggregate_expression_pushdown = true;
        config
    })?;
    assert_eq!(optimized.schema(), original_schema);
    assert_snapshot!(displayable(optimized.as_ref()).indent(true).to_string().trim(), @r"
    AggregateExec: mode=Partial, gby=[__datafusion_agg_expr_1@1 as k], aggr=[sum(b * 2), count(c)]
      DataSourceExec: file_groups={1 group: [[x]]}, projection=[c, a@0 + 1 as __datafusion_agg_expr_1, b@1 * 2 as __datafusion_agg_expr_2], file_type=csv, has_header=false
    ");
    Ok(())
}

#[test]
fn single_aggregate_with_filter_and_cooperative_exec() -> Result<()> {
    let schema = schema();
    let input: Arc<dyn ExecutionPlan> = Arc::new(CooperativeExec::new(csv_exec(&schema)));
    // FILTER (WHERE c > 0) references a pass-through column that changes index.
    let filter = binary(col("c", &schema)?, Operator::Gt, lit(0i64), &schema)?;
    let plan = aggregate(AggregateMode::Single, input, Some(filter))?;

    assert_snapshot!(optimize(Arc::clone(&plan), false)?, @r"
    AggregateExec: mode=Single, gby=[a@0 + 1 as k], aggr=[sum(b * 2), count(c)]
      CooperativeExec
        DataSourceExec: file_groups={1 group: [[x]]}, projection=[a, b, c], file_type=csv, has_header=false
    ");
    assert_snapshot!(optimize(plan, true)?, @r"
    AggregateExec: mode=Single, gby=[__datafusion_agg_expr_1@1 as k], aggr=[sum(b * 2), count(c)]
      CooperativeExec
        DataSourceExec: file_groups={1 group: [[x]]}, projection=[c, a@0 + 1 as __datafusion_agg_expr_1, b@1 * 2 as __datafusion_agg_expr_2], file_type=csv, has_header=false
    ");
    Ok(())
}

#[test]
fn round_robin_repartition_between_aggregate_and_source_is_crossed() -> Result<()> {
    let schema = schema();
    let input: Arc<dyn ExecutionPlan> = Arc::new(RepartitionExec::try_new(
        csv_exec(&schema),
        Partitioning::RoundRobinBatch(4),
    )?);
    let plan = aggregate(AggregateMode::Partial, input, None)?;
    assert_snapshot!(optimize(plan, true)?, @r"
    AggregateExec: mode=Partial, gby=[__datafusion_agg_expr_1@1 as k], aggr=[sum(b * 2), count(c)]
      RepartitionExec: partitioning=RoundRobinBatch(4), input_partitions=1
        DataSourceExec: file_groups={1 group: [[x]]}, projection=[c, a@0 + 1 as __datafusion_agg_expr_1, b@1 * 2 as __datafusion_agg_expr_2], file_type=csv, has_header=false
    ");
    Ok(())
}

#[test]
fn hash_repartition_between_aggregate_and_source_blocks_pushdown() -> Result<()> {
    let schema = schema();
    let input: Arc<dyn ExecutionPlan> = Arc::new(RepartitionExec::try_new(
        csv_exec(&schema),
        Partitioning::Hash(vec![col("a", &schema)?], 4),
    )?);
    let plan = aggregate(AggregateMode::Partial, input, None)?;
    assert_snapshot!(optimize(plan, true)?, @r"
    AggregateExec: mode=Partial, gby=[a@0 + 1 as k], aggr=[sum(b * 2), count(c)]
      RepartitionExec: partitioning=Hash([a@0], 4), input_partitions=1
        DataSourceExec: file_groups={1 group: [[x]]}, projection=[a, b, c], file_type=csv, has_header=false
    ");
    Ok(())
}

#[test]
fn source_that_does_not_absorb_expressions_is_left_alone() -> Result<()> {
    let schema = schema();
    let plan = aggregate(AggregateMode::Partial, memory_exec(&schema), None)?;
    assert_snapshot!(optimize(plan, true)?, @r"
    AggregateExec: mode=Partial, gby=[a@0 + 1 as k], aggr=[sum(b * 2), count(c)]
      DataSourceExec: partitions=0, partition_sizes=[]
    ");
    Ok(())
}

#[test]
fn filter_between_aggregate_and_source_blocks_pushdown() -> Result<()> {
    let schema = schema();
    let predicate = binary(col("c", &schema)?, Operator::Gt, lit(0i64), &schema)?;
    let input: Arc<dyn ExecutionPlan> =
        Arc::new(FilterExec::try_new(predicate, csv_exec(&schema))?);
    let plan = aggregate(AggregateMode::Partial, input, None)?;
    assert_snapshot!(optimize(plan, true)?, @r"
    AggregateExec: mode=Partial, gby=[a@0 + 1 as k], aggr=[sum(b * 2), count(c)]
      FilterExec: c@2 > 0
        DataSourceExec: file_groups={1 group: [[x]]}, projection=[a, b, c], file_type=csv, has_header=false
    ");
    Ok(())
}

#[test]
fn final_aggregate_is_left_alone() -> Result<()> {
    let schema = schema();
    let plan = aggregate(AggregateMode::Final, csv_exec(&schema), None)?;
    assert_snapshot!(optimize(plan, true)?, @r"
    AggregateExec: mode=Final, gby=[a@0 + 1 as k], aggr=[sum(b * 2), count(c)]
      DataSourceExec: file_groups={1 group: [[x]]}, projection=[a, b, c], file_type=csv, has_header=false
    ");
    Ok(())
}

#[test]
fn volatile_and_column_only_arguments_are_left_alone() -> Result<()> {
    let schema = schema();
    let input = csv_exec(&schema);
    let random = Arc::new(ScalarFunctionExpr::try_new(
        random(),
        vec![],
        &schema,
        Arc::new(ConfigOptions::default()),
    )?);
    let a_plus_random = binary(col("a", &schema)?, Operator::Plus, random, &schema)?;
    let aggr_expr = vec![
        sum_of(a_plus_random, "sum(a + random())", &schema),
        sum_of(col("b", &schema)?, "sum(b)", &schema),
        count_of(lit(ScalarValue::Int64(Some(1))), "count(1)", &schema),
    ];
    let plan: Arc<dyn ExecutionPlan> = Arc::new(AggregateExec::try_new(
        AggregateMode::Partial,
        PhysicalGroupBy::default(),
        aggr_expr,
        vec![None, None, None],
        input,
        Arc::clone(&schema),
    )?);
    assert_snapshot!(optimize(plan, true)?, @r"
    AggregateExec: mode=Partial, gby=[], aggr=[sum(a + random()), sum(b), count(1)]
      DataSourceExec: file_groups={1 group: [[x]]}, projection=[a, b, c], file_type=csv, has_header=false
    ");
    Ok(())
}
