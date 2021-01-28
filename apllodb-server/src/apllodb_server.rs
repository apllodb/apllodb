pub(crate) mod response;
mod use_case;

use apllodb_immutable_schema_engine::ApllodbImmutableSchemaEngine;
use apllodb_shared_components::{ApllodbResult, ApllodbSessionResult, Session};

use std::rc::Rc;
use use_case::UseCase;

use crate::ApllodbCommandSuccess;

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
    pub async fn command(
        &self,
        session: Session,
        sql: String,
    ) -> ApllodbSessionResult<ApllodbCommandSuccess> {
        self.use_case().command(session, &sql).await
    }

    fn use_case(&self) -> UseCase<ApllodbImmutableSchemaEngine> {
        UseCase::new(self.engine.clone())
    }
}

#[cfg(any(test, feature = "test-support"))]
use apllodb_shared_components::{SessionWithDb, SessionWithTx};

#[cfg(any(test, feature = "test-support"))]
impl ApllodbServer {
    /// shortcut to CREATE / USE database
    pub async fn session_with_db(&self) -> ApllodbResult<SessionWithDb> {
        apllodb_storage_engine_interface::test_support::session_with_db(self.engine.as_ref()).await
    }

    /// shortcut to CREATE / USE database and BEGIN transaction
    pub async fn session_with_tx(&self) -> ApllodbResult<SessionWithTx> {
        apllodb_storage_engine_interface::test_support::session_with_tx(self.engine.as_ref()).await
    }
}
