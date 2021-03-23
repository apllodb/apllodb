mod insert_planner;
mod update_planner;

use std::sync::Arc;

use apllodb_shared_components::{
    ApllodbError, ApllodbResult, ApllodbSessionError, ApllodbSessionResult, Session, SessionWithTx,
};
use apllodb_sql_parser::apllodb_ast::Command;
use apllodb_storage_engine_interface::StorageEngine;

use self::{
    insert_planner::InsertPlanner,
    modification_executor::ModificationExecutor,
    modification_plan::{modification_plan_tree::ModificationPlanTree, ModificationPlan},
    update_planner::UpdatePlanner,
};

use super::sql_processor_context::SQLProcessorContext;

pub(crate) mod modification_executor;
pub(crate) mod modification_plan;

/// Processes ÎNSERT/UPDATE/DELETE command.
#[derive(Debug, new)]
pub(crate) struct ModificationProcessor<Engine: StorageEngine> {
    context: Arc<SQLProcessorContext<Engine>>,
}

impl<Engine: StorageEngine> ModificationProcessor<Engine> {}

impl<Engine: StorageEngine> ModificationProcessor<Engine> {
    /// Executes parsed INSERT/UPDATE/DELETE command.
    pub async fn run(
        &self,
        session: SessionWithTx,
        command: Command,
    ) -> ApllodbSessionResult<SessionWithTx> {
        match command {
            Command::InsertCommandVariant(ic) => {
                let planner = InsertPlanner::new(&self.context.node_repo, ic);
                let plan_tree_res = planner.run();
                self.run_plan_tree(session, plan_tree_res).await
            }
            Command::UpdateCommandVariant(uc) => {
                let planner = UpdatePlanner::new(uc);
                let plan_tree_res = planner.run();
                self.run_plan_tree(session, plan_tree_res).await
            }
            _ => Err(ApllodbSessionError::new(
                ApllodbError::feature_not_supported("only INSERT is supported for DML currently"),
                Session::from(session),
            )),
        }
    }

    async fn run_plan_tree(
        &self,
        session: SessionWithTx,
        plan_tree_res: ApllodbResult<ModificationPlanTree>,
    ) -> ApllodbSessionResult<SessionWithTx> {
        match plan_tree_res {
            Ok(plan) => {
                let executor = ModificationExecutor::new(self.context.clone());
                executor.run(session, ModificationPlan::new(plan)).await
            }
            Err(e) => Err(ApllodbSessionError::new(e, Session::from(session))),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::sql_processor::sql_processor_context::SQLProcessorContext;
    use apllodb_shared_components::{
        test_support::test_models::People, ApllodbResult, ColumnName, NNSqlValue, SqlValue,
        SqlValues, TableName,
    };
    use apllodb_sql_parser::ApllodbSqlParser;
    use apllodb_storage_engine_interface::test_support::{default_mock_engine, MockWithTxMethods};
    use futures::FutureExt;
    use mockall::predicate::{always, eq};
    use once_cell::sync::Lazy;

    use super::ModificationProcessor;

    #[derive(Clone, PartialEq, Debug, new)]
    struct TestDatum {
        in_insert_sql: &'static str,
        expected_insert_table: TableName,
        expected_insert_columns: Vec<ColumnName>,
        expected_insert_values: Vec<SqlValues>,
    }

    #[async_std::test]
    #[allow(clippy::redundant_clone)]
    async fn test_modification_processor_with_sql() -> ApllodbResult<()> {
        let parser = ApllodbSqlParser::default();

        static TEST_DATA: Lazy<Box<[TestDatum]>> = Lazy::new(|| {
            vec![TestDatum::new(
                "INSERT INTO people (id, age) VALUES (1, 13)",
                People::table_name(),
                vec![
                    People::ffr_id().as_column_name().clone(),
                    People::ffr_age().as_column_name().clone(),
                ],
                vec![SqlValues::new(vec![
                    SqlValue::NotNull(NNSqlValue::Integer(1)),
                    SqlValue::NotNull(NNSqlValue::Integer(13)),
                ])],
            )]
            .into_boxed_slice()
        });

        for test_datum in TEST_DATA.iter() {
            log::debug!("testing with SQL: {}", test_datum.in_insert_sql);

            // mocking insert()
            let mut engine = default_mock_engine();

            engine.expect_with_tx().returning(move || {
                let test_datum = test_datum.clone();

                let mut with_tx = MockWithTxMethods::new();
                with_tx
                    .expect_insert()
                    .with(
                        always(),
                        eq(test_datum.expected_insert_table),
                        eq(test_datum.expected_insert_columns),
                        eq(test_datum.expected_insert_values),
                    )
                    .returning(|session, _, _, _| async { Ok(session) }.boxed_local());
                with_tx
            });

            let context = Arc::new(SQLProcessorContext::new(engine));

            let ast = parser.parse(test_datum.in_insert_sql).unwrap();
            ModificationProcessor::run_directly(context.clone(), ast.0).await?;
        }

        Ok(())
    }
}
