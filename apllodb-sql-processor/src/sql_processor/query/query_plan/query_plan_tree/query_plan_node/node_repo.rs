use std::{
    collections::HashMap,
    sync::{Mutex, RwLock},
};

use apllodb_shared_components::{ApllodbError, ApllodbResult};

use crate::correlation::correlation_name::CorrelationName;

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

    /// # Panics
    ///
    /// - no node has been created.
    pub(crate) fn latest_node_id(&self) -> QueryPlanNodeId {
        self.hmap
            .read()
            .unwrap()
            .iter()
            .max_by(|(a_id, _), (b_id, _)| a_id.cmp(b_id))
            .map(|(id, _)| *id)
            .expect("no QueryPlanNode exists (already removed?)")
    }

    /// # Failures
    ///
    /// - [UndefinedObject](apllodb-shared-components::SqlState::NameErrorNotFound) when:
    ///    - no matching node is found.
    pub(crate) fn find_correlation_node(
        &self,
        correlation_name: &CorrelationName,
    ) -> ApllodbResult<QueryPlanNodeId> {
        self.hmap
            .read()
            .unwrap()
            .iter()
            .find_map(|(id, node)| {
                node.source_correlation_name()
                    .map(|corr_name| (correlation_name == &corr_name).then(|| *id))
                    .flatten()
            })
            .ok_or_else(|| {
                ApllodbError::name_error_not_found("no QueryPlanNode exists (already removed?)")
            })
    }

    /// # Panics
    ///
    /// when node with id does not exist.
    pub(crate) fn remove(&self, id: QueryPlanNodeId) -> QueryPlanNode {
        self.hmap
            .write()
            .unwrap()
            .remove(&id)
            .unwrap_or_else(|| panic!("QueryPlanNode:{:?} does not exist (already removed?)", id))
    }
}
