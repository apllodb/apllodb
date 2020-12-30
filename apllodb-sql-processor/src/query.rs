pub(crate) mod query_executor;
pub(crate) mod query_plan;

use apllodb_shared_components::{ApllodbResult, RecordIterator};
use apllodb_sql_parser::apllodb_ast::SelectCommand;
use apllodb_storage_engine_interface::StorageEngine;

use self::{query_executor::QueryExecutor, query_plan::QueryPlan};

use std::convert::TryFrom;

/// Processes SELECT command.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, new)]
pub struct QueryProcessor<'exe, Engine: StorageEngine> {
    tx: &'exe Engine::Tx,
}

impl<'exe, Engine: StorageEngine> QueryProcessor<'exe, Engine> {
    /// Executes parsed SELECT query.
    pub fn run(&self, select_command: SelectCommand) -> ApllodbResult<RecordIterator> {
        // TODO query rewrite -> SelectCommand

        let plan = QueryPlan::try_from(select_command)?;

        // TODO plan optimization -> QueryPlan

        let executor = QueryExecutor::<'_, Engine>::new(self.tx);
        executor.run(plan)
    }
}
