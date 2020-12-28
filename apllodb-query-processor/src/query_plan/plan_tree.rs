pub(crate) mod plan_node;

use serde::{Deserialize, Serialize};

use self::plan_node::PlanNode;

/// Query plan tree.
/// This tree is a binary tree because every SELECT operation can break down into unary or binary operations.
#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize, new)]
pub(crate) struct PlanTree {
    pub(crate) root: PlanNode,
}
