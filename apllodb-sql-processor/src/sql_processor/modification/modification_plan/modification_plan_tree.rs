pub(crate) mod modification_plan_node;

use self::modification_plan_node::ModificationPlanNode;

/// Modification plan tree.
/// This tree is a binary tree, whose root node is a ModificationPlanNode and its children are QueryPlanNode to DML.
#[derive(Clone, PartialEq, Debug, new)]
pub(crate) struct ModificationPlanTree {
    pub(crate) root: ModificationPlanNode,
}
