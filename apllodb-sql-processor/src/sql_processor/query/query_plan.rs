pub(crate) mod query_plan_tree;

use std::sync::Arc;

use self::query_plan_tree::{query_plan_node::node_id::QueryPlanNodeIdGenerator, QueryPlanTree};

/// Query plan from which an executor can do its work deterministically.
#[derive(Clone, PartialEq, Debug, new)]
pub(crate) struct QueryPlan {
    pub(crate) plan_tree: QueryPlanTree,
    pub(crate) id_gen: Arc<QueryPlanNodeIdGenerator>,
    // TODO evaluated cost, etc...
    // See PostgreSQL's plan structure: <https://github.com/postgres/postgres/blob/master/src/include/nodes/plannodes.h#L110>
}
