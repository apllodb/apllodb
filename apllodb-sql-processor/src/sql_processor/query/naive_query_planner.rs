use apllodb_shared_components::{
    ApllodbResult, AstTranslator, CorrelationReference, FieldIndex, FullFieldReference, Ordering,
    RecordFieldRefSchema,
};
use apllodb_sql_parser::apllodb_ast::{self, FromItem, SelectCommand, SelectField};
use apllodb_storage_engine_interface::ProjectionQuery;

use crate::sql_processor::query::query_plan::query_plan_tree::{
    query_plan_node::{
        node_kind::{QueryPlanNodeKind, QueryPlanNodeLeaf, QueryPlanNodeUnary},
        operation::{LeafPlanOperation, UnaryPlanOperation},
    },
    QueryPlanTree,
};

use super::query_plan::query_plan_tree::query_plan_node::node_repo::QueryPlanNodeRepository;

/// Translates [SelectCommand](apllodb_sql_parser::apllodb_ast::SelectCommand) into [QueryPlanTree](crate::sql_processor::query::query_plan::query_plan_tree::QueryPlanTree).
///
/// Output tree has the following form:
///
/// ```text
/// proj
///  |
/// aggregation
///  |
/// sort
///  |
/// selection
///  |
/// join
///  |------+
/// CORR   CORR
/// ```
///
/// Nodes are created from bottom to top.
#[derive(Clone, Debug, new)]
pub(crate) struct NaiveQueryPlanner<'r> {
    node_repo: &'r QueryPlanNodeRepository,
    select_command: SelectCommand,
}

impl<'r> NaiveQueryPlanner<'r> {
    pub(crate) fn from_select_command(&self) -> ApllodbResult<QueryPlanTree> {
        if self.select_command.grouping_elements.is_some() {
            unimplemented!();
        }
        if self.select_command.having_conditions.is_some() {
            unimplemented!();
        }

        self.create_correlation_nodes()?;

        let select_fields = self.select_command.select_fields.into_vec();
        let ffrs: Vec<FullFieldReference> =
            Self::select_fields_into_ffrs(&select_fields, &from_correlations)?;
        let schema = RecordFieldRefSchema::new(ffrs);

        let node1_id = if let Some(condition) = self.select_command.where_condition {
            let selection_op = UnaryPlanOperation::Selection {
                condition: AstTranslator::condition_in_select(condition, &from_correlations)?,
            };
            self.node_repo
                .create(QueryPlanNodeKind::Unary(QueryPlanNodeUnary {
                    op: selection_op,
                    left: leaf_node_id,
                }))
        } else {
            leaf_node_id
        };

        let node2_id = if let Some(order_byes) = sc.order_bys {
            self.node_repo
                .create(QueryPlanNodeKind::Unary(QueryPlanNodeUnary {
                    op: Self::sort_node(order_byes, &from_correlations)?,
                    left: node1_id,
                }))
        } else {
            node1_id
        };

        // FIXME not necessary? `let ffrs: Vec<FullFieldReference>` already filters necessary fields.
        let root_node_id = self
            .node_repo
            .create(QueryPlanNodeKind::Unary(QueryPlanNodeUnary {
                op: Self::projection_node(
                    select_fields
                        .into_iter()
                        .map(|select_field| {
                            if let apllodb_ast::Expression::ColumnReferenceVariant(ast_colref) =
                                select_field.expression
                            {
                                (ast_colref, select_field.alias)
                            } else {
                                panic!("fix 'FIXME' above!")
                            }
                        })
                        .collect(),
                    &from_correlations,
                )?,
                left: node2_id,
            }));

        Ok(QueryPlanTree::new(root_node_id))
    }

    fn create_correlation_nodes(&self) -> ApllodbResult<()> {
        let ast_from_item = self.select_command_into_from_item();
        let from_correlations = AstTranslator::from_item(ast_from_item)?;

        for corref in from_correlations {
            let _ = self
                .node_repo
                .create(QueryPlanNodeKind::Leaf(QueryPlanNodeLeaf {
                    op: LeafPlanOperation::SeqScan {
                        table_name: corref.as_table_name().clone(),
                        projection: ProjectionQuery::Schema(schema),
                    },
                }));
        }

        Ok(())
    }

    /// including all fields used during a SELECT execution
    fn widest_schema(&self) -> RecordFieldRefSchema {
        todo!()
    }

    fn select_command_into_from_item(&self) -> FromItem {
        self.select_command
            .from_items
            .expect("currently SELECT w/o FROM is unimplemented")
            .as_vec()
            .first()
            .unwrap()
            .clone()
    }

    fn select_fields_into_ffrs(
        select_fields: &[SelectField],
        correlations: &[CorrelationReference],
    ) -> ApllodbResult<Vec<FullFieldReference>> {
        select_fields
            .iter()
            .map(|select_field| Self::select_field_into_ffr(select_field, correlations))
            .collect::<ApllodbResult<_>>()
    }

    fn select_field_into_ffr(
        select_field: &SelectField,
        correlations: &[CorrelationReference],
    ) -> ApllodbResult<FullFieldReference> {
        match &select_field.expression {
            apllodb_ast::Expression::ConstantVariant(_) => {
                unimplemented!();
            }
            apllodb_ast::Expression::ColumnReferenceVariant(ast_colref) => {
                AstTranslator::select_field_column_reference(
                    ast_colref.clone(),
                    select_field.alias.clone(),
                    correlations,
                )
            }
            apllodb_ast::Expression::UnaryOperatorVariant(_, _)
            | apllodb_ast::Expression::BinaryOperatorVariant(_, _, _) => {
                // TODO このレイヤーで計算しちゃいたい
                unimplemented!();
            }
        }
    }

    fn projection_node(
        fields: Vec<(apllodb_ast::ColumnReference, Option<apllodb_ast::Alias>)>,
        correlations: &[CorrelationReference],
    ) -> ApllodbResult<UnaryPlanOperation> {
        let node = UnaryPlanOperation::Projection {
            fields: fields
                .into_iter()
                .map(|(ast_colref, ast_field_alias)| {
                    AstTranslator::select_field_column_reference(
                        ast_colref,
                        ast_field_alias,
                        correlations,
                    )
                    .map(FieldIndex::from)
                })
                .collect::<ApllodbResult<_>>()?,
        };
        Ok(node)
    }

    fn sort_node(
        ast_order_byes: apllodb_ast::NonEmptyVec<apllodb_ast::OrderBy>,
        correlations: &[CorrelationReference],
    ) -> ApllodbResult<UnaryPlanOperation> {
        let order_byes = ast_order_byes.into_vec();

        let field_orderings: Vec<(FieldIndex, Ordering)> = order_byes
            .into_iter()
            .map(|order_by| {
                let expression =
                    AstTranslator::expression_in_select(order_by.expression, correlations)?;
                let ffr = match expression {
                    apllodb_shared_components::Expression::FullFieldReferenceVariant(ffr) => ffr,
                    apllodb_shared_components::Expression::ConstantVariant(_)
                    | apllodb_shared_components::Expression::UnaryOperatorVariant(_, _)
                    | apllodb_shared_components::Expression::BooleanExpressionVariant(_) => {
                        unimplemented!()
                    }
                };
                let index = FieldIndex::from(ffr);
                let ordering = AstTranslator::ordering(order_by.ordering);
                Ok((index, ordering))
            })
            .collect::<ApllodbResult<_>>()?;

        Ok(UnaryPlanOperation::Sort { field_orderings })
    }
}
