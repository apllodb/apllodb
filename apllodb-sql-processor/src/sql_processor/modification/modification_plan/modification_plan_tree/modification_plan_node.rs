use apllodb_shared_components::TableName;
use serde::{Deserialize, Serialize};

use crate::sql_processor::query::query_plan::query_plan_tree::query_plan_node::QueryPlanNode;

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
/// Root node of modification plan tree.
pub(crate) enum ModificationPlanNode {
    Insert(InsertNode),
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
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
    pub(crate) child: QueryPlanNode,
}
