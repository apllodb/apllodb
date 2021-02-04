pub(crate) mod query_plan_tree;

use std::convert::TryFrom;

use apllodb_shared_components::{
    ApllodbError, ApllodbResult, AstTranslator, ColumnName, FieldIndex,
};
use apllodb_sql_parser::apllodb_ast::{self, SelectCommand};
use apllodb_storage_engine_interface::ProjectionQuery;
use serde::{Deserialize, Serialize};

use self::query_plan_tree::{
    query_plan_node::{
        LeafPlanOperation, QueryPlanNode, QueryPlanNodeLeaf, QueryPlanNodeUnary, UnaryPlanOperation,
    },
    QueryPlanTree,
};

/// Query plan from which an executor can do its work deterministically.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, new)]
pub(crate) struct QueryPlan {
    pub(crate) plan_tree: QueryPlanTree,
    // TODO evaluated cost, etc...
    // See PostgreSQL's plan structure: <https://github.com/postgres/postgres/blob/master/src/include/nodes/plannodes.h#L110>
}

impl TryFrom<SelectCommand> for QueryPlan {
    type Error = ApllodbError;

    fn try_from(sc: SelectCommand) -> ApllodbResult<Self> {
        if sc.where_condition.is_some() {
            unimplemented!();
        }
        if sc.grouping_elements.is_some() {
            unimplemented!();
        }
        if sc.having_conditions.is_some() {
            unimplemented!();
        }
        if sc.order_bys.is_some() {
            unimplemented!();
        }

        let from_items = sc
            .from_items
            .expect("currently SELECT w/o FROM is unimplemented")
            .into_vec();

        let from_item = if from_items.len() != 1 {
            unimplemented!()
        } else {
            from_items.first().unwrap().clone()
        };

        let select_fields = sc.select_fields.into_vec();
        let column_names: Vec<ColumnName> = select_fields
            .iter()
            .map(|select_field| {
                match &select_field.expression {
                    apllodb_ast::Expression::ConstantVariant(_) => {
                        unimplemented!();
                    }
                    apllodb_ast::Expression::ColumnReferenceVariant(_) => {
                        let ffr = AstTranslator::select_field_column_reference(
                            select_field.clone(),
                            from_items.clone(),
                        )?;
                        Ok(ffr.as_column_name().clone())  // field alias情報が消えている
                    }
                    apllodb_ast::Expression::UnaryOperatorVariant(_, _) => {
                        // TODO このレイヤーで計算しちゃいたい
                        unimplemented!();
                    }
                }
            })
            .collect::<ApllodbResult<_>>()?;

        let seq_scan_node = QueryPlanNode::Leaf(QueryPlanNodeLeaf {
            op: LeafPlanOperation::SeqScan {
                table_name: AstTranslator::table_name(from_item.table_name)?,  // correlation alias情報が消えている
                projection: ProjectionQuery::ColumnNames(column_names),
            },
        });
        let projection_node = QueryPlanNode::Unary(QueryPlanNodeUnary {
            op: UnaryPlanOperation::Projection {
                fields: select_fields
                    .into_iter()
                    .map(|select_field| {
                        AstTranslator::select_field_column_reference(
                            select_field,
                            from_items.clone(),
                        )
                        .map(FieldIndex::from)
                    })
                    .collect::<ApllodbResult<_>>()?,
            },
            left: Box::new(seq_scan_node),
        });

        Ok(QueryPlan::new(QueryPlanTree::new(projection_node)))
    }
}
