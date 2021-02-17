pub(crate) mod query_executor;
pub(crate) mod query_plan;

use apllodb_shared_components::{
    ApllodbSessionError, ApllodbSessionResult, RecordIterator, Session, SessionWithTx,
};
use apllodb_sql_parser::apllodb_ast::SelectCommand;
use apllodb_storage_engine_interface::StorageEngine;

use self::{query_executor::QueryExecutor, query_plan::QueryPlan};

use std::{convert::TryFrom, rc::Rc};

/// Processes SELECT command.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct QueryProcessor<Engine: StorageEngine> {
    engine: Rc<Engine>,
}

impl<Engine: StorageEngine> QueryProcessor<Engine> {
    pub(crate) fn new(engine: Rc<Engine>) -> Self {
        Self { engine }
    }

    /// Executes parsed SELECT query.
    pub async fn run(
        &self,
        session: SessionWithTx,
        select_command: SelectCommand,
    ) -> ApllodbSessionResult<(RecordIterator, SessionWithTx)> {
        // TODO query rewrite -> SelectCommand

        match QueryPlan::try_from(select_command) {
            Ok(plan) => {
                let executor = QueryExecutor::new(self.engine.clone());
                executor.run(session, plan).await
                // TODO plan optimization -> QueryPlan
            }
            Err(e) => Err(ApllodbSessionError::new(e, Session::from(session))),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::test_support::utility_functions::r_projection;
    use apllodb_shared_components::{
        test_support::{fixture::*, test_models::People},
        ApllodbResult, Record,
    };
    use apllodb_sql_parser::{apllodb_ast::Command, ApllodbSqlParser};
    use apllodb_storage_engine_interface::test_support::{
        default_mock_engine, mock_select, session_with_tx, MockWithTxMethods,
    };

    use super::QueryProcessor;

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
            mock_select(&mut with_tx, &FULL_MODELS);

            with_tx
        });
        let engine = Rc::new(engine);

        let test_data: Vec<TestDatum> = vec![
            // full scan
            TestDatum::new(
                "SELECT id, age FROM people",
                vec![
                    T_PEOPLE_R1.clone(),
                    T_PEOPLE_R2.clone(),
                    T_PEOPLE_R3.clone(),
                ],
            ),
            // projection
            TestDatum::new(
                "SELECT id FROM people",
                vec![
                    r_projection(T_PEOPLE_R1.clone(), vec![People::ufr_id().into()])?,
                    r_projection(T_PEOPLE_R2.clone(), vec![People::ufr_id().into()])?,
                    r_projection(T_PEOPLE_R3.clone(), vec![People::ufr_id().into()])?,
                ],
            ),
            TestDatum::new(
                "SELECT age FROM people",
                vec![
                    r_projection(T_PEOPLE_R1.clone(), vec![People::ufr_age().into()])?,
                    r_projection(T_PEOPLE_R2.clone(), vec![People::ufr_age().into()])?,
                    r_projection(T_PEOPLE_R3.clone(), vec![People::ufr_age().into()])?,
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
            let session = session_with_tx(engine.as_ref()).await?;
            let processor = QueryProcessor::new(engine.clone());
            let (result, _) = processor.run(session, select_command).await?;

            assert_eq!(
                result.collect::<Vec<Record>>(),
                test_datum.expected_select_records,
            );
        }
        Ok(())
    }
}
