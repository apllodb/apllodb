pub(crate) mod query_plan_tree;

use std::convert::TryFrom;

use apllodb_shared_components::{
    ApllodbError, ApllodbResult, AstTranslator, CorrelationReference, FieldIndex,
    FullFieldReference, Ordering, RecordFieldRefSchema,
};
use apllodb_sql_parser::apllodb_ast::{self, FromItem, SelectCommand, SelectField};
use apllodb_storage_engine_interface::ProjectionQuery;

use self::query_plan_tree::{
    query_plan_node::{
        LeafPlanOperation, QueryPlanNode, QueryPlanNodeLeaf, QueryPlanNodeUnary, UnaryPlanOperation,
    },
    QueryPlanTree,
};

/// Query plan from which an executor can do its work deterministically.
#[derive(Clone, PartialEq, Debug, new)]
pub(crate) struct QueryPlan {
    pub(crate) plan_tree: QueryPlanTree,
    // TODO evaluated cost, etc...
    // See PostgreSQL's plan structure: <https://github.com/postgres/postgres/blob/master/src/include/nodes/plannodes.h#L110>
}

impl TryFrom<SelectCommand> for QueryPlan {
    type Error = ApllodbError;

    fn try_from(sc: SelectCommand) -> ApllodbResult<Self> {
        if sc.grouping_elements.is_some() {
            unimplemented!();
        }
        if sc.having_conditions.is_some() {
            unimplemented!();
        }

        let from_item = Self::select_command_into_from_item(sc.clone());
        let correlations = AstTranslator::from_item(from_item)?;

        let select_fields = sc.select_fields.into_vec();
        let ffrs: Vec<FullFieldReference> =
            Self::select_fields_into_ffrs(&select_fields, &correlations)?;
        let schema = RecordFieldRefSchema::new(ffrs);

        if correlations.len() != 1 {
            unimplemented!("currently SELECT w/ 0 or 2+ FROM items is not implemented");
        }
        let corref = correlations[0].clone();

        let leaf_node = QueryPlanNode::Leaf(QueryPlanNodeLeaf {
            op: LeafPlanOperation::SeqScan {
                table_name: corref.as_table_name().clone(),
                projection: ProjectionQuery::Schema(schema),
            },
        });

        let node1 = if let Some(condition) = sc.where_condition {
            let selection_op = UnaryPlanOperation::Selection {
                condition: AstTranslator::condition_in_select(condition, &correlations)?,
            };
            QueryPlanNode::Unary(QueryPlanNodeUnary {
                op: selection_op,
                left: Box::new(leaf_node),
            })
        } else {
            leaf_node
        };

        let node2 = if let Some(order_byes) = sc.order_bys {
            QueryPlanNode::Unary(QueryPlanNodeUnary {
                op: Self::sort_node(order_byes, &correlations)?,
                left: Box::new(node1),
            })
        } else {
            node1
        };

        // FIXME not necessary? `let ffrs: Vec<FullFieldReference>` already filters necessary fields.
        let root_node = QueryPlanNode::Unary(QueryPlanNodeUnary {
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
                &correlations,
            )?,
            left: Box::new(node2),
        });

        Ok(QueryPlan::new(QueryPlanTree::new(root_node)))
    }
}

impl QueryPlan {
    fn select_command_into_from_item(select_command: SelectCommand) -> FromItem {
        select_command
            .from_item
            .expect("currently SELECT w/o FROM is unimplemented")
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
