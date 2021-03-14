use crate::sql_processor::query::{
    query_executor::QueryExecutor,
    query_plan::{
        query_plan_tree::query_plan_node::{
            node_id::QueryPlanNodeId, node_repo::QueryPlanNodeRepository,
        },
        QueryPlan,
    },
};
use apllodb_storage_engine_interface::MockStorageEngine;
use std::{rc::Rc, sync::Arc};

impl QueryExecutor<MockStorageEngine> {
    pub fn factory(engine: MockStorageEngine) -> Self {
        let engine = Rc::new(engine);
        let repo = Arc::new(QueryPlanNodeRepository::default());
        Self::new(engine.clone(), repo.clone())
    }
}

impl QueryPlan {
    pub fn factory_with_repo(f: impl FnOnce(&QueryPlanNodeRepository) -> QueryPlanNodeId) -> Self {
        f()
    }
}
