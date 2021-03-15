pub(crate) mod response;
mod use_case;

use apllodb_immutable_schema_engine::ApllodbImmutableSchemaEngine;
use apllodb_shared_components::{ApllodbSessionResult, Session};
use apllodb_sql_processor::SQLProcessorContext;

use std::sync::Arc;
use use_case::UseCase;

use crate::ApllodbCommandSuccess;

#[derive(Clone, Debug)]
pub struct ApllodbServer {
    context: Arc<SQLProcessorContext<ApllodbImmutableSchemaEngine>>,
}

impl Default for ApllodbServer {
    fn default() -> Self {
        let engine = ApllodbImmutableSchemaEngine::default();
        let context = Arc::new(SQLProcessorContext::new(engine));
        Self { context }
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
        UseCase::new(self.context.clone())
    }
}
