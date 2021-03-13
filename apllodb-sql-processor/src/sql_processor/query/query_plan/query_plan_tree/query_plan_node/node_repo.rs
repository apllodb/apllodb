use apllodb_shared_components::ApllodbResult;

use super::{node_id::QueryPlanNodeId, QueryPlanNode};

#[derive(Debug, Default)]
pub(crate) struct QueryPlanNodeRepository {}

impl QueryPlanNodeRepository {
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
