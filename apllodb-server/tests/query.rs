mod sql_test;

use apllodb_server::test_support::test_setup;
use apllodb_shared_components::{ColumnName, ColumnReference, FieldIndex, TableName};
use apllodb_storage_engine_interface::test_support::fixture::*;
use itertools::Itertools;
use pretty_assertions::assert_eq;
use sql_test::{SqlTest, Step, StepRes, Steps};

#[ctor::ctor]
fn setup() {
    test_setup();
}

fn field(table_name: &str, column_name: &str) -> FieldIndex {
    FieldIndex::InColumnReference(ColumnReference::new(
        TableName::new(table_name).unwrap(),
        ColumnName::new(column_name).unwrap(),
    ))
}

#[async_std::test]
async fn test_fullscan() {
    SqlTest::default()
        .add_steps(Steps::SetupPeopleBodyPetDataset)
        .add_step(Step::new(
            "SELECT id, age FROM people",
            StepRes::OkQuery(Box::new(|records| {
                let mut records = records
                    .sorted_by_key(|r| r.get::<i32>(&field("people", "id")).unwrap().unwrap());
                assert_eq!(records.next(), Some(T_PEOPLE_R1.clone()));
                assert_eq!(records.next(), Some(T_PEOPLE_R2.clone()));
                assert_eq!(records.next(), Some(T_PEOPLE_R3.clone()));
                assert!(records.next().is_none());
                Ok(())
            })),
        ))
        .add_step(Step::new(
            "SELECT people_id, height FROM body",
            StepRes::OkQuery(Box::new(|records| {
                let mut records =
                    records.sorted_by_key(|r| r.get::<i32>(&field("body", "id")).unwrap().unwrap());
                assert_eq!(records.next(), Some(T_BODY_R1.clone()));
                assert_eq!(records.next(), Some(T_BODY_R3.clone()));
                assert!(records.next().is_none());
                Ok(())
            })),
        ))
        .add_step(Step::new(
            "SELECT people_id, kind, age FROM pet",
            StepRes::OkQuery(Box::new(|records| {
                let mut records =
                    records.sorted_by_key(|r| r.get::<i32>(&field("pet", "id")).unwrap().unwrap());
                assert_eq!(records.next(), Some(T_PET_R1.clone()));
                assert_eq!(records.next(), Some(T_PET_R3_1.clone()));
                assert_eq!(records.next(), Some(T_PET_R3_2.clone()));
                assert!(records.next().is_none());
                Ok(())
            })),
        ))
        .run()
        .await;
}
