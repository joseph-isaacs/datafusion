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

//! This file implements the `ProjectionPushdown` physical optimization rule.
//! The function [`remove_unnecessary_projections`] tries to push down all
//! projections one by one if the operator below is amenable to this. If a
//! projection reaches a source, it can even disappear from the plan entirely.

use crate::PhysicalOptimizerRule;
use arrow::datatypes::{Fields, Schema, SchemaRef};
use datafusion_common::alias::AliasGenerator;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use datafusion_common::config::ConfigOptions;
use datafusion_common::tree_node::{
    Transformed, TransformedResult, TreeNode, TreeNodeRecursion,
};
use datafusion_common::{JoinSide, JoinType, Result};
use datafusion_physical_expr::aggregate::AggregateExprBuilder;
use datafusion_physical_expr::expressions::{Column, Literal};
use datafusion_physical_expr::utils::collect_columns;
use datafusion_physical_expr_common::physical_expr::{PhysicalExpr, is_volatile};
use datafusion_physical_expr_common::sort_expr::PhysicalSortExpr;
use datafusion_physical_plan::ExecutionPlan;
use datafusion_physical_plan::Partitioning;
use datafusion_physical_plan::aggregates::{
    AggregateExec, AggregateMode, PhysicalGroupBy,
};
use datafusion_physical_plan::coop::CooperativeExec;
use datafusion_physical_plan::joins::NestedLoopJoinExec;
use datafusion_physical_plan::joins::utils::{ColumnIndex, JoinFilter};
use datafusion_physical_plan::projection::{
    ProjectionExec, ProjectionExpr, remove_unnecessary_projections,
};
use datafusion_physical_plan::repartition::RepartitionExec;

/// This rule inspects `ProjectionExec`'s in the given physical plan and tries to
/// remove or swap with its child.
///
/// Furthermore, tries to push down projections from nested loop join filters that only depend on
/// one side of the join. By pushing these projections down, functions that only depend on one side
/// of the join must be evaluated for the cartesian product of the two sides.
///
/// When `datafusion.optimizer.enable_aggregate_expression_pushdown` is set, the
/// rule also extracts the scalar expressions used as aggregate arguments and
/// group by keys into a projection below the aggregate and pushes that
/// projection into the data source, keeping the rewrite only when the source
/// absorbs the projection entirely.
#[derive(Default, Debug)]
pub struct ProjectionPushdown {}

impl ProjectionPushdown {
    #[expect(missing_docs)]
    pub fn new() -> Self {
        Self {}
    }
}

impl PhysicalOptimizerRule for ProjectionPushdown {
    fn optimize(
        &self,
        plan: Arc<dyn ExecutionPlan>,
        config: &ConfigOptions,
    ) -> Result<Arc<dyn ExecutionPlan>> {
        let alias_generator = AliasGenerator::new();
        let plan = plan
            .transform_up(|plan| match plan.downcast_ref::<NestedLoopJoinExec>() {
                None => Ok(Transformed::no(plan)),
                Some(hash_join) => try_push_down_join_filter(
                    Arc::clone(&plan),
                    hash_join,
                    &alias_generator,
                ),
            })
            .map(|t| t.data)?;

        let plan = if config.optimizer.enable_aggregate_expression_pushdown {
            plan.transform_up(|plan| {
                try_push_down_aggregate_expressions(plan, &alias_generator)
            })
            .data()?
        } else {
            plan
        };

        plan.transform_down(remove_unnecessary_projections).data()
    }

    fn name(&self) -> &str {
        "ProjectionPushdown"
    }

    fn schema_check(&self) -> bool {
        true
    }
}

/// Tries to push down parts of the filter.
///
/// See [JoinFilterRewriter] for details.
fn try_push_down_join_filter(
    original_plan: Arc<dyn ExecutionPlan>,
    join: &NestedLoopJoinExec,
    alias_generator: &AliasGenerator,
) -> Result<Transformed<Arc<dyn ExecutionPlan>>> {
    // Mark joins are currently not supported.
    if matches!(join.join_type(), JoinType::LeftMark | JoinType::RightMark) {
        return Ok(Transformed::no(original_plan));
    }

    let projections = join.projection();
    let Some(filter) = join.filter() else {
        return Ok(Transformed::no(original_plan));
    };

    let original_lhs_length = join.left().schema().fields().len();
    let original_rhs_length = join.right().schema().fields().len();

    let lhs_rewrite = try_push_down_projection(
        Arc::clone(&join.right().schema()),
        Arc::clone(join.left()),
        JoinSide::Left,
        filter.clone(),
        alias_generator,
    )?;
    let rhs_rewrite = try_push_down_projection(
        Arc::clone(&lhs_rewrite.data.0.schema()),
        Arc::clone(join.right()),
        JoinSide::Right,
        lhs_rewrite.data.1,
        alias_generator,
    )?;
    if !lhs_rewrite.transformed && !rhs_rewrite.transformed {
        return Ok(Transformed::no(original_plan));
    }

    let join_filter = minimize_join_filter(
        Arc::clone(rhs_rewrite.data.1.expression()),
        rhs_rewrite.data.1.column_indices(),
        lhs_rewrite.data.0.schema().as_ref(),
        rhs_rewrite.data.0.schema().as_ref(),
    );

    let new_lhs_length = lhs_rewrite.data.0.schema().fields.len();
    let projections = match projections.as_ref() {
        None => match join.join_type() {
            JoinType::Inner | JoinType::Left | JoinType::Right | JoinType::Full => {
                // Build projections that ignore the newly projected columns.
                let mut projections = Vec::new();
                projections.extend(0..original_lhs_length);
                projections.extend(new_lhs_length..new_lhs_length + original_rhs_length);
                projections
            }
            JoinType::LeftSemi | JoinType::LeftAnti => {
                // Only return original left columns
                let mut projections = Vec::new();
                projections.extend(0..original_lhs_length);
                projections
            }
            JoinType::RightSemi | JoinType::RightAnti => {
                // Only return original right columns
                let mut projections = Vec::new();
                projections.extend(0..original_rhs_length);
                projections
            }
            _ => unreachable!("Unsupported join type"),
        },
        Some(projections) => {
            let rhs_offset = new_lhs_length - original_lhs_length;
            projections
                .iter()
                .map(|idx| {
                    if *idx >= original_lhs_length {
                        idx + rhs_offset
                    } else {
                        *idx
                    }
                })
                .collect()
        }
    };

    Ok(Transformed::yes(Arc::new(NestedLoopJoinExec::try_new(
        lhs_rewrite.data.0,
        rhs_rewrite.data.0,
        Some(join_filter),
        join.join_type(),
        Some(projections),
    )?)))
}

/// Tries to push down parts of `expr` into the `join_side`.
fn try_push_down_projection(
    other_schema: SchemaRef,
    plan: Arc<dyn ExecutionPlan>,
    join_side: JoinSide,
    join_filter: JoinFilter,
    alias_generator: &AliasGenerator,
) -> Result<Transformed<(Arc<dyn ExecutionPlan>, JoinFilter)>> {
    let expr = Arc::clone(join_filter.expression());
    let original_plan_schema = plan.schema();
    let mut rewriter = JoinFilterRewriter::new(
        join_side,
        original_plan_schema.as_ref(),
        join_filter.column_indices().to_vec(),
        alias_generator,
    );
    let new_expr = rewriter.rewrite(expr)?;

    if new_expr.transformed {
        let new_join_side =
            ProjectionExec::try_new(rewriter.join_side_projections, plan)?;
        let new_schema = Arc::clone(&new_join_side.schema());

        let (lhs_schema, rhs_schema) = match join_side {
            JoinSide::Left => (new_schema, other_schema),
            JoinSide::Right => (other_schema, new_schema),
            JoinSide::None => unreachable!("Mark join not supported"),
        };
        let intermediate_schema = rewriter
            .intermediate_column_indices
            .iter()
            .map(|ci| match ci.side {
                JoinSide::Left => Arc::clone(&lhs_schema.fields[ci.index]),
                JoinSide::Right => Arc::clone(&rhs_schema.fields[ci.index]),
                JoinSide::None => unreachable!("Mark join not supported"),
            })
            .collect::<Fields>();

        let join_filter = JoinFilter::new(
            new_expr.data,
            rewriter.intermediate_column_indices,
            Arc::new(Schema::new(intermediate_schema)),
        );
        Ok(Transformed::yes((Arc::new(new_join_side), join_filter)))
    } else {
        Ok(Transformed::no((plan, join_filter)))
    }
}

/// Creates a new [JoinFilter] and tries to minimize the internal schema.
///
/// This could eliminate some columns that were only part of a computation that has been pushed
/// down. As this computation is now materialized on one side of the join, the original input
/// columns are not needed anymore.
fn minimize_join_filter(
    expr: Arc<dyn PhysicalExpr>,
    old_column_indices: &[ColumnIndex],
    lhs_schema: &Schema,
    rhs_schema: &Schema,
) -> JoinFilter {
    let mut used_columns = HashSet::new();
    expr.apply(|expr| {
        if let Some(col) = expr.downcast_ref::<Column>() {
            used_columns.insert(col.index());
        }
        Ok(TreeNodeRecursion::Continue)
    })
    .expect("Closure cannot fail");

    let new_column_indices = old_column_indices
        .iter()
        .enumerate()
        .filter(|(idx, _)| used_columns.contains(idx))
        .map(|(_, ci)| ci.clone())
        .collect::<Vec<_>>();
    let fields = new_column_indices
        .iter()
        .map(|ci| match ci.side {
            JoinSide::Left => lhs_schema.field(ci.index).clone(),
            JoinSide::Right => rhs_schema.field(ci.index).clone(),
            JoinSide::None => unreachable!("Mark join not supported"),
        })
        .collect::<Fields>();

    let final_expr = expr
        .transform_up(|expr| match expr.downcast_ref::<Column>() {
            None => Ok(Transformed::no(expr)),
            Some(column) => {
                let new_idx = used_columns
                    .iter()
                    .filter(|idx| **idx < column.index())
                    .count();
                let new_column = Column::new(column.name(), new_idx);
                Ok(Transformed::yes(
                    Arc::new(new_column) as Arc<dyn PhysicalExpr>
                ))
            }
        })
        .expect("Closure cannot fail");

    JoinFilter::new(
        final_expr.data,
        new_column_indices,
        Arc::new(Schema::new(fields)),
    )
}

/// Implements the push-down machinery.
///
/// The rewriter starts at the top of the filter expression and traverses the expression tree. For
/// each (sub-)expression, the rewriter checks whether it only refers to one side of the join. If
/// this is never the case, no subexpressions of the filter can be pushed down. If there is a
/// subexpression that can be computed using only one side of the join, the entire subexpression is
/// pushed down to the join side.
struct JoinFilterRewriter<'a> {
    join_side: JoinSide,
    join_side_schema: &'a Schema,
    join_side_projections: Vec<(Arc<dyn PhysicalExpr>, String)>,
    intermediate_column_indices: Vec<ColumnIndex>,
    alias_generator: &'a AliasGenerator,
}

impl<'a> JoinFilterRewriter<'a> {
    /// Creates a new [JoinFilterRewriter].
    fn new(
        join_side: JoinSide,
        join_side_schema: &'a Schema,
        column_indices: Vec<ColumnIndex>,
        alias_generator: &'a AliasGenerator,
    ) -> Self {
        let projections = join_side_schema
            .fields()
            .iter()
            .enumerate()
            .map(|(idx, field)| {
                (
                    Arc::new(Column::new(field.name(), idx)) as Arc<dyn PhysicalExpr>,
                    field.name().to_string(),
                )
            })
            .collect();

        Self {
            join_side,
            join_side_schema,
            join_side_projections: projections,
            intermediate_column_indices: column_indices,
            alias_generator,
        }
    }

    /// Executes the push-down machinery on `expr`.
    ///
    /// See the [JoinFilterRewriter] for further information.
    fn rewrite(
        &mut self,
        expr: Arc<dyn PhysicalExpr>,
    ) -> Result<Transformed<Arc<dyn PhysicalExpr>>> {
        let depends_on_this_side = self.depends_on_join_side(&expr, self.join_side)?;
        // We don't push down things that do not depend on this side (other side or no side).
        if !depends_on_this_side {
            return Ok(Transformed::no(expr));
        }

        // Recurse if there is a dependency to both sides or if the entire expression is volatile.
        let depends_on_other_side =
            self.depends_on_join_side(&expr, self.join_side.negate())?;
        if depends_on_other_side || is_volatile(&expr) {
            return expr.map_children(|expr| self.rewrite(expr));
        }

        // There is only a dependency on this side.

        // If this expression has no children, we do not push down, as it should already be a column
        // reference.
        if expr.children().is_empty() {
            return Ok(Transformed::no(expr));
        }

        // Otherwise, we push down a projection.
        let alias = self.alias_generator.next("join_proj_push_down");
        let idx = self.create_new_column(alias.clone(), expr)?;

        Ok(Transformed::yes(
            Arc::new(Column::new(&alias, idx)) as Arc<dyn PhysicalExpr>
        ))
    }

    /// Creates a new column in the current join side.
    fn create_new_column(
        &mut self,
        name: String,
        expr: Arc<dyn PhysicalExpr>,
    ) -> Result<usize> {
        // First, add a new projection. The expression must be rewritten, as it is no longer
        // executed against the filter schema.
        let new_idx = self.join_side_projections.len();
        let rewritten_expr = expr.transform_up(|expr| {
            Ok(match expr.downcast_ref::<Column>() {
                None => Transformed::no(expr),
                Some(column) => {
                    let intermediate_column =
                        &self.intermediate_column_indices[column.index()];
                    assert_eq!(intermediate_column.side, self.join_side);

                    let join_side_index = intermediate_column.index;
                    let field = self.join_side_schema.field(join_side_index);
                    let new_column = Column::new(field.name(), join_side_index);
                    Transformed::yes(Arc::new(new_column) as Arc<dyn PhysicalExpr>)
                }
            })
        })?;
        self.join_side_projections.push((rewritten_expr.data, name));

        // Then, update the column indices
        let new_intermediate_idx = self.intermediate_column_indices.len();
        let idx = ColumnIndex {
            index: new_idx,
            side: self.join_side,
        };
        self.intermediate_column_indices.push(idx);

        Ok(new_intermediate_idx)
    }

    /// Checks whether the entire expression depends on the given `join_side`.
    fn depends_on_join_side(
        &mut self,
        expr: &Arc<dyn PhysicalExpr>,
        join_side: JoinSide,
    ) -> Result<bool> {
        let mut result = false;
        expr.apply(|expr| match expr.downcast_ref::<Column>() {
            None => Ok(TreeNodeRecursion::Continue),
            Some(c) => {
                let column_index = &self.intermediate_column_indices[c.index()];
                if column_index.side == join_side {
                    result = true;
                    return Ok(TreeNodeRecursion::Stop);
                }
                Ok(TreeNodeRecursion::Continue)
            }
        })?;

        Ok(result)
    }
}

/// Prefix for the aliases of aggregate input expressions that
/// [`try_push_down_aggregate_expressions`] extracts into a projection.
const AGGREGATE_EXPR_ALIAS_PREFIX: &str = "__datafusion_agg_expr";

/// Tries to move the scalar expressions that an [`AggregateExec`] evaluates on
/// its input (aggregate arguments and group by keys) into the data source.
///
/// Starting from `aggregate <- source`, where the aggregate computes e.g.
/// `avg(octet_length(url))`, the rule builds
/// `aggregate' <- projection <- source` with `projection` computing
/// `octet_length(url)` and `aggregate'` referencing the projected column, and
/// then lets the regular projection pushdown
/// ([`remove_unnecessary_projections`]) absorb `projection` into the source.
///
/// The rewrite is speculative and does not rely on a cost model. It is only
/// attempted when only cardinality preserving operators ([`CooperativeExec`]
/// and round robin [`RepartitionExec`]) sit between the aggregate and the
/// source, and it is only kept when the source absorbs the projection
/// entirely, so no `ProjectionExec` is left in the plan. Together these
/// guarantee that the extracted expressions are evaluated on exactly the rows
/// they were evaluated on before, regardless of how expensive they are. If
/// either condition fails, the original aggregate is returned unchanged.
///
/// The projection is inserted directly above the source rather than directly
/// below the aggregate: the regular pushdown refuses to move computed
/// projections below a repartition, while the aggregate's argument
/// expressions are already evaluated once per input row on either side of a
/// round robin repartition. A source that accepts an expression projection is
/// expected to evaluate it efficiently (typically it is parallel internally,
/// which is why it did not repartition itself).
///
/// Only `Partial`, `Single` and `SinglePartitioned` aggregates are rewritten:
/// `Final` aggregates consume intermediate state rather than evaluating their
/// arguments. Aggregates that produce a dynamic filter are left alone as well.
fn try_push_down_aggregate_expressions(
    plan: Arc<dyn ExecutionPlan>,
    alias_generator: &AliasGenerator,
) -> Result<Transformed<Arc<dyn ExecutionPlan>>> {
    let Some(aggregate) = plan.downcast_ref::<AggregateExec>() else {
        return Ok(Transformed::no(plan));
    };
    if !matches!(
        aggregate.mode(),
        AggregateMode::Partial | AggregateMode::Single | AggregateMode::SinglePartitioned
    ) {
        return Ok(Transformed::no(plan));
    }

    // A dynamic filter produced by the aggregate (e.g. for `max(f(x))`) may
    // already be referenced by the source in terms of the current arguments.
    // Rebuilding the aggregate would detach it, so leave such plans alone.
    if !plan.dynamic_expressions_produced().is_empty() {
        return Ok(Transformed::no(plan));
    }

    let input = aggregate.input();
    let input_schema = input.schema();

    // Only worth trying when nothing between the aggregate and the source
    // reduces the number of rows; the operators crossed preserve the schema,
    // so the aggregate's expressions can be evaluated directly on the leaf.
    let Some(leaf) = cardinality_preserving_leaf(input) else {
        return Ok(Transformed::no(plan));
    };

    // Collect the distinct expressions worth extracting.
    let mut extracted: Vec<Arc<dyn PhysicalExpr>> = vec![];
    let mut consider = |expr: &Arc<dyn PhysicalExpr>| {
        if is_extractable_aggregate_input(expr) && !extracted.contains(expr) {
            extracted.push(Arc::clone(expr));
        }
    };
    for (expr, _) in aggregate.group_expr().expr() {
        consider(expr);
    }
    for aggr_expr in aggregate.aggr_expr() {
        for arg in aggr_expr.expressions() {
            consider(&arg);
        }
    }
    if extracted.is_empty() {
        return Ok(Transformed::no(plan));
    }

    // Columns the aggregate still needs after the extracted expressions have
    // been replaced by references to the projected columns.
    let mut needed_columns: Vec<Column> = vec![];
    let mut collect_needed = |expr: &Arc<dyn PhysicalExpr>| -> Result<()> {
        expr.apply(|node| {
            if extracted.contains(node) {
                return Ok(TreeNodeRecursion::Jump);
            }
            if let Some(column) = node.downcast_ref::<Column>()
                && !needed_columns.contains(column)
            {
                needed_columns.push(column.clone());
            }
            Ok(TreeNodeRecursion::Continue)
        })
        .map(|_| ())
    };
    for (expr, _) in aggregate.group_expr().expr() {
        collect_needed(expr)?;
    }
    for aggr_expr in aggregate.aggr_expr() {
        for arg in aggr_expr.expressions() {
            collect_needed(&arg)?;
        }
        for sort_expr in aggr_expr.order_bys() {
            collect_needed(&sort_expr.expr)?;
        }
    }
    for filter in aggregate.filter_expr().iter().flatten() {
        collect_needed(filter)?;
    }
    needed_columns.sort_by_key(|column| column.index());

    // Build the projection: pass-through columns first, extracted expressions
    // after them under fresh aliases.
    let mut column_map: HashMap<usize, usize> = HashMap::new();
    let mut projection_exprs: Vec<ProjectionExpr> = vec![];
    for column in &needed_columns {
        column_map.insert(column.index(), projection_exprs.len());
        projection_exprs
            .push(ProjectionExpr::new(Arc::new(column.clone()), column.name()));
    }
    let extracted_offset = projection_exprs.len();
    for expr in &extracted {
        let alias = loop {
            let alias = alias_generator.next(AGGREGATE_EXPR_ALIAS_PREFIX);
            if input_schema.index_of(&alias).is_err() {
                break alias;
            }
        };
        projection_exprs.push(ProjectionExpr::new(Arc::clone(expr), alias));
    }
    let projection: Arc<dyn ExecutionPlan> =
        Arc::new(ProjectionExec::try_new(projection_exprs, Arc::clone(leaf))?);
    let projected_schema = projection.schema();

    // Let the regular projection pushdown absorb the projection into the
    // source; if a `ProjectionExec` survives, the source declined.
    let new_leaf = remove_unnecessary_projections(projection)?.data;
    if new_leaf.is::<ProjectionExec>() {
        return Ok(Transformed::no(plan));
    }
    let new_input = Arc::clone(input)
        .transform_up(|node| {
            if node.children().is_empty() {
                Ok(Transformed::yes(Arc::clone(&new_leaf)))
            } else {
                Ok(Transformed::no(node))
            }
        })
        .data()?;

    let rewrite = |expr: &Arc<dyn PhysicalExpr>| -> Result<Arc<dyn PhysicalExpr>> {
        Arc::clone(expr)
            .transform_down(|node| {
                if let Some(position) = extracted.iter().position(|e| e == &node) {
                    let index = extracted_offset + position;
                    let column = Column::new(projected_schema.field(index).name(), index);
                    return Ok(Transformed::new(
                        Arc::new(column) as _,
                        true,
                        TreeNodeRecursion::Jump,
                    ));
                }
                if let Some(column) = node.downcast_ref::<Column>() {
                    let index = column_map[&column.index()];
                    return Ok(Transformed::yes(
                        Arc::new(Column::new(column.name(), index)) as _,
                    ));
                }
                Ok(Transformed::no(node))
            })
            .data()
    };

    let group_by = aggregate.group_expr();
    let new_group_exprs = group_by
        .expr()
        .iter()
        .map(|(expr, name)| Ok((rewrite(expr)?, name.clone())))
        .collect::<Result<Vec<_>>>()?;
    let new_group_by = PhysicalGroupBy::new(
        new_group_exprs,
        group_by.null_expr().to_vec(),
        group_by.groups().to_vec(),
        !group_by.is_single(),
    );

    let mut new_aggr_exprs = Vec::with_capacity(aggregate.aggr_expr().len());
    for aggr_expr in aggregate.aggr_expr() {
        let args = aggr_expr
            .expressions()
            .iter()
            .map(&rewrite)
            .collect::<Result<Vec<_>>>()?;
        let order_bys = aggr_expr
            .order_bys()
            .iter()
            .map(|sort_expr| {
                Ok(PhysicalSortExpr::new(
                    rewrite(&sort_expr.expr)?,
                    sort_expr.options,
                ))
            })
            .collect::<Result<Vec<_>>>()?;
        let mut builder =
            AggregateExprBuilder::new(Arc::new(aggr_expr.fun().clone()), args)
                .schema(Arc::clone(&projected_schema))
                .alias(aggr_expr.name())
                .order_by(order_bys)
                .with_ignore_nulls(aggr_expr.ignore_nulls())
                .with_distinct(aggr_expr.is_distinct())
                .with_reversed(aggr_expr.is_reversed());
        if let Some(human_display) = aggr_expr.human_display() {
            builder = builder.human_display(human_display);
        }
        if let Some(alias) = aggr_expr.human_display_alias() {
            builder = builder.human_display_alias(alias);
        }
        new_aggr_exprs.push(Arc::new(builder.build()?));
    }

    let new_filter_exprs = aggregate
        .filter_expr()
        .iter()
        .map(|filter| filter.as_ref().map(&rewrite).transpose())
        .collect::<Result<Vec<_>>>()?;

    let new_aggregate = AggregateExec::try_new(
        *aggregate.mode(),
        new_group_by,
        new_aggr_exprs,
        new_filter_exprs,
        new_input,
        projected_schema,
    )?
    .with_limit_options(aggregate.limit_options());

    // The rewrite must be invisible to the parent operators, and must not
    // downgrade a streaming aggregate to a hash aggregate because the source
    // no longer advertises the ordering of the extracted expressions.
    if new_aggregate.schema() != aggregate.schema()
        || new_aggregate.input_order_mode() != aggregate.input_order_mode()
    {
        return Ok(Transformed::no(plan));
    }

    Ok(Transformed::yes(Arc::new(new_aggregate)))
}

/// Returns `true` if `expr` is worth extracting out of an aggregate: it does
/// some computation over at least one input column and re-evaluating it
/// elsewhere in the plan cannot change its result.
fn is_extractable_aggregate_input(expr: &Arc<dyn PhysicalExpr>) -> bool {
    !expr.is::<Column>()
        && !expr.is::<Literal>()
        && !is_volatile(expr)
        && !collect_columns(expr).is_empty()
}

/// Returns the leaf below `plan` if every operator on the way to it preserves
/// both the cardinality and the schema of its input.
fn cardinality_preserving_leaf(
    plan: &Arc<dyn ExecutionPlan>,
) -> Option<&Arc<dyn ExecutionPlan>> {
    let mut current = plan;
    loop {
        let children = current.children();
        if children.is_empty() {
            return Some(current);
        }
        let is_round_robin_repartition = current
            .downcast_ref::<RepartitionExec>()
            .is_some_and(|repartition| {
                matches!(repartition.partitioning(), Partitioning::RoundRobinBatch(_))
            });
        if !(current.is::<CooperativeExec>() || is_round_robin_repartition) {
            return None;
        }
        current = children[0];
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use arrow::datatypes::{DataType, Field, FieldRef, Schema};
    use datafusion_expr_common::operator::Operator;
    use datafusion_functions::math::random;
    use datafusion_physical_expr::ScalarFunctionExpr;
    use datafusion_physical_expr::expressions::{binary, lit};
    use datafusion_physical_expr_common::physical_expr::PhysicalExpr;
    use datafusion_physical_plan::displayable;
    use datafusion_physical_plan::empty::EmptyExec;
    use insta::assert_snapshot;
    use std::sync::Arc;

    #[tokio::test]
    async fn no_computation_does_not_project() -> Result<()> {
        let (left_schema, right_schema) = create_simple_schemas();
        let optimized_plan = run_test(
            left_schema,
            right_schema,
            a_x(),
            None,
            a_greater_than_x,
            JoinType::Inner,
        )?;

        assert_snapshot!(optimized_plan, @r"
        NestedLoopJoinExec: join_type=Inner, filter=a@0 > x@1
          EmptyExec
          EmptyExec
        ");
        Ok(())
    }

    #[tokio::test]
    async fn simple_push_down() -> Result<()> {
        let (left_schema, right_schema) = create_simple_schemas();
        let optimized_plan = run_test(
            left_schema,
            right_schema,
            a_x(),
            None,
            a_plus_one_greater_than_x_plus_one,
            JoinType::Inner,
        )?;

        assert_snapshot!(optimized_plan, @r"
        NestedLoopJoinExec: join_type=Inner, filter=join_proj_push_down_1@0 > join_proj_push_down_2@1, projection=[a@0, x@2]
          ProjectionExec: expr=[a@0 as a, a@0 + 1 as join_proj_push_down_1]
            EmptyExec
          ProjectionExec: expr=[x@0 as x, x@0 + 1 as join_proj_push_down_2]
            EmptyExec
        ");
        Ok(())
    }

    #[tokio::test]
    async fn does_not_push_down_short_circuiting_expressions() -> Result<()> {
        let (left_schema, right_schema) = create_simple_schemas();
        let optimized_plan = run_test(
            left_schema,
            right_schema,
            a_x(),
            None,
            |schema| {
                binary(
                    lit(false),
                    Operator::And,
                    a_plus_one_greater_than_x_plus_one(schema)?,
                    schema,
                )
            },
            JoinType::Inner,
        )?;

        assert_snapshot!(optimized_plan, @r"
        NestedLoopJoinExec: join_type=Inner, filter=false AND join_proj_push_down_1@0 > join_proj_push_down_2@1, projection=[a@0, x@2]
          ProjectionExec: expr=[a@0 as a, a@0 + 1 as join_proj_push_down_1]
            EmptyExec
          ProjectionExec: expr=[x@0 as x, x@0 + 1 as join_proj_push_down_2]
            EmptyExec
        ");
        Ok(())
    }

    #[tokio::test]
    async fn does_not_push_down_volatile_functions() -> Result<()> {
        let (left_schema, right_schema) = create_simple_schemas();
        let optimized_plan = run_test(
            left_schema,
            right_schema,
            a_x(),
            None,
            a_plus_rand_greater_than_x,
            JoinType::Inner,
        )?;

        assert_snapshot!(optimized_plan, @r"
        NestedLoopJoinExec: join_type=Inner, filter=a@0 + rand() > x@1
          EmptyExec
          EmptyExec
        ");
        Ok(())
    }

    #[tokio::test]
    async fn complex_schema_push_down() -> Result<()> {
        let (left_schema, right_schema) = create_complex_schemas();

        let optimized_plan = run_test(
            left_schema,
            right_schema,
            a_b_x_z(),
            None,
            a_plus_b_greater_than_x_plus_z,
            JoinType::Inner,
        )?;

        assert_snapshot!(optimized_plan, @r"
        NestedLoopJoinExec: join_type=Inner, filter=join_proj_push_down_1@0 > join_proj_push_down_2@1, projection=[a@0, b@1, c@2, x@4, y@5, z@6]
          ProjectionExec: expr=[a@0 as a, b@1 as b, c@2 as c, a@0 + b@1 as join_proj_push_down_1]
            EmptyExec
          ProjectionExec: expr=[x@0 as x, y@1 as y, z@2 as z, x@0 + z@2 as join_proj_push_down_2]
            EmptyExec
        ");
        Ok(())
    }

    #[tokio::test]
    async fn push_down_with_existing_projections() -> Result<()> {
        let (left_schema, right_schema) = create_complex_schemas();

        let optimized_plan = run_test(
            left_schema,
            right_schema,
            a_b_x_z(),
            Some(vec![1, 3, 5]), // ("b", "x", "z")
            a_plus_b_greater_than_x_plus_z,
            JoinType::Inner,
        )?;

        assert_snapshot!(optimized_plan, @r"
        NestedLoopJoinExec: join_type=Inner, filter=join_proj_push_down_1@0 > join_proj_push_down_2@1, projection=[b@1, x@4, z@6]
          ProjectionExec: expr=[a@0 as a, b@1 as b, c@2 as c, a@0 + b@1 as join_proj_push_down_1]
            EmptyExec
          ProjectionExec: expr=[x@0 as x, y@1 as y, z@2 as z, x@0 + z@2 as join_proj_push_down_2]
            EmptyExec
        ");
        Ok(())
    }

    #[tokio::test]
    async fn left_semi_join_projection() -> Result<()> {
        let (left_schema, right_schema) = create_simple_schemas();

        let left_semi_join_plan = run_test(
            left_schema.clone(),
            right_schema.clone(),
            a_x(),
            None,
            a_plus_one_greater_than_x_plus_one,
            JoinType::LeftSemi,
        )?;

        assert_snapshot!(left_semi_join_plan, @r"
        NestedLoopJoinExec: join_type=LeftSemi, filter=join_proj_push_down_1@0 > join_proj_push_down_2@1, projection=[a@0]
          ProjectionExec: expr=[a@0 as a, a@0 + 1 as join_proj_push_down_1]
            EmptyExec
          ProjectionExec: expr=[x@0 as x, x@0 + 1 as join_proj_push_down_2]
            EmptyExec
        ");
        Ok(())
    }

    #[tokio::test]
    async fn right_semi_join_projection() -> Result<()> {
        let (left_schema, right_schema) = create_simple_schemas();
        let right_semi_join_plan = run_test(
            left_schema,
            right_schema,
            a_x(),
            None,
            a_plus_one_greater_than_x_plus_one,
            JoinType::RightSemi,
        )?;
        assert_snapshot!(right_semi_join_plan, @r"
        NestedLoopJoinExec: join_type=RightSemi, filter=join_proj_push_down_1@0 > join_proj_push_down_2@1, projection=[x@0]
          ProjectionExec: expr=[a@0 as a, a@0 + 1 as join_proj_push_down_1]
            EmptyExec
          ProjectionExec: expr=[x@0 as x, x@0 + 1 as join_proj_push_down_2]
            EmptyExec
        ");
        Ok(())
    }

    fn run_test(
        left_schema: Schema,
        right_schema: Schema,
        column_indices: Vec<ColumnIndex>,
        existing_projections: Option<Vec<usize>>,
        filter_expr_builder: impl FnOnce(&Schema) -> Result<Arc<dyn PhysicalExpr>>,
        join_type: JoinType,
    ) -> Result<String> {
        let left = Arc::new(EmptyExec::new(Arc::new(left_schema.clone())));
        let right = Arc::new(EmptyExec::new(Arc::new(right_schema.clone())));

        let join_fields: Vec<_> = column_indices
            .iter()
            .map(|ci| match ci.side {
                JoinSide::Left => left_schema.field(ci.index).clone(),
                JoinSide::Right => right_schema.field(ci.index).clone(),
                JoinSide::None => unreachable!(),
            })
            .collect();
        let join_schema = Arc::new(Schema::new(join_fields));

        let filter_expr = filter_expr_builder(join_schema.as_ref())?;

        let join_filter = JoinFilter::new(filter_expr, column_indices, join_schema);

        let join = NestedLoopJoinExec::try_new(
            left,
            right,
            Some(join_filter),
            &join_type,
            existing_projections,
        )?;

        let optimizer = ProjectionPushdown::new();
        let optimized_plan = optimizer.optimize(Arc::new(join), &Default::default())?;

        let displayable_plan = displayable(optimized_plan.as_ref()).indent(false);
        Ok(displayable_plan.to_string())
    }

    fn create_simple_schemas() -> (Schema, Schema) {
        let left_schema = Schema::new(vec![Field::new("a", DataType::Int32, false)]);
        let right_schema = Schema::new(vec![Field::new("x", DataType::Int32, false)]);

        (left_schema, right_schema)
    }

    fn create_complex_schemas() -> (Schema, Schema) {
        let left_schema = Schema::new(vec![
            Field::new("a", DataType::Int32, false),
            Field::new("b", DataType::Int32, false),
            Field::new("c", DataType::Int32, false),
        ]);

        let right_schema = Schema::new(vec![
            Field::new("x", DataType::Int32, false),
            Field::new("y", DataType::Int32, false),
            Field::new("z", DataType::Int32, false),
        ]);

        (left_schema, right_schema)
    }

    fn a_x() -> Vec<ColumnIndex> {
        vec![
            ColumnIndex {
                index: 0,
                side: JoinSide::Left,
            },
            ColumnIndex {
                index: 0,
                side: JoinSide::Right,
            },
        ]
    }

    fn a_b_x_z() -> Vec<ColumnIndex> {
        vec![
            ColumnIndex {
                index: 0,
                side: JoinSide::Left,
            },
            ColumnIndex {
                index: 1,
                side: JoinSide::Left,
            },
            ColumnIndex {
                index: 0,
                side: JoinSide::Right,
            },
            ColumnIndex {
                index: 2,
                side: JoinSide::Right,
            },
        ]
    }

    fn a_plus_one_greater_than_x_plus_one(
        join_schema: &Schema,
    ) -> Result<Arc<dyn PhysicalExpr>> {
        let left_expr = binary(
            Arc::new(Column::new("a", 0)),
            Operator::Plus,
            lit(1),
            join_schema,
        )?;
        let right_expr = binary(
            Arc::new(Column::new("x", 1)),
            Operator::Plus,
            lit(1),
            join_schema,
        )?;
        binary(left_expr, Operator::Gt, right_expr, join_schema)
    }

    fn a_plus_rand_greater_than_x(join_schema: &Schema) -> Result<Arc<dyn PhysicalExpr>> {
        let left_expr = binary(
            Arc::new(Column::new("a", 0)),
            Operator::Plus,
            Arc::new(ScalarFunctionExpr::new(
                "rand",
                random(),
                vec![],
                FieldRef::new(Field::new("out", DataType::Float64, false)),
                Arc::new(ConfigOptions::default()),
            )),
            join_schema,
        )?;
        let right_expr = Arc::new(Column::new("x", 1));
        binary(left_expr, Operator::Gt, right_expr, join_schema)
    }

    fn a_greater_than_x(join_schema: &Schema) -> Result<Arc<dyn PhysicalExpr>> {
        binary(
            Arc::new(Column::new("a", 0)),
            Operator::Gt,
            Arc::new(Column::new("x", 1)),
            join_schema,
        )
    }

    fn a_plus_b_greater_than_x_plus_z(
        join_schema: &Schema,
    ) -> Result<Arc<dyn PhysicalExpr>> {
        let lhs = binary(
            Arc::new(Column::new("a", 0)),
            Operator::Plus,
            Arc::new(Column::new("b", 1)),
            join_schema,
        )?;
        let rhs = binary(
            Arc::new(Column::new("x", 2)),
            Operator::Plus,
            Arc::new(Column::new("z", 3)),
            join_schema,
        )?;
        binary(lhs, Operator::Gt, rhs, join_schema)
    }
}
