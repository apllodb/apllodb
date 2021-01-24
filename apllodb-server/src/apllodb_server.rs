mod use_case;

use apllodb_immutable_schema_engine::ApllodbImmutableSchemaEngine;
use apllodb_rpc_interface::ApllodbSuccess;
use apllodb_shared_components::{ApllodbResult, DatabaseName, Session, SessionWithTx};

use std::rc::Rc;
use use_case::UseCase;

#[derive(Clone, Debug)]
pub struct ApllodbServer {
    use_case: Rc<UseCase<ApllodbImmutableSchemaEngine>>,
}

impl Default for ApllodbServer {
    fn default() -> Self {
        let engine = Rc::new(ApllodbImmutableSchemaEngine::default());
        let use_case = Rc::new(UseCase::new(engine));
        Self { use_case }
    }
}

impl ApllodbServer {
    pub async fn begin_transaction(self, database: DatabaseName) -> ApllodbResult<SessionWithTx> {
        self.use_case.begin_transaction(database).await
    }

    pub async fn command(self, session: Session, sql: String) -> ApllodbResult<ApllodbSuccess> {
        self.use_case.command(session, &sql).await
    }
}
