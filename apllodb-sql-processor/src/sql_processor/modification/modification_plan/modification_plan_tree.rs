pub(crate) mod modification_plan_node;

use serde::{Deserialize, Serialize};

use self::modification_plan_node::ModificationPlanNode;

/// Modification plan tree.
/// This tree is a binary tree, whose root node is a ModificationPlanNode and its children are QueryPlanNode to DML.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, new)]
pub(crate) struct ModificationPlanTree {
    pub(crate) root: ModificationPlanNode,
}
