mod sql_test;

use apllodb_server::test_support::test_setup;
use apllodb_shared_components::{ApllodbErrorKind, FieldIndex};
use apllodb_storage_engine_interface::test_support::fixture::*;
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
                let mut records = records.sorted_by_key(|r| {
                    r.get::<i64>(&FieldIndex::factory_colref("people", "id"))
                        .unwrap()
                        .unwrap()
                });
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
        .add_step(Step::new(
            // projection to PK
            "SELECT id FROM people",
            StepRes::OkQuery(Box::new(|records| {
                let mut records = records.sorted_by_key(|r| {
                    r.get::<i64>(&FieldIndex::factory_colref("people", "id"))
                        .unwrap()
                        .unwrap()
                });

                let id_field = FieldIndex::factory_colref("people", "id");

                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&id_field).unwrap(),
                    T_PEOPLE_R1.get::<i64>(&id_field).unwrap()
                );
                assert_eq!(
                    r.get::<i32>(&FieldIndex::factory_colref("people", "age"))
                        .unwrap_err()
                        .kind(),
                    &ApllodbErrorKind::InvalidColumnReference
                );

                Ok(())
            })),
        ))
        .run()
        .await;
}
