mod sql_test;

use apllodb_server::test_support::{fixture::PEOPLE_REC1, test_setup};
use apllodb_shared_components::FieldIndex;
use sql_test::{SqlTest, Step, StepRes, Steps};

#[ctor::ctor]
fn setup() {
    test_setup();
}

#[ignore]
#[async_std::test]
async fn test_update() {
    SqlTest::default()
        .add_steps(Steps::SetupPeopleDataset)
        .add_step(Step::new("BEGIN", StepRes::Ok))
        .add_step(Step::new(
            "SELECT id, age FROM people WHERE id = 1",
            StepRes::OkQuery(Box::new(|mut rec_iter| {
                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i32>(&FieldIndex::from("age")).unwrap(),
                    PEOPLE_REC1.get::<i32>(&FieldIndex::from("age")).unwrap()
                );
                Ok(())
            })),
        ))
        .add_step(Step::new(
            "UPDATE people SET age = 100 WHERE id = 2",
            StepRes::Ok,
        ))
        .add_step(Step::new(
            "SELECT id, age FROM people WHERE id = 1",
            StepRes::OkQuery(Box::new(|mut rec_iter| {
                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i32>(&FieldIndex::from("age")).unwrap(),
                    PEOPLE_REC1.get::<i32>(&FieldIndex::from("age")).unwrap()
                );
                Ok(())
            })),
        ))
        .add_step(Step::new(
            "UPDATE people SET age = 100 WHERE id = 1",
            StepRes::Ok,
        ))
        .add_step(Step::new(
            "SELECT id, age FROM people WHERE id = 1",
            StepRes::OkQuery(Box::new(|mut rec_iter| {
                let r = rec_iter.next().unwrap();
                assert_eq!(r.get::<i32>(&FieldIndex::from("age")).unwrap(), Some(100));
                Ok(())
            })),
        ))
        .add_step(Step::new("UPDATE people SET age = 200", StepRes::Ok))
        .add_step(Step::new(
            "SELECT id, age FROM people WHERE id = 1",
            StepRes::OkQuery(Box::new(|mut rec_iter| {
                let r = rec_iter.next().unwrap();
                assert_eq!(r.get::<i32>(&FieldIndex::from("age")).unwrap(), Some(200));
                Ok(())
            })),
        ))
        // updates PK
        .add_step(Step::new(
            "UPDATE people SET id = 0 WHERE age = 200",
            StepRes::Ok,
        ))
        .add_step(Step::new(
            "SELECT id, age FROM people WHERE age = 200",
            StepRes::OkQuery(Box::new(|mut rec_iter| {
                let r = rec_iter.next().unwrap();
                assert_eq!(r.get::<i64>(&FieldIndex::from("id")).unwrap(), Some(0));
                Ok(())
            })),
        ))
        .run()
        .await;
}