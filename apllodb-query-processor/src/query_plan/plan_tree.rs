pub(crate) mod plan_node;

use serde::{Deserialize, Serialize};

use self::plan_node::PlanNode;

/// Query plan tree.
/// This tree is a binary tree because every SELECT operation can break down into unary or binary operations.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
pub(crate) struct PlanTree {
    root: PlanNode,
}
