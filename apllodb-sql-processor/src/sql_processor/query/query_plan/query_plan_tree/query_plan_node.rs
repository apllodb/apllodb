pub(crate) mod node_id;
pub(crate) mod node_kind;
pub(crate) mod operation;

use self::{
    node_id::{QueryPlanNodeId, QueryPlanNodeIdGenerator},
    node_kind::QueryPlanNodeKind,
};
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
    pub(crate) fn new(id_gen: &QueryPlanNodeIdGenerator, kind: QueryPlanNodeKind) -> Self {
        Self {
            id: id_gen.gen(),
            kind,
        }
    }
}
