use apllodb_shared_components::ApllodbResult;
use apllodb_storage_engine_interface::StorageEngine;

use self::{
    ddl::DDLProcessor, modification::ModificationProcessor, query::QueryProcessor,
    success::SQLProcessorSuccess,
};

pub(crate) mod ddl;
pub(crate) mod modification;
pub(crate) mod query;

mod success;

/// Processes SQL.
#[derive(Clone, Debug)]
pub struct SQLProcessor<'sqlp, Engine: StorageEngine> {
    query_processor: &'sqlp QueryProcessor<'sqlp, Engine>,
    modification_processor: &'sqlp ModificationProcessor<'sqlp, Engine>,
    ddl_proessor: &'sqlp DDLProcessor<'sqlp, Engine>,
    // TODO コネクションオブジェクトみたいなのがないと、既に開いている tx を参照などできない。
}

impl<Engine: StorageEngine> SQLProcessor<'_, Engine> {
    pub fn new(conn: Engine::Conn) -> Self {}

    pub fn run(&self, sql: &str) -> ApllodbResult<SQLProcessorSuccess> {}
}
