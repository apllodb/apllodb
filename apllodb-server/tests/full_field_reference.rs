mod sql_test;

use apllodb_server::test_support::test_setup;
use apllodb_shared_components::{ApllodbErrorKind, FieldIndex};
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
        index: FieldIndex,
        expected_result: Result<(), ApllodbErrorKind>,
    }

    let test_data = vec![
        // TestDatum {
        //     sql: "SELECT id FROM people",
        //     index: FieldIndex::from("id"),
        //     expected_result: Ok(()),
        // },
        // TestDatum {
        //     sql: "SELECT id FROM people",
        //     index: FieldIndex::from("people.id"),
        //     expected_result: Ok(()),
        // },
        // TestDatum {
        //     sql: "SELECT id FROM people",
        //     index: FieldIndex::from("xxx"),
        //     expected_result: Err(ApllodbErrorKind::InvalidName),
        // },
        // TestDatum {
        //     sql: "SELECT id FROM people",
        //     index: FieldIndex::from("people.xxx"),
        //     expected_result: Err(ApllodbErrorKind::InvalidName),
        // },
        // TestDatum {
        //     sql: "SELECT id FROM people",
        //     index: FieldIndex::from("xxx.id"),
        //     expected_result: Err(ApllodbErrorKind::InvalidName),
        // },
        // TestDatum {
        //     sql: "SELECT people.id FROM people",
        //     index: FieldIndex::from("id"),
        //     expected_result: Ok(()),
        // },
        // TestDatum {
        //     sql: "SELECT people.id FROM people",
        //     index: FieldIndex::from("people.id"),
        //     expected_result: Ok(()),
        // },
        // TestDatum {
        //     sql: "SELECT id FROM people t_alias",
        //     index: FieldIndex::from("id"),
        //     expected_result: Ok(()),
        // },
        // TestDatum {
        //     sql: "SELECT id FROM people t_alias",
        //     index: FieldIndex::from("people.id"),
        //     expected_result: Ok(()),
        // },
        TestDatum {
            sql: "SELECT id FROM people t_alias",
            index: FieldIndex::from("t_alias.id"),
            expected_result: Ok(()),
        },
        // TestDatum {
        //     sql: "SELECT id c_alias FROM people",
        //     index: FieldIndex::from("id"),
        //     expected_result: Ok(()),
        // },
        // TestDatum {
        //     sql: "SELECT id c_alias FROM people",
        //     index: FieldIndex::from("c_alias"),
        //     expected_result: Ok(()),
        // },
        // TestDatum {
        //     sql: "SELECT id c_alias FROM people",
        //     index: FieldIndex::from("people.id"),
        //     expected_result: Ok(()),
        // },
        // TestDatum {
        //     sql: "SELECT id c_alias FROM people",
        //     index: FieldIndex::from("people.c_alias"),
        //     expected_result: Ok(()),
        // },
        // TestDatum {
        //     sql: "SELECT id c_alias FROM people t_alias",
        //     index: FieldIndex::from("c_alias"),
        //     expected_result: Ok(()),
        // },
        // TestDatum {
        //     sql: "SELECT id c_alias FROM people t_alias",
        //     index: FieldIndex::from("people.c_alias"),
        //     expected_result: Ok(()),
        // },
        // TestDatum {
        //     sql: "SELECT id c_alias FROM people t_alias",
        //     index: FieldIndex::from("t_alias.c_alias"),
        //     expected_result: Ok(()),
        // },
    ];

    let mut sql_test = SqlTest::default()
        .add_steps(Steps::SetupPeopleDataset)
        .add_step(Step::new("BEGIN", StepRes::Ok));

    for test_datum in test_data {
        sql_test = sql_test.add_step(Step::new(
            test_datum.sql,
            StepRes::OkQuery(Box::new(move |mut records| {
                let r = records.next().unwrap();

                match r.get::<i64>(&test_datum.clone().index) {
                    Ok(_) => assert!(
                        test_datum.expected_result.is_ok(),
                        format!(
                            "FieldIndex `{:?}` should be valid for Record::get() with this SQL: {}",
                            test_datum.index, test_datum.sql
                        )
                    ),
                    Err(e) => {
                        println!("{}", e);
                        assert_eq!(
                            e.kind(),
                            &test_datum.clone().expected_result.unwrap_err(),
                            "FieldIndex: `{:?}`, SQL: {}",
                            test_datum.index,
                            test_datum.sql
                        )
                    }
                }

                Ok(())
            })),
        ));
    }

    sql_test.run().await;
}
