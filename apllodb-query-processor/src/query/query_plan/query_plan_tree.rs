pub(crate) mod query_plan_node;

use serde::{Deserialize, Serialize};

use self::query_plan_node::QueryPlanNode;

/// Query plan tree.
/// This tree is a binary tree because every SELECT operation can break down into unary or binary operations.
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, new)]
pub(crate) struct QueryPlanTree {
    pub(crate) root: QueryPlanNode,
}
