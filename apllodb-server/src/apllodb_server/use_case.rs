mod sql_processor_response;

use std::rc::Rc;

use apllodb_shared_components::{
    ApllodbResult, DatabaseName, Session, SessionWithTx, SessionWithoutDb,
};
use apllodb_sql_processor::SQLProcessor;
use apllodb_storage_engine_interface::{
    StorageEngine, WithDbMethods, WithTxMethods, WithoutDbMethods,
};

use crate::ApllodbSuccess;

use self::sql_processor_response::to_server_resp;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, new)]
pub(in crate::apllodb_server) struct UseCase<Engine: StorageEngine> {
    engine: Rc<Engine>,
}

impl<Engine: StorageEngine> UseCase<Engine> {
    pub(in crate::apllodb_server) async fn begin_transaction(
        &self,
        database: DatabaseName,
    ) -> ApllodbResult<SessionWithTx> {
        let session = self
            .engine
            .without_db()
            .use_database(SessionWithoutDb::default(), database)
            .await?;

        let session = self.engine.with_db().begin_transaction(session).await?;

        Ok(session)
    }

    pub(in crate::apllodb_server) async fn commit_transaction(
        &self,
        session: SessionWithTx,
    ) -> ApllodbResult<()> {
        self.engine.with_tx().commit_transaction(session).await?;
        Ok(())
    }

    pub(in crate::apllodb_server) async fn command(
        &self,
        session: Session,
        sql: &str,
    ) -> ApllodbResult<ApllodbSuccess> {
        let sql_proc = SQLProcessor::new(self.engine.clone());
        let sql_proc_succ = sql_proc.run(session, sql).await?;
        Ok(to_server_resp(sql_proc_succ))
    }
}
