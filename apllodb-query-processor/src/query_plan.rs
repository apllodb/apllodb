pub(crate) mod plan_tree;

use serde::{Deserialize, Serialize};

use self::plan_tree::PlanTree;

/// Query plan from which an executor can do its work deterministically.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
pub(crate) struct QueryPlan {
    plan_tree: PlanTree,
    // TODO evaluated cost, etc...
    // See PostgreSQL's plan structure: <https://github.com/postgres/postgres/blob/master/src/include/nodes/plannodes.h#L110>
}
