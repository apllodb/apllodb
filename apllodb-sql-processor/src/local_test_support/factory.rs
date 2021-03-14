use crate::{
    sql_processor::{
        ddl::DDLProcessor,
        modification::{
            modification_executor::ModificationExecutor, modification_plan::ModificationPlan,
            ModificationProcessor,
        },
        query::{query_executor::QueryExecutor, query_plan::QueryPlan, QueryProcessor},
    },
    SQLProcessorContext,
};
use apllodb_shared_components::{ApllodbError, ApllodbResult, Records};
use apllodb_sql_parser::apllodb_ast;
use apllodb_storage_engine_interface::{test_support::session_with_tx, MockStorageEngine};
use std::sync::Arc;

impl QueryProcessor<MockStorageEngine> {
    pub async fn run_directly(
        context: Arc<SQLProcessorContext<MockStorageEngine>>,
        select_command: apllodb_ast::SelectCommand,
    ) -> ApllodbResult<Records> {
        let session = session_with_tx(&context.engine).await?;

        let processor = Self::new(context.clone());
        processor
            .run(session, select_command)
            .await
            .map(|(records, _)| records)
            .map_err(ApllodbError::from)
    }
}

impl ModificationProcessor<MockStorageEngine> {
    pub async fn run_directly(
        context: Arc<SQLProcessorContext<MockStorageEngine>>,
        command: apllodb_ast::Command,
    ) -> ApllodbResult<()> {
        let session = session_with_tx(&context.engine).await?;

        let processor = Self::new(context.clone());
        processor
            .run(session, command)
            .await
            .map(|_| ())
            .map_err(ApllodbError::from)
    }
}

impl DDLProcessor<MockStorageEngine> {
    pub async fn run_directly(
        context: Arc<SQLProcessorContext<MockStorageEngine>>,
        command: apllodb_ast::Command,
    ) -> ApllodbResult<()> {
        let session = session_with_tx(&context.engine).await?;

        let processor = Self::new(context.clone());
        processor
            .run(session, command)
            .await
            .map(|_| ())
            .map_err(ApllodbError::from)
    }
}

impl QueryExecutor<MockStorageEngine> {
    pub async fn run_directly(
        context: Arc<SQLProcessorContext<MockStorageEngine>>,
        plan: QueryPlan,
    ) -> ApllodbResult<Records> {
        let session = session_with_tx(&context.engine).await?;

        let executor = Self::new(context.clone());
        executor
            .run(session, plan)
            .await
            .map(|(records, _)| records)
            .map_err(ApllodbError::from)
    }
}

impl ModificationExecutor<MockStorageEngine> {
    pub async fn run_directly(
        context: Arc<SQLProcessorContext<MockStorageEngine>>,
        plan: ModificationPlan,
    ) -> ApllodbResult<()> {
        let session = session_with_tx(&context.engine).await?;

        let executor = Self::new(context.clone());
        executor
            .run(session, plan)
            .await
            .map(|_| ())
            .map_err(ApllodbError::from)
    }
}
