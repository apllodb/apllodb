mod sql_test;

use apllodb_server::{test_support::test_setup, Record, RecordIndex, SchemaIndex, SqlState};
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
        .add_steps(Steps::SetupPeopleBodyPetDataset)
        .add_step(Step::new("BEGIN", StepRes::Ok))
        .add_step(Step::new(
            "SELECT id, age FROM people",
            StepRes::OkQuery(Box::new(|records| {
                let mut records = records.sorted_by_key(|r| {
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("id")))
                        .unwrap()
                        .unwrap()
                });
                assert_eq!(records.next(), Some(Record::fx_people1()));
                assert_eq!(records.next(), Some(Record::fx_people2()));
                assert_eq!(records.next(), Some(Record::fx_people3()));
                assert!(records.next().is_none());
                Ok(())
            })),
        ))
        .add_step(
            // reproduces: https://github.com/darwin-education/apllodb/issues/132
            Step::new(
                "SELECT id, people_id, kind, age FROM pet",
                StepRes::OkQuery(Box::new(|records| {
                    let mut records = records.sorted_by_key(|r| {
                        r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("id")))
                            .unwrap()
                            .unwrap()
                    });
                    assert_eq!(records.next(), Some(Record::fx_pet1()));
                    assert_eq!(records.next(), Some(Record::fx_pet3_1()));
                    assert_eq!(records.next(), Some(Record::fx_pet3_2()));
                    assert!(records.next().is_none());
                    Ok(())
                })),
            ),
        )
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
                    let id_index = RecordIndex::Name(SchemaIndex::from("id"));

                    let mut records =
                        records.sorted_by_key(|r| r.get::<i64>(&id_index).unwrap().unwrap());

                    let r = records.next().unwrap();
                    assert_eq!(
                        r.get::<i64>(&id_index).unwrap(),
                        Record::fx_people1().get::<i64>(&id_index).unwrap()
                    );
                    assert_eq!(
                        r.get::<i32>(&RecordIndex::Name(SchemaIndex::from("age")))
                            .unwrap_err()
                            .kind(),
                        &SqlState::NameErrorNotFound
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
                    let age_index = RecordIndex::Name(SchemaIndex::from("age"));
                    let mut records =
                        records.sorted_by_key(|r| r.get::<i32>(&age_index).unwrap().unwrap());
                    let r = records.next().unwrap();
                    assert_eq!(
                        r.get::<i32>(&age_index).unwrap(),
                        Record::fx_people1().get::<i32>(&age_index).unwrap()
                    );
                    assert_eq!(
                        r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("id")))
                            .unwrap_err()
                            .kind(),
                        &SqlState::NameErrorNotFound
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
                    let id_index = RecordIndex::Name(SchemaIndex::from("id"));

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
                    let age_index = RecordIndex::Name(SchemaIndex::from("age"));

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
            // DataExceptionIllegalOperation
            Step::new(
                "SELECT id, age FROM people WHERE 1",
                StepRes::Err(SqlState::DataExceptionIllegalOperation),
            ),
        )
        .run()
        .await;
}

#[async_std::test]
async fn test_sort() {
    SqlTest::default()
        .add_steps(Steps::SetupPeopleBodyPetDataset)
        .add_step(Step::new("BEGIN", StepRes::Ok))
        .add_step(Step::new(
            // PK, ASC (default)
            "SELECT id, age FROM people ORDER BY id",
            StepRes::OkQuery(Box::new(|mut records| {
                assert_eq!(records.next(), Some(Record::fx_people1()));
                assert_eq!(records.next(), Some(Record::fx_people2()));
                assert_eq!(records.next(), Some(Record::fx_people3()));
                assert!(records.next().is_none());
                Ok(())
            })),
        ))
        .add_step(Step::new(
            // PK, ASC
            "SELECT id, age FROM people ORDER BY id ASC",
            StepRes::OkQuery(Box::new(|mut records| {
                assert_eq!(records.next(), Some(Record::fx_people1()));
                assert_eq!(records.next(), Some(Record::fx_people2()));
                assert_eq!(records.next(), Some(Record::fx_people3()));
                assert!(records.next().is_none());
                Ok(())
            })),
        ))
        .add_step(Step::new(
            // PK, DESC
            "SELECT id, age FROM people ORDER BY id DESC",
            StepRes::OkQuery(Box::new(|mut records| {
                assert_eq!(records.next(), Some(Record::fx_people3()));
                assert_eq!(records.next(), Some(Record::fx_people2()));
                assert_eq!(records.next(), Some(Record::fx_people1()));
                assert!(records.next().is_none());
                Ok(())
            })),
        ))
        .add_step(Step::new(
            // non-PK, ASC (default)
            "SELECT id, age FROM people ORDER BY age",
            StepRes::OkQuery(Box::new(|mut records| {
                assert_eq!(records.next(), Some(Record::fx_people1()));
                assert_eq!(records.next(), Some(Record::fx_people3()));
                assert_eq!(records.next(), Some(Record::fx_people2()));
                assert!(records.next().is_none());
                Ok(())
            })),
        ))
        .add_step(Step::new(
            // non-PK, ASC
            "SELECT id, age FROM people ORDER BY age ASC",
            StepRes::OkQuery(Box::new(|mut records| {
                assert_eq!(records.next(), Some(Record::fx_people1()));
                assert_eq!(records.next(), Some(Record::fx_people3()));
                assert_eq!(records.next(), Some(Record::fx_people2()));
                assert!(records.next().is_none());
                Ok(())
            })),
        ))
        .add_step(Step::new(
            // non-PK, ASC
            "SELECT id, age FROM people ORDER BY age DESC",
            StepRes::OkQuery(Box::new(|mut records| {
                assert_eq!(records.next(), Some(Record::fx_people2()));
                assert_eq!(records.next(), Some(Record::fx_people3()));
                assert_eq!(records.next(), Some(Record::fx_people1()));
                assert!(records.next().is_none());
                Ok(())
            })),
        ))
        .add_step(Step::new(
            // non-PK, ASC ; PK, DESC
            "SELECT id, people_id, kind, age FROM pet ORDER BY kind ASC, id DESC",
            StepRes::OkQuery(Box::new(|mut records| {
                assert_eq!(records.next(), Some(Record::fx_pet3_2()));
                assert_eq!(records.next(), Some(Record::fx_pet3_1()));
                assert_eq!(records.next(), Some(Record::fx_pet1()));
                assert!(records.next().is_none());
                Ok(())
            })),
        ))
        .add_step(Step::new(
            // non-PK, DESC ; PK, DESC
            "SELECT id, people_id, kind, age FROM pet ORDER BY kind DESC, id DESC",
            StepRes::OkQuery(Box::new(|mut records| {
                assert_eq!(records.next(), Some(Record::fx_pet3_1()));
                assert_eq!(records.next(), Some(Record::fx_pet1()));
                assert_eq!(records.next(), Some(Record::fx_pet3_2()));
                assert!(records.next().is_none());
                Ok(())
            })),
        ))
        .run()
        .await;
}

#[async_std::test]
async fn test_inner_join() {
    SqlTest::default()
        .add_steps(Steps::SetupPeopleBodyPetDataset)
        .add_step(Step::new("BEGIN", StepRes::Ok))
        .add_step(Step::new(
            "SELECT people.id, people.age, body.height FROM people INNER JOIN body ON people.id = body.people_id",
            StepRes::OkQuery(Box::new(| records| {
                let mut records =
                records.sorted_by_key(|r| r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("people.id"))).unwrap().unwrap());

                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("people.id"))).unwrap(), 
                    Record::fx_people1().get::<i64>(&RecordIndex::Name(SchemaIndex::from("id"))).unwrap()
                );
                assert_eq!(
                    r.get::<i32>(&RecordIndex::Name(SchemaIndex::from("age"))).unwrap(), 
                    Record::fx_people1().get::<i32>(&RecordIndex::Name(SchemaIndex::from("age"))).unwrap()
                );
                assert_eq!(
                    r.get::<i32>(&RecordIndex::Name(SchemaIndex::from("height"))).unwrap(), 
                    Record::fx_body1().get::<i32>(&RecordIndex::Name(SchemaIndex::from("height"))).unwrap()
                );

                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("people.id"))).unwrap(), 
                    Record::fx_people3().get::<i64>(&RecordIndex::Name(SchemaIndex::from("id"))).unwrap()
                );
                assert_eq!(
                    r.get::<i32>(&RecordIndex::Name(SchemaIndex::from("age"))).unwrap(), 
                    Record::fx_people3().get::<i32>(&RecordIndex::Name(SchemaIndex::from("age"))).unwrap()
                );
                assert_eq!(
                    r.get::<i32>(&RecordIndex::Name(SchemaIndex::from("height"))).unwrap(), 
                    Record::fx_body3().get::<i32>(&RecordIndex::Name(SchemaIndex::from("height"))).unwrap()
                );

                assert!(records.next().is_none());
                Ok(())
            })),
        ))
        .add_step(Step::new(
            // body table appears in join, but does not in projection.
            "SELECT people.id FROM people INNER JOIN body ON people.id = body.people_id",
            StepRes::OkQuery(Box::new(| records| {
                let mut records =
                records.sorted_by_key(|r| r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("people.id"))).unwrap().unwrap());

                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("people.id"))).unwrap(), 
                    Record::fx_people1().get::<i64>(&RecordIndex::Name(SchemaIndex::from("id"))).unwrap()
                );

                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("people.id"))).unwrap(), 
                    Record::fx_people3().get::<i64>(&RecordIndex::Name(SchemaIndex::from("id"))).unwrap()
                );

                assert!(records.next().is_none());
                Ok(())
            })),
        ))
        // TODO 3-table join
        .run()
        .await;
}
