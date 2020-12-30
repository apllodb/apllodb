pub(crate) mod query_executor;
pub(crate) mod query_plan;

use apllodb_shared_components::{ApllodbResult, RecordIterator};
use apllodb_sql_parser::apllodb_ast::SelectCommand;
use apllodb_storage_engine_interface::StorageEngine;

use self::{query_executor::QueryExecutor, query_plan::QueryPlan};

use std::convert::TryFrom;

/// Processes SELECT command.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, new)]
pub struct QueryProcessor<'exe, Engine: StorageEngine> {
    tx: &'exe Engine::Tx,
}

impl<'exe, Engine: StorageEngine> QueryProcessor<'exe, Engine> {
    /// Executes parsed SELECT query.
    pub fn run(&self, select_command: SelectCommand) -> ApllodbResult<RecordIterator> {
        // TODO query rewrite -> SelectCommand

        let plan = QueryPlan::try_from(select_command)?;

        // TODO plan optimization -> QueryPlan

        let executor = QueryExecutor::<'_, Engine>::new(self.tx);
        executor.run(plan)
    }
}

#[cfg(test)]
mod tests {
    use apllodb_shared_components::{ApllodbResult, Record};
    use apllodb_sql_parser::{apllodb_ast::Command, ApllodbSqlParser};
    use pretty_assertions::assert_eq;

    use crate::{
        test_support::{
            mock_tx::mock_tx_select::mock_select_with_models::{
                mock_select_with_models, ModelsMock,
            },
            setup,
            test_models::{Body, People, Pet},
            test_storage_engine::TestStorageEngine,
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

        let mut tx = TestStorageEngine::begin()?;

        mock_select_with_models(
            &mut tx,
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
        let processor = QueryProcessor::<'_, TestStorageEngine>::new(&tx);

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

            let result = processor.run(select_command)?;

            assert_eq!(
                result.collect::<Vec<Record>>(),
                test_datum.expected_select_records,
            );
        }
        Ok(())
    }
}
