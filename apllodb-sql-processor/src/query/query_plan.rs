pub(crate) mod query_plan_tree;

use std::convert::TryFrom;

use apllodb_shared_components::{ApllodbError, ApllodbResult, ColumnName, TableName};
use apllodb_sql_parser::apllodb_ast::{self, SelectCommand};
use apllodb_storage_engine_interface::ProjectionQuery;
use serde::{Deserialize, Serialize};

use self::query_plan_tree::{
    query_plan_node::{LeafPlanOperation, QueryPlanNode, QueryPlanNodeLeaf},
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

        let from_items = sc.from_items.into_vec();
        let table_names: Vec<TableName> = from_items
            .into_iter()
            .map(|from_item| {
                if from_item.alias.is_some() {
                    unimplemented!();
                }
                TableName::new(from_item.table_name.0 .0)
            })
            .collect::<ApllodbResult<_>>()?;

        let table_name = if table_names.len() != 1 {
            unimplemented!()
        } else {
            table_names.first().unwrap().clone()
        };

        let select_fields = sc.select_fields.into_vec();
        let column_names: Vec<ColumnName> = select_fields
            .into_iter()
            .map(|select_field| {
                if select_field.alias.is_some() {
                    unimplemented!();
                }

                match select_field.expression {
                    apllodb_ast::Expression::ConstantVariant(_) => {
                        unimplemented!();
                    }
                    apllodb_ast::Expression::ColumnReferenceVariant(colref) => {
                        if colref.correlation.is_some() {
                            unimplemented!();
                        }

                        ColumnName::new(colref.column_name.0 .0)
                    }
                }
            })
            .collect::<ApllodbResult<_>>()?;

        let seq_scan_node = QueryPlanNode::Leaf(QueryPlanNodeLeaf {
            op: LeafPlanOperation::SeqScan {
                table_name,
                projection: ProjectionQuery::ColumnNames(column_names),
            },
        });

        Ok(QueryPlan::new(QueryPlanTree::new(seq_scan_node)))
    }
}
