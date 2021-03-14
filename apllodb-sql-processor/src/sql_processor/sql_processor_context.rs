use apllodb_storage_engine_interface::StorageEngine;

use super::query::query_plan::query_plan_tree::query_plan_node::node_repo::QueryPlanNodeRepository;

/// Context object each Processor/Executor has.
/// A context object must be moved out after an SQL process.
#[derive(Debug)]
pub struct SQLProcessorContext<Engine: StorageEngine> {
    pub(crate) engine: Engine,
    pub(crate) node_repo: QueryPlanNodeRepository,
}

impl<Engine: StorageEngine> SQLProcessorContext<Engine> {
    /// Constructor
    pub fn new(engine: Engine) -> Self {
        Self {
            engine,
            node_repo: QueryPlanNodeRepository::default(),
        }
    }
}
