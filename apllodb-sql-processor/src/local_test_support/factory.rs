use crate::{
    sql_processor::{
        ddl::DdlProcessor,
        modification::ModificationProcessor,
        query::{query_executor::QueryExecutor, query_plan::QueryPlan, QueryProcessor},
    },
    SqlProcessorContext,
};
use apllodb_immutable_schema_engine_infra::test_support::session_with_tx;
use apllodb_shared_components::{ApllodbError, ApllodbResult, Records};
use apllodb_sql_parser::apllodb_ast;
use apllodb_storage_engine_interface::MockStorageEngine;
use std::sync::Arc;

impl QueryProcessor<MockStorageEngine> {
    pub async fn run_directly(
        context: Arc<SqlProcessorContext<MockStorageEngine>>,
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
        context: Arc<SqlProcessorContext<MockStorageEngine>>,
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

impl DdlProcessor<MockStorageEngine> {
    pub async fn run_directly(
        context: Arc<SqlProcessorContext<MockStorageEngine>>,
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
        context: Arc<SqlProcessorContext<MockStorageEngine>>,
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
