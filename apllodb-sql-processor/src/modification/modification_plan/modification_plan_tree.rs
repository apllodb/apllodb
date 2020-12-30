pub(crate) mod modification_plan_node;

use serde::{Deserialize, Serialize};

use self::modification_plan_node::ModificationPlanNode;

/// Modification plan tree.
/// This tree is a binary tree, whose root node is a [ModificationPlanNode](self::modification_plan_node::ModificationPlanNode) and its children are [QueryPlanNode](crate::query::query_plan::query_plan_tree::QueryPlanNode) (providing input [RecordIterator](apllodb_shared_components::RecordIterator) to DML).
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize, new)]
pub(crate) struct ModificationPlanTree {
    pub(crate) root: ModificationPlanNode,
}
