pub(crate) mod response;
mod use_case;

use apllodb_immutable_schema_engine::ApllodbImmutableSchemaEngine;
use apllodb_shared_components::{ApllodbSessionResult, Session};

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
