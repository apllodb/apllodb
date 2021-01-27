pub(crate) mod response;
mod use_case;

use apllodb_immutable_schema_engine::ApllodbImmutableSchemaEngine;
use apllodb_shared_components::{ApllodbResult, DatabaseName, Session, SessionWithTx};

use std::rc::Rc;
use use_case::UseCase;

use crate::ApllodbSuccess;

#[derive(Clone, Debug)]
pub struct ApllodbServer {
    engine: Rc<ApllodbImmutableSchemaEngine>,
}

impl Default for ApllodbServer {
    fn default() -> Self {
        let engine = Rc::new(ApllodbImmutableSchemaEngine::default());
        Self { engine }
    }
}

impl ApllodbServer {
    pub async fn begin_transaction(&self, database: DatabaseName) -> ApllodbResult<SessionWithTx> {
        self.use_case().begin_transaction(database).await
    }

    pub async fn commit_transaction(&self, session: SessionWithTx) -> ApllodbResult<()> {
        self.use_case().commit_transaction(session).await
    }

    pub async fn command(&self, session: Session, sql: String) -> ApllodbResult<ApllodbSuccess> {
        self.use_case().command(session, &sql).await
    }

    fn use_case(&self) -> UseCase<ApllodbImmutableSchemaEngine> {
        UseCase::new(self.engine.clone())
    }
}
