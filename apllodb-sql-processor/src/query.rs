pub(crate) mod query_executor;
pub(crate) mod query_plan;

use apllodb_shared_components::{ApllodbResult, RecordIterator, SessionWithTx};
use apllodb_sql_parser::apllodb_ast::SelectCommand;

use self::{query_executor::QueryExecutor, query_plan::QueryPlan};

use std::convert::TryFrom;

/// Processes SELECT command.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, new)]
pub struct QueryProcessor;

impl QueryProcessor {
    /// Executes parsed SELECT query.
    pub fn run(
        &self,
        session: &SessionWithTx,
        select_command: SelectCommand,
    ) -> ApllodbResult<RecordIterator> {
        // TODO query rewrite -> SelectCommand

        let plan = QueryPlan::try_from(select_command)?;

        // TODO plan optimization -> QueryPlan

        let executor = QueryExecutor::new(&self.dml_methods);
        // executor.run(tx, plan)
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use apllodb_shared_components::{ApllodbResult, Record};
    use apllodb_sql_parser::{apllodb_ast::Command, ApllodbSqlParser};
    use pretty_assertions::assert_eq;

    use crate::{
        test_support::{
            mock_dml::{
                mock_tx_select::mock_select_with_models::{mock_select_with_models, ModelsMock},
                MockDML,
            },
            setup,
            test_models::{Body, People, Pet},
            test_storage_engine::{TestStorageEngine, TestTx},
            utility_functions::r_projection,
        },
        QueryProcessor,
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

    #[test]
    #[allow(clippy::redundant_clone)]
    fn test_query_processor_with_sql() -> ApllodbResult<()> {
        setup();

        let t_people_r1 = People::record(1, 13);
        let t_people_r2 = People::record(2, 70);
        let t_people_r3 = People::record(3, 35);

        let t_body_r1 = Body::record(1, 145);
        let t_body_r3 = Body::record(3, 175);

        let t_pet_r1 = Pet::record(1, "dog", 13);
        let t_pet_r3_1 = Pet::record(3, "dog", 5);
        let t_pet_r3_2 = Pet::record(3, "cat", 3);

        let mut tx = TestTx;
        let mut dml = MockDML::new();

        mock_select_with_models(
            &mut dml,
            ModelsMock {
                people: vec![
                    t_people_r1.clone(),
                    t_people_r2.clone(),
                    t_people_r3.clone(),
                ],
                body: vec![t_body_r1.clone(), t_body_r3.clone()],
                pet: vec![t_pet_r1.clone(), t_pet_r3_1.clone(), t_pet_r3_2.clone()],
            },
        );

        let parser = ApllodbSqlParser::new();
        let processor = QueryProcessor::<TestStorageEngine>::new(&dml);

        let test_data: Vec<TestDatum> = vec![
            // full scan
            TestDatum::new(
                "SELECT id, age FROM people",
                vec![
                    t_people_r1.clone(),
                    t_people_r2.clone(),
                    t_people_r3.clone(),
                ],
            ),
            // projection
            TestDatum::new(
                "SELECT id FROM people",
                vec![
                    r_projection(t_people_r1.clone(), vec![People::colref_id()])?,
                    r_projection(t_people_r2.clone(), vec![People::colref_id()])?,
                    r_projection(t_people_r3.clone(), vec![People::colref_id()])?,
                ],
            ),
            TestDatum::new(
                "SELECT age FROM people",
                vec![
                    r_projection(t_people_r1.clone(), vec![People::colref_age()])?,
                    r_projection(t_people_r2.clone(), vec![People::colref_age()])?,
                    r_projection(t_people_r3.clone(), vec![People::colref_age()])?,
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

            let result = processor.run(&mut tx, select_command)?;

            assert_eq!(
                result.collect::<Vec<Record>>(),
                test_datum.expected_select_records,
            );
        }
        Ok(())
    }
}
