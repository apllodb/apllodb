pub(crate) mod naive_query_planner;
pub(crate) mod query_executor;
pub(crate) mod query_plan;

use apllodb_shared_components::{
    ApllodbSessionError, ApllodbSessionResult, Session, SessionWithTx,
};
use apllodb_sql_parser::apllodb_ast::SelectCommand;
use apllodb_storage_engine_interface::StorageEngine;

use crate::records::Records;

use self::{
    naive_query_planner::NaiveQueryPlanner, query_executor::QueryExecutor, query_plan::QueryPlan,
};

use std::sync::Arc;

use super::sql_processor_context::SqlProcessorContext;

/// Processes SELECT command.
#[derive(Debug, new)]
pub(crate) struct QueryProcessor<Engine: StorageEngine> {
    context: Arc<SqlProcessorContext<Engine>>,
}

impl<Engine: StorageEngine> QueryProcessor<Engine> {
    /// Executes parsed SELECT query.
    pub async fn run(
        &self,
        session: SessionWithTx,
        select_command: SelectCommand,
    ) -> ApllodbSessionResult<(Records, SessionWithTx)> {
        // TODO query rewrite -> SelectCommand

        let planner = NaiveQueryPlanner::new(&self.context.node_repo, select_command);

        match planner.run() {
            Ok(plan) => {
                let executor = QueryExecutor::new(self.context.clone());
                executor.run(session, QueryPlan::new(plan)).await
                // TODO plan optimization -> QueryPlan
            }
            Err(e) => Err(ApllodbSessionError::new(e, Session::from(session))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::QueryProcessor;
    use crate::{
        records::record::Record, sql_processor::sql_processor_context::SqlProcessorContext,
    };
    use apllodb_shared_components::ApllodbResult;
    use apllodb_sql_parser::{apllodb_ast::Command, ApllodbSqlParser};
    use apllodb_storage_engine_interface::test_support::{
        default_mock_engine, mock_select,
        test_models::{ModelsMock, People},
        MockWithTxMethods,
    };
    use pretty_assertions::assert_eq;
    use std::sync::Arc;

    #[derive(Clone, PartialEq, Debug)]
    struct TestDatum {
        in_select_sql: String,
        expected_select_records: Vec<Record>,
    }
    impl TestDatum {
        fn new(in_select_sql: &str, expected_select_records: Vec<Record>) -> Self {
            Self {
                in_select_sql: in_select_sql.to_string(),
                expected_select_records,
            }
        }
    }

    #[async_std::test]
    #[allow(clippy::redundant_clone)]
    async fn test_query_processor_with_sql() -> ApllodbResult<()> {
        let parser = ApllodbSqlParser::default();

        let mut engine = default_mock_engine();
        engine.expect_with_tx().returning(|| {
            let mut with_tx = MockWithTxMethods::new();

            // mocking select()
            mock_select(&mut with_tx, ModelsMock::fx_full());

            with_tx
        });
        let context = Arc::new(SqlProcessorContext::new(engine));

        let test_data: Vec<TestDatum> = vec![
            // full scan
            TestDatum::new(
                "SELECT id, age FROM people",
                vec![
                    Record::fx_people1().projection(
                        &vec![People::tc_id().into(), People::tc_age().into()]
                            .into_iter()
                            .collect(),
                    )?,
                    Record::fx_people2().projection(
                        &vec![People::tc_id().into(), People::tc_age().into()]
                            .into_iter()
                            .collect(),
                    )?,
                    Record::fx_people3().projection(
                        &vec![People::tc_id().into(), People::tc_age().into()]
                            .into_iter()
                            .collect(),
                    )?,
                ],
            ),
            // projection
            TestDatum::new(
                "SELECT id FROM people",
                vec![
                    Record::fx_people1()
                        .projection(&vec![People::tc_id().into()].into_iter().collect())?,
                    Record::fx_people2()
                        .projection(&vec![People::tc_id().into()].into_iter().collect())?,
                    Record::fx_people3()
                        .projection(&vec![People::tc_id().into()].into_iter().collect())?,
                ],
            ),
            TestDatum::new(
                "SELECT age FROM people",
                vec![
                    Record::fx_people1()
                        .projection(&vec![People::tc_age().into()].into_iter().collect())?,
                    Record::fx_people2()
                        .projection(&vec![People::tc_age().into()].into_iter().collect())?,
                    Record::fx_people3()
                        .projection(&vec![People::tc_age().into()].into_iter().collect())?,
                ],
            ),
        ];

        for test_datum in test_data {
            log::debug!("testing with input SQL: {}", test_datum.in_select_sql);

            let ast = parser.parse(test_datum.in_select_sql).unwrap();
            let select_command = match ast.0 {
                Command::SelectCommandVariant(sc) => sc,
                _ => {
                    panic!("only SELECT is acceptable for this test")
                }
            };
            let result = QueryProcessor::run_directly(context.clone(), select_command).await?;

            assert_eq!(
                result.collect::<Vec<Record>>(),
                test_datum.expected_select_records,
            );
        }
        Ok(())
    }
}
