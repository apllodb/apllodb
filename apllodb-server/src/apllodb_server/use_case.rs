mod sql_processor_response;

use std::rc::Rc;

use apllodb_shared_components::{ApllodbSessionResult, Session};
use apllodb_sql_processor::SQLProcessor;
use apllodb_storage_engine_interface::StorageEngine;

use crate::ApllodbCommandSuccess;

use self::sql_processor_response::to_server_resp;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, new)]
pub(in crate::apllodb_server) struct UseCase<Engine: StorageEngine> {
    engine: Rc<Engine>,
}

impl<Engine: StorageEngine> UseCase<Engine> {
    pub(in crate::apllodb_server) async fn command(
        &self,
        session: Session,
        sql: &str,
    ) -> ApllodbSessionResult<ApllodbCommandSuccess> {
        let sql_proc = SQLProcessor::new(self.engine.clone());
        let sql_proc_succ = sql_proc.run(session, sql).await?;
        Ok(to_server_resp(sql_proc_succ))
    }
}
