mod sql_test;

use apllodb_server::test_support::test_setup;
use apllodb_shared_components::{test_support::fixture::*, ApllodbErrorKind, FieldIndex};
use itertools::Itertools;
use pretty_assertions::assert_eq;
use sql_test::{SqlTest, Step, StepRes, Steps};

#[ctor::ctor]
fn setup() {
    test_setup();
}

#[async_std::test]
async fn test_fullscan() {
    SqlTest::default()
        .add_steps(Steps::SetupPeopleDataset)
        .add_step(Step::new("BEGIN", StepRes::Ok))
        .add_step(Step::new(
            "SELECT id, age FROM people",
            StepRes::OkQuery(Box::new(|records| {
                let mut records = records
                    .sorted_by_key(|r| r.get::<i64>(&FieldIndex::from("id")).unwrap().unwrap());
                assert_eq!(records.next(), Some(T_PEOPLE_R1.clone()));
                assert_eq!(records.next(), Some(T_PEOPLE_R2.clone()));
                assert_eq!(records.next(), Some(T_PEOPLE_R3.clone()));
                assert!(records.next().is_none());
                Ok(())
            })),
        ))
        .run()
        .await;
}

#[async_std::test]
async fn test_projection() {
    SqlTest::default()
        .add_steps(Steps::SetupPeopleDataset)
        .add_step(Step::new("BEGIN", StepRes::Ok))
        .add_step(
            // projection to PK
            Step::new(
                "SELECT id FROM people",
                StepRes::OkQuery(Box::new(|records| {
                    let id_index = FieldIndex::from("id");

                    let mut records =
                        records.sorted_by_key(|r| r.get::<i64>(&id_index).unwrap().unwrap());

                    let r = records.next().unwrap();
                    assert_eq!(
                        r.get::<i64>(&id_index).unwrap(),
                        T_PEOPLE_R1.get::<i64>(&id_index).unwrap()
                    );
                    assert_eq!(
                        r.get::<i32>(&FieldIndex::from("age")).unwrap_err().kind(),
                        &ApllodbErrorKind::InvalidName
                    );

                    Ok(())
                })),
            ),
        )
        .add_step(
            // projection to non-PK
            Step::new(
                "SELECT age FROM people",
                StepRes::OkQuery(Box::new(|records| {
                    let age_index = FieldIndex::from("age");
                    let mut records =
                        records.sorted_by_key(|r| r.get::<i32>(&age_index).unwrap().unwrap());
                    let r = records.next().unwrap();
                    assert_eq!(
                        r.get::<i32>(&age_index).unwrap(),
                        T_PEOPLE_R1.get::<i32>(&age_index).unwrap()
                    );
                    assert_eq!(
                        r.get::<i64>(&FieldIndex::from("id")).unwrap_err().kind(),
                        &ApllodbErrorKind::InvalidName
                    );
                    Ok(())
                })),
            ),
        )
        .run()
        .await;
}

#[async_std::test]
async fn test_selection() {
    SqlTest::default()
        .add_steps(Steps::SetupPeopleDataset)
        .add_step(Step::new("BEGIN", StepRes::Ok))
        .add_step(
            // selection to PK
            Step::new(
                "SELECT id, age FROM people WHERE id = 2",
                StepRes::OkQuery(Box::new(|mut records| {
                    let id_index = FieldIndex::from("id");

                    let r = records.next().unwrap();
                    assert_eq!(r.get::<i64>(&id_index).unwrap(), Some(2));
                    assert!(records.next().is_none());

                    Ok(())
                })),
            ),
        )
        .add_step(
            // selection to non-PK
            Step::new(
                "SELECT id, age FROM people WHERE age = 35",
                StepRes::OkQuery(Box::new(|mut records| {
                    let age_index = FieldIndex::from("age");

                    let r = records.next().unwrap();
                    assert_eq!(r.get::<i32>(&age_index).unwrap(), Some(35));
                    assert!(records.next().is_none());

                    Ok(())
                })),
            ),
        )
        .add_step(
            // NULL is evaluated as FALSE
            Step::new(
                "SELECT id, age FROM people WHERE NULL",
                StepRes::OkQuery(Box::new(|records| {
                    assert_eq!(records.count(), 0);
                    Ok(())
                })),
            ),
        )
        .add_step(
            // DatatypeMismatch
            Step::new(
                "SELECT id, age FROM people WHERE 1",
                StepRes::Err(ApllodbErrorKind::DatatypeMismatch),
            ),
        )
        .run()
        .await;
}
