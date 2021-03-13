use std::collections::HashMap;

use apllodb_shared_components::ApllodbResult;

use super::{
    node_id::{QueryPlanNodeId, QueryPlanNodeIdGenerator},
    node_kind::QueryPlanNodeKind,
    QueryPlanNode,
};

#[derive(Debug, Default)]
pub(crate) struct QueryPlanNodeRepository {
    hmap: HashMap<QueryPlanNodeId, QueryPlanNode>,
    id_gen: QueryPlanNodeIdGenerator,
}

impl QueryPlanNodeRepository {
    pub(crate) fn create(&mut self, kind: QueryPlanNodeKind) -> &QueryPlanNode {
        let node = QueryPlanNode::new(self.id_gen.gen(), kind);
        let id = node.id;
        self.hmap.insert(id, node);
        self.hmap.get(&id).expect("just inserted")
    }

    /// # Failures
    ///
    /// -
    pub(crate) fn find(&self, _id: QueryPlanNodeId) -> ApllodbResult<&QueryPlanNode> {
        todo!()
    }

    pub(crate) fn remove(&mut self, _id: QueryPlanNodeId) -> ApllodbResult<QueryPlanNode> {
        todo!()
    }
}
