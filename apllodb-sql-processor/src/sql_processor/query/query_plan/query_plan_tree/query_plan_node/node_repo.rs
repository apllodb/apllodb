use std::{
    collections::HashMap,
    sync::{Mutex, RwLock},
};

use apllodb_shared_components::{ApllodbError, ApllodbErrorKind, ApllodbResult};

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

    /// # Failures
    ///
    /// - [UndefinedObject](apllodb-shared-components::ApllodbErrorKind::UndefinedObject) when:
    ///    - no node has been created.
    pub(crate) fn latest_node_id(&self) -> ApllodbResult<QueryPlanNodeId> {
        self.hmap
            .read()
            .unwrap()
            .iter()
            .max_by(|(a_id, _), (b_id, _)| a_id.cmp(b_id))
            .map(|(id, _)| *id)
            .ok_or_else(|| {
                ApllodbError::new(
                    ApllodbErrorKind::UndefinedObject,
                    "no QueryPlanNode exists (already removed?)",
                    None,
                )
            })
    }

    /// # Failures
    ///
    /// - [UndefinedObject](apllodb-shared-components::ApllodbErrorKind::UndefinedObject) when:
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
                    .map(|corr_name| {
                        if correlation_name == &corr_name {
                            Some(*id)
                        } else {
                            None
                        }
                    })
                    .flatten()
            })
            .ok_or_else(|| {
                ApllodbError::new(
                    ApllodbErrorKind::UndefinedObject,
                    "no QueryPlanNode exists (already removed?)",
                    None,
                )
            })
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
