use std::collections::HashMap;

use apllodb_shared_components::Expression;
use apllodb_storage_engine_interface::{ColumnName, TableName};

use crate::sql_processor::query::query_plan::query_plan_tree::query_plan_node::node_id::QueryPlanNodeId;

#[derive(Clone, PartialEq, Debug)]
/// Root node of modification plan tree.
pub(crate) enum ModificationPlanNode {
    Insert(InsertNode),
    Update(UpdateNode),
}

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct InsertNode {
    pub(crate) table_name: TableName,

    /// Records to insert are sometimes passed in SQL;
    ///
    /// ```sql
    /// INSERT INTO t (id, c) VALUES (1, "xyz"), (2, "abc");
    /// ```
    ///
    /// and other times fetched from tables.
    ///
    /// ```sql
    /// INSERT INTO t (id, c) SELECT c_id, d FROM s;
    /// ```
    pub(crate) child: QueryPlanNodeId,
}

#[derive(Clone, PartialEq, Debug)]
pub(crate) struct UpdateNode {
    pub(crate) table_name: TableName,
    pub(crate) column_values: HashMap<ColumnName, Expression>,

    /// Expression here must be evaluated as BOOLEAN (NULL is FALSE in BOOLEAN context).
    /// Otherwise [DatatypeMismatch](apllodb-shared-components::ApllodbErrorKind::DatatypeMismatch).
    pub(crate) where_condition: Option<Expression>,
}
