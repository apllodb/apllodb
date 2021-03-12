pub(crate) mod query_executor;
pub(crate) mod query_plan;

use apllodb_shared_components::{
    ApllodbSessionError, ApllodbSessionResult, Records, Session, SessionWithTx,
};
use apllodb_sql_parser::apllodb_ast::SelectCommand;
use apllodb_storage_engine_interface::StorageEngine;

use self::{
    query_executor::QueryExecutor,
    query_plan::{query_plan_tree::query_plan_node::node_id::QueryPlanNodeIdGenerator, QueryPlan},
};

use std::{rc::Rc, sync::Arc};

/// Processes SELECT command.
#[derive(Eq, PartialEq, Hash, Debug, new)]
pub(crate) struct QueryProcessor<Engine: StorageEngine> {
    engine: Rc<Engine>,
    id_gen: Arc<QueryPlanNodeIdGenerator>,
}

impl<Engine: StorageEngine> QueryProcessor<Engine> {
    /// Executes parsed SELECT query.
    pub async fn run(
        &self,
        session: SessionWithTx,
        select_command: SelectCommand,
    ) -> ApllodbSessionResult<(Records, SessionWithTx)> {
        // TODO query rewrite -> SelectCommand

        match QueryPlan::from_select_command(self.id_gen.clone(), select_command) {
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
    use std::{rc::Rc, sync::Arc};

    use super::QueryProcessor;
    use crate::sql_processor::query::query_plan::query_plan_tree::query_plan_node::node_id::QueryPlanNodeIdGenerator;
    use apllodb_shared_components::{
        test_support::{fixture::*, test_models::People},
        ApllodbResult, Record,
    };
    use apllodb_sql_parser::{apllodb_ast::Command, ApllodbSqlParser};
    use apllodb_storage_engine_interface::test_support::{
        default_mock_engine, mock_select, session_with_tx, MockWithTxMethods,
    };

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
                    PEOPLE_RECORD1.clone(),
                    PEOPLE_RECORD2.clone(),
                    PEOPLE_RECORD3.clone(),
                ],
            ),
            // projection
            TestDatum::new(
                "SELECT id FROM people",
                vec![
                    PEOPLE_RECORD1
                        .clone()
                        .projection(&[People::field_idx(People::ffr_id())])?,
                    PEOPLE_RECORD2
                        .clone()
                        .projection(&[People::field_idx(People::ffr_id())])?,
                    PEOPLE_RECORD3
                        .clone()
                        .projection(&[People::field_idx(People::ffr_id())])?,
                ],
            ),
            TestDatum::new(
                "SELECT age FROM people",
                vec![
                    PEOPLE_RECORD1
                        .clone()
                        .projection(&[People::field_idx(People::ffr_age())])?,
                    PEOPLE_RECORD2
                        .clone()
                        .projection(&[People::field_idx(People::ffr_age())])?,
                    PEOPLE_RECORD3
                        .clone()
                        .projection(&[People::field_idx(People::ffr_age())])?,
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
            let id_gen = QueryPlanNodeIdGenerator::new();
            let processor = QueryProcessor::new(engine.clone(), Arc::new(id_gen));
            let (result, _) = processor.run(session, select_command).await?;

            assert_eq!(
                result.collect::<Vec<Record>>(),
                test_datum.expected_select_records,
            );
        }
        Ok(())
    }
}
