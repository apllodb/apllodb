use std::{
    collections::HashMap,
    sync::{Mutex, RwLock},
};

use apllodb_shared_components::{ApllodbError, ApllodbErrorKind, ApllodbResult};

use super::{
    node_id::{QueryPlanNodeId, QueryPlanNodeIdGenerator},
    node_kind::QueryPlanNodeKind,
    QueryPlanNode,
};

#[derive(Debug, Default)]
pub(crate) struct QueryPlanNodeRepository {
    hmap: RwLock<HashMap<QueryPlanNodeId, QueryPlanNode>>,
    id_gen: Mutex<QueryPlanNodeIdGenerator>,
}

impl QueryPlanNodeRepository {
    pub(crate) fn create(&self, kind: QueryPlanNodeKind) -> QueryPlanNodeId {
        let node = QueryPlanNode::new(self.id_gen.lock().unwrap().gen(), kind);
        let id = node.id;
        self.hmap.write().unwrap().insert(id, node);
        id
    }

    /// # Failures
    ///
    /// - [UndefinedObject](apllodb-shared-components::ApllodbErrorKind::UndefinedObject) when:
    ///    - node with id does not exist.
    pub(crate) fn remove(&self, id: QueryPlanNodeId) -> ApllodbResult<QueryPlanNode> {
        self.hmap.write().unwrap().remove(&id).ok_or_else(|| {
            ApllodbError::new(
                ApllodbErrorKind::UndefinedObject,
                format!("QueryPlanNode:{:?} does not exist (already removed?)", id),
                None,
            )
        })
    }
}
