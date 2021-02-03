mod sql_test;

use apllodb_server::test_support::test_setup;
use apllodb_shared_components::{ApllodbErrorKind, FullFieldReference};
use sql_test::{SqlTest, Step, StepRes, Steps};

#[ctor::ctor]
fn setup() {
    test_setup();
}

#[async_std::test]
async fn test_select_with_various_field_spec() {
    #[derive(Clone)]
    struct TestDatum {
        sql: &'static str,
        ffr: FullFieldReference,
        expected_result: Result<(), ApllodbErrorKind>,
    }

    let test_data = vec![
        TestDatum {
            sql: "SELECT id FROM people",
            ffr: FullFieldReference::factory("people", "id"),
            expected_result: Ok(()),
        },
        TestDatum {
            sql: "SELECT people.id FROM people",
            ffr: FullFieldReference::factory("people", "id"),
            expected_result: Ok(()),
        },
    ];

    let mut sql_test = SqlTest::default()
        .add_steps(Steps::SetupPeopleDataset)
        .add_step(Step::new("BEGIN", StepRes::Ok));

    for test_datum in test_data {
        sql_test = sql_test.add_step(Step::new(
            test_datum.sql,
            StepRes::OkQuery(Box::new(move |mut records| {
                let r = records.next().unwrap();

                match r.get::<i64>(&test_datum.clone().ffr.into_field_index()) {
                    Ok(_) => assert!(
                        test_datum.expected_result.is_ok(),
                        format!(
                            "FieldIndex `{:?}` should be valid for Record::get() with this SQL: {}",
                            test_datum.ffr, test_datum.sql
                        )
                    ),
                    Err(e) => {
                        assert_eq!(e.kind(), &test_datum.clone().expected_result.unwrap_err())
                    }
                }

                Ok(())
            })),
        ));
    }

    sql_test.run().await;
}
