mod sql_processor_response;

use std::rc::Rc;

use apllodb_rpc_interface::ApllodbRpcSuccess;
use apllodb_shared_components::{ApllodbResult, DatabaseName, Session, SessionWithTx};
use apllodb_sql_parser::ApllodbSqlParser;
use apllodb_sql_processor::SQLProcessor;
use apllodb_storage_engine_interface::StorageEngine;

use self::sql_processor_response::to_rpc_success;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, new)]
pub(in crate::apllodb_server) struct UseCase<Engine: StorageEngine> {
    engine: Rc<Engine>,
}

impl<Engine: StorageEngine> UseCase<Engine> {
    pub(in crate::apllodb_server) async fn begin_transaction(
        self,
        database: DatabaseName,
    ) -> ApllodbResult<SessionWithTx> {
        todo!()
    }

    pub(in crate::apllodb_server) async fn command(
        self,
        session: Session,
        sql: &str,
    ) -> ApllodbResult<ApllodbRpcSuccess> {
        let parser = ApllodbSqlParser::new();
        let sql_proc = SQLProcessor::new(self.engine.clone());
        let sql_proc_succ = sql_proc.run(session, sql).await?;
        Ok(to_rpc_success(sql_proc_succ))
    }
}
