pub(crate) mod query_plan_tree;

use serde::{Deserialize, Serialize};

use self::query_plan_tree::QueryPlanTree;

/// Query plan from which an executor can do its work deterministically.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, new)]
pub(crate) struct QueryPlan {
    pub(crate) plan_tree: QueryPlanTree,
    // TODO evaluated cost, etc...
    // See PostgreSQL's plan structure: <https://github.com/postgres/postgres/blob/master/src/include/nodes/plannodes.h#L110>
}
