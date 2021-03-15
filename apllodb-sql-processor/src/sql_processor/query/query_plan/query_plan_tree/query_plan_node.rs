pub(crate) mod node_id;
pub(crate) mod node_kind;
pub(crate) mod node_repo;
pub(crate) mod operation;

use apllodb_shared_components::CorrelationReference;

use self::{node_id::QueryPlanNodeId, node_kind::QueryPlanNodeKind, operation::LeafPlanOperation};
use std::hash::Hash;

/// Node of query plan tree.
#[derive(Clone, Debug)]
pub(crate) struct QueryPlanNode {
    pub(crate) id: QueryPlanNodeId,
    pub(crate) kind: QueryPlanNodeKind,
}

impl PartialEq for QueryPlanNode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for QueryPlanNode {}
impl Hash for QueryPlanNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl QueryPlanNode {
    pub(in crate::sql_processor::query::query_plan::query_plan_tree::query_plan_node) fn new(
        id: QueryPlanNodeId,
        kind: QueryPlanNodeKind,
    ) -> Self {
        Self { id, kind }
    }

    /// Returns CorrelationReference if this node is a correlation data source (SeqScan, for example).
    pub(in crate::sql_processor::query::query_plan::query_plan_tree::query_plan_node) fn source_correlation_reference(
        &self,
    ) -> Option<CorrelationReference> {
        match &self.kind {
            QueryPlanNodeKind::Leaf(leaf) => match &leaf.op {
                LeafPlanOperation::SeqScan { table_name, .. } => {
                    Some(CorrelationReference::TableNameVariant(table_name.clone()))
                }
                _ => None,
            },
            _ => None,
        }
    }
}
