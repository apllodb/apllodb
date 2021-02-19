pub(crate) mod query_plan_tree;

use std::convert::TryFrom;

use apllodb_shared_components::{
    ApllodbError, ApllodbResult, AstTranslator, ColumnName, FieldIndex, FromItem,
    FullFieldReference, Ordering, TableWithAlias,
};
use apllodb_sql_parser::apllodb_ast::{self, SelectCommand, SelectField};
use apllodb_storage_engine_interface::{AliasDef, ProjectionQuery};

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

        let from_item = Self::select_command_into_from_item(sc.clone())?;

        let select_fields = sc.select_fields.into_vec();
        let ffrs: Vec<FullFieldReference> =
            Self::select_fields_into_ffrs(&select_fields, &from_item)?;

        let column_names: Vec<ColumnName> = ffrs
            .iter()
            .map(|ffr| ffr.as_column_name())
            .cloned()
            .collect();

        let table_with_aliases: Vec<TableWithAlias> = (&from_item).into();
        assert!(
            table_with_aliases.len() == 1,
            "FROM item must be 1 currently"
        );
        let table_with_alias = table_with_aliases.first().unwrap();

        let alias_def = AliasDef::new(table_with_alias.clone(), &ffrs);

        let leaf_node = QueryPlanNode::Leaf(QueryPlanNodeLeaf {
            op: LeafPlanOperation::SeqScan {
                table_name: table_with_alias.table_name.clone(),
                projection: ProjectionQuery::ColumnNames(column_names),
                alias_def,
            },
        });

        let node1 = if let Some(condition) = sc.where_condition {
            let selection_op = UnaryPlanOperation::Selection {
                condition: AstTranslator::condition(condition)?,
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
                op: Self::sort_node(order_byes)?,
                left: Box::new(node1),
            })
        } else {
            node1
        };

        let root_node = QueryPlanNode::Unary(QueryPlanNodeUnary {
            op: Self::projection_node(select_fields)?,
            left: Box::new(node2),
        });

        Ok(QueryPlan::new(QueryPlanTree::new(root_node)))
    }
}

impl QueryPlan {
    fn select_command_into_from_item(select_command: SelectCommand) -> ApllodbResult<FromItem> {
        let ast_from_items = select_command
            .from_items
            .expect("currently SELECT w/o FROM is unimplemented")
            .into_vec();

        assert!(ast_from_items.len() == 1, "currently FROM item must be 1");

        let ast_from_item = ast_from_items.first().unwrap();
        AstTranslator::from_item(ast_from_item.clone())
    }

    fn select_fields_into_ffrs(
        select_fields: &[SelectField],
        from_item: &FromItem,
    ) -> ApllodbResult<Vec<FullFieldReference>> {
        select_fields
            .iter()
            .map(|select_field| Self::select_field_into_ffr(select_field, &from_item))
            .collect::<ApllodbResult<_>>()
    }

    fn select_field_into_ffr(
        select_field: &SelectField,
        from_item: &FromItem,
    ) -> ApllodbResult<FullFieldReference> {
        match &select_field.expression {
            apllodb_ast::Expression::ConstantVariant(_) => {
                unimplemented!();
            }
            apllodb_ast::Expression::ColumnReferenceVariant(_) => {
                let sfr = AstTranslator::select_field_column_reference(select_field.clone())?;
                sfr.resolve(Some(from_item.clone()))
            }
            apllodb_ast::Expression::UnaryOperatorVariant(_, _)
            | apllodb_ast::Expression::BinaryOperatorVariant(_, _, _) => {
                // TODO このレイヤーで計算しちゃいたい
                unimplemented!();
            }
        }
    }

    fn projection_node(select_fields: Vec<SelectField>) -> ApllodbResult<UnaryPlanOperation> {
        let node = UnaryPlanOperation::Projection {
            fields: select_fields
                .into_iter()
                .map(|select_field| {
                    AstTranslator::select_field_column_reference(select_field).map(FieldIndex::from)
                })
                .collect::<ApllodbResult<_>>()?,
        };
        Ok(node)
    }

    fn sort_node(
        ast_order_byes: apllodb_ast::NonEmptyVec<apllodb_ast::OrderBy>,
    ) -> ApllodbResult<UnaryPlanOperation> {
        let order_byes = ast_order_byes.into_vec();

        let field_orderings: Vec<(FieldIndex, Ordering)> = order_byes
            .into_iter()
            .map(|order_by| {
                let expression = AstTranslator::expression(order_by.expression)?;
                let ffr = match expression {
                    apllodb_shared_components::Expression::SelectFieldReferenceVariant(ffr) => ffr,
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
