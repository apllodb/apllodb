use std::collections::HashSet;

use apllodb_shared_components::{
    ApllodbResult, AstTranslator, CorrelationIndex, CorrelationReference, Expression, FieldIndex,
    FullFieldReference, Ordering, RecordFieldRefSchema,
};
use apllodb_sql_parser::apllodb_ast::{self, SelectCommand};
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
    pub(crate) fn run(&self) -> ApllodbResult<QueryPlanTree> {
        self.create_correlation_nodes()?;

        // join

        self.create_selection_node()?;

        self.create_sort_node()?;

        // aggregation

        self.create_projection_node()?;

        Ok(QueryPlanTree::new(self.node_repo.latest_node_id()?))
    }

    fn create_correlation_nodes(&self) -> ApllodbResult<()> {
        let from_correlations = self.select_command_into_correlation_references()?;

        let widest_schema = self.widest_schema()?;

        for corref in from_correlations {
            let _ = self
                .node_repo
                .create(QueryPlanNodeKind::Leaf(QueryPlanNodeLeaf {
                    op: LeafPlanOperation::SeqScan {
                        table_name: corref.as_table_name().clone(),
                        projection: ProjectionQuery::Schema(
                            widest_schema.filter_by_correlation(&CorrelationIndex::from(corref)),
                        ),
                    },
                }));
        }

        Ok(())
    }

    fn create_selection_node(&self) -> ApllodbResult<()> {
        if let Some(ast_condition) = &self.select_command.where_condition {
            let from_correlations = self.select_command_into_correlation_references()?;

            let selection_op = UnaryPlanOperation::Selection {
                condition: AstTranslator::condition_in_select(
                    ast_condition.clone(),
                    &from_correlations,
                )?,
            };
            let child_id = self.node_repo.latest_node_id()?;

            let _ = self
                .node_repo
                .create(QueryPlanNodeKind::Unary(QueryPlanNodeUnary {
                    op: selection_op,
                    left: child_id,
                }));
        }
        Ok(())
    }

    fn create_sort_node(&self) -> ApllodbResult<()> {
        if let Some(ast_order_bys) = &self.select_command.order_bys {
            let from_correlations = self.select_command_into_correlation_references()?;

            let sort_op = Self::sort_node(ast_order_bys.clone(), &from_correlations)?;
            let child_id = self.node_repo.latest_node_id()?;

            let _ = self
                .node_repo
                .create(QueryPlanNodeKind::Unary(QueryPlanNodeUnary {
                    op: sort_op,
                    left: child_id,
                }));
        }
        Ok(())
    }

    fn create_projection_node(&self) -> ApllodbResult<()> {
        let from_correlations = self.select_command_into_correlation_references()?;
        let select_fields = self.select_command.select_fields.as_vec().clone();

        let projection_op = Self::projection_node(
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
        )?;
        let child_id = self.node_repo.latest_node_id()?;

        let _ = self
            .node_repo
            .create(QueryPlanNodeKind::Unary(QueryPlanNodeUnary {
                op: projection_op,
                left: child_id,
            }));

        Ok(())
    }

    /// including all fields used during a SELECT execution
    fn widest_schema(&self) -> ApllodbResult<RecordFieldRefSchema> {
        let mut widest_ffrs = HashSet::<FullFieldReference>::new();

        for ffr in self.ffrs_in_selection()? {
            widest_ffrs.insert(ffr);
        }
        for ffr in self.ffrs_in_sort()? {
            widest_ffrs.insert(ffr);
        }
        for ffr in self.ffrs_in_projection()? {
            widest_ffrs.insert(ffr);
        }

        Ok(RecordFieldRefSchema::new(widest_ffrs.into_iter().collect()))
    }
    fn ffrs_in_selection(&self) -> ApllodbResult<HashSet<FullFieldReference>> {
        if let Some(ast_condition) = &self.select_command.where_condition {
            let from_correlations = self.select_command_into_correlation_references()?;
            let expression =
                AstTranslator::condition_in_select(ast_condition.clone(), &from_correlations)?;
            let ffrs = expression.to_full_field_references();
            Ok(ffrs.into_iter().collect())
        } else {
            Ok(HashSet::new())
        }
    }
    fn ffrs_in_sort(&self) -> ApllodbResult<HashSet<FullFieldReference>> {
        // FIXME create_sort_node() と共通化
        if let Some(ast_order_bys) = &self.select_command.order_bys {
            let from_correlations = self.select_command_into_correlation_references()?;

            let ast_order_bys = ast_order_bys.clone().into_vec();

            let ffrs: HashSet<FullFieldReference> = ast_order_bys
                .into_iter()
                .map(|ast_order_by| {
                    let expression = AstTranslator::expression_in_select(
                        ast_order_by.expression,
                        &from_correlations,
                    )?;
                    let ffr = if let Expression::FullFieldReferenceVariant(ffr) = expression {
                        ffr
                    } else {
                        unimplemented!(
                            "ORDER BY's expression is supposed to be a field name currently"
                        );
                    };
                    Ok(ffr)
                })
                .collect::<ApllodbResult<_>>()?;
            Ok(ffrs)
        } else {
            Ok(HashSet::new())
        }
    }
    fn ffrs_in_projection(&self) -> ApllodbResult<HashSet<FullFieldReference>> {
        let from_correlations = self.select_command_into_correlation_references()?;
        let ast_select_fields = self.select_command.select_fields.as_vec().clone();

        ast_select_fields
            .iter()
            .map(|select_field| Self::select_field_into_ffr(select_field, &from_correlations))
            .collect::<ApllodbResult<_>>()
    }

    fn select_command_into_correlation_references(
        &self,
    ) -> ApllodbResult<Vec<CorrelationReference>> {
        let ast_from_item = self
            .select_command
            .from_items
            .clone()
            .expect("currently SELECT w/o FROM is unimplemented")
            .as_vec()
            .first()
            .unwrap()
            .clone();
        AstTranslator::from_item(ast_from_item)
    }

    fn select_field_into_ffr(
        ast_select_field: &apllodb_ast::SelectField,
        from_correlations: &[CorrelationReference],
    ) -> ApllodbResult<FullFieldReference> {
        match &ast_select_field.expression {
            apllodb_ast::Expression::ConstantVariant(_) => {
                unimplemented!();
            }
            apllodb_ast::Expression::ColumnReferenceVariant(ast_colref) => {
                AstTranslator::select_field_column_reference(
                    ast_colref.clone(),
                    ast_select_field.alias.clone(),
                    from_correlations,
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
        from_correlations: &[CorrelationReference],
    ) -> ApllodbResult<UnaryPlanOperation> {
        let ast_order_bys = ast_order_byes.into_vec();

        let field_orderings: Vec<(FieldIndex, Ordering)> = ast_order_bys
            .into_iter()
            .map(|ast_order_by| {
                let expression = AstTranslator::expression_in_select(
                    ast_order_by.expression,
                    from_correlations,
                )?;
                let ffr = if let Expression::FullFieldReferenceVariant(ffr) = expression {
                    ffr
                } else {
                    unimplemented!(
                        "ORDER BY's expression is supposed to be a field name currently"
                    );
                };
                let index = FieldIndex::from(ffr);
                let ordering = AstTranslator::ordering(ast_order_by.ordering);
                Ok((index, ordering))
            })
            .collect::<ApllodbResult<_>>()?;

        Ok(UnaryPlanOperation::Sort { field_orderings })
    }
}
