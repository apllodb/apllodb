//! Scenario for v0.1.0-pre demonstration.

mod sql_test;

use apllodb_server::{test_support::test_setup, RecordIndex, SchemaIndex};
use sql_test::{SqlTest, Step, StepRes};

#[ctor::ctor]
fn setup() {
    test_setup();
}

#[async_std::test]
async fn test_scenario_010_pre_demo() {
    SqlTest::default()
        // -- prepare v1 schema
        .add_step(Step::new("BEGIN", StepRes::Ok))
        .add_step(Step::new(
            r#"
            CREATE TABLE company (
              id BIGINT NOT NULL,
              name TEXT NOT NULL,
              hq_area TEXT NOT NULL,
              num_employees INTEGER NOT NULL,

              PRIMARY KEY (id)
            );"#,
            StepRes::Ok,
        ))
        .add_step(Step::new(
            r#"
            CREATE TABLE merger_history (
              id BIGINT NOT NULL,
              merging_company_id BIGINT NOT NULL,
              merged_company_id BIGINT NOT NULL,

              PRIMARY KEY (id)
            );"#,
            StepRes::Ok,
        ))
        .add_step(Step::new("COMMIT", StepRes::Ok))
        // -- prepare initial data
        .add_step(Step::new("BEGIN", StepRes::Ok))
        .add_step(Step::new(
            r#"
            INSERT INTO company (id, name, hq_area, num_employees)
            VALUES
              (101, "Sony", "Tokyo", 3000),
              (102, "KONAMI", "Tokyo", 1000),
              (103, "Hudson", "Tokyo", 500),
              (104, "Ericsson", "Sweden", 1200);
            "#,
            StepRes::Ok,
        ))
        .add_step(Step::new(
            r#"
            INSERT INTO merger_history (id, merging_company_id, merged_company_id)
            VALUES
              (1, 102, 103),
              (2, 101, 104);
            "#,
            StepRes::Ok,
        ))
        .add_step(Step::new("COMMIT", StepRes::Ok))
        // -- confirm inserted data
        .add_step(Step::new("BEGIN", StepRes::Ok))
        .add_step(Step::new(
            r#"SELECT id, name, hq_area, num_employees FROM company ORDER BY id;"#,
            StepRes::OkQuery(Box::new(|mut records| {
                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("id")))
                        .unwrap()
                        .unwrap(),
                    101
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("name")))
                        .unwrap()
                        .unwrap(),
                    "Sony"
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("hq_area")))
                        .unwrap()
                        .unwrap(),
                    "Tokyo"
                );
                assert_eq!(
                    r.get::<i32>(&RecordIndex::Name(SchemaIndex::from("num_employees")))
                        .unwrap()
                        .unwrap(),
                    3000
                );

                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("id")))
                        .unwrap()
                        .unwrap(),
                    102
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("name")))
                        .unwrap()
                        .unwrap(),
                    "KONAMI"
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("hq_area")))
                        .unwrap()
                        .unwrap(),
                    "Tokyo"
                );
                assert_eq!(
                    r.get::<i32>(&RecordIndex::Name(SchemaIndex::from("num_employees")))
                        .unwrap()
                        .unwrap(),
                    1000
                );

                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("id")))
                        .unwrap()
                        .unwrap(),
                    103
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("name")))
                        .unwrap()
                        .unwrap(),
                    "Hudson"
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("hq_area")))
                        .unwrap()
                        .unwrap(),
                    "Tokyo"
                );
                assert_eq!(
                    r.get::<i32>(&RecordIndex::Name(SchemaIndex::from("num_employees")))
                        .unwrap()
                        .unwrap(),
                    500
                );

                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("id")))
                        .unwrap()
                        .unwrap(),
                    104
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("name")))
                        .unwrap()
                        .unwrap(),
                    "Ericsson"
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("hq_area")))
                        .unwrap()
                        .unwrap(),
                    "Sweden"
                );
                assert_eq!(
                    r.get::<i32>(&RecordIndex::Name(SchemaIndex::from("num_employees")))
                        .unwrap()
                        .unwrap(),
                    1200
                );

                assert!(records.next().is_none());
                Ok(())
            })),
        ))
        .add_step(Step::new(
            r#"SELECT id, name FROM company WHERE hq_area = "Tokyo" ORDER BY name ASC;"#,
            StepRes::OkQuery(Box::new(|mut records| {
                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("id")))
                        .unwrap()
                        .unwrap(),
                    103
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("name")))
                        .unwrap()
                        .unwrap(),
                    "Hudson"
                );

                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("id")))
                        .unwrap()
                        .unwrap(),
                    102
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("name")))
                        .unwrap()
                        .unwrap(),
                    "KONAMI"
                );

                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("id")))
                        .unwrap()
                        .unwrap(),
                    101
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("name")))
                        .unwrap()
                        .unwrap(),
                    "Sony"
                );


                assert!(records.next().is_none());
                Ok(())
            })),
        ))
        .add_step(Step::new(
            r#"
            SELECT merger_history.merging_company_id, company.name, company.hq_area, company.num_employees
              FROM company INNER JOIN merger_history ON company.id = merger_history.merging_company_id
              ORDER BY merger_history.id;
            "#,
            StepRes::OkQuery(Box::new(|mut records| {
                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from(
                        "merger_history.merging_company_id"
                    )))
                    .unwrap()
                    .unwrap(),
                    102
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("company.name")))
                        .unwrap()
                        .unwrap(),
                    "KONAMI"
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("company.hq_area")))
                        .unwrap()
                        .unwrap(),
                    "Tokyo"
                );
                assert_eq!(
                    r.get::<i32>(&RecordIndex::Name(SchemaIndex::from("company.num_employees")))
                        .unwrap()
                        .unwrap(),
                    1000
                );

                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from(
                        "merger_history.merging_company_id"
                    )))
                    .unwrap()
                    .unwrap(),
                    101
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("company.name")))
                        .unwrap()
                        .unwrap(),
                    "Sony"
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("company.hq_area")))
                        .unwrap()
                        .unwrap(),
                    "Tokyo"
                );
                assert_eq!(
                    r.get::<i32>(&RecordIndex::Name(SchemaIndex::from("company.num_employees")))
                        .unwrap()
                        .unwrap(),
                    3000
                );

                assert!(records.next().is_none());
                Ok(())
            })),
        ))
        .add_step(Step::new("ABORT", StepRes::Ok))
        // -- ALTER TABLE ADD COLUMN to non-empty table, which is impossible for normal RDBMS.
        .add_step(Step::new("BEGIN", StepRes::Ok))
        .add_step(Step::new(
            r#"
            ALTER TABLE company
              ADD COLUMN
                market_cap BIGINT NOT NULL;
            "#,
            StepRes::Ok,
        ))
        .add_step(Step::new(
            r#"SELECT id, name, market_cap FROM company ORDER BY id;"#,
            StepRes::OkQuery(Box::new(|mut records| {
                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("id")))
                        .unwrap()
                        .unwrap(),
                    101
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("name")))
                        .unwrap()
                        .unwrap(),
                    "Sony"
                );
                assert!(r
                    .get::<i64>(&RecordIndex::Name(SchemaIndex::from("market_cap")))
                    .unwrap()
                    .is_none());

                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("id")))
                        .unwrap()
                        .unwrap(),
                    102
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("name")))
                        .unwrap()
                        .unwrap(),
                    "KONAMI"
                );
                assert!(r
                    .get::<i64>(&RecordIndex::Name(SchemaIndex::from("market_cap")))
                    .unwrap()
                    .is_none());

                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("id")))
                        .unwrap()
                        .unwrap(),
                    103
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("name")))
                        .unwrap()
                        .unwrap(),
                    "Hudson"
                );
                assert!(r
                    .get::<i64>(&RecordIndex::Name(SchemaIndex::from("market_cap")))
                    .unwrap()
                    .is_none());

                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("id")))
                        .unwrap()
                        .unwrap(),
                    104
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("name")))
                        .unwrap()
                        .unwrap(),
                    "Ericsson"
                );
                assert!(r
                    .get::<i64>(&RecordIndex::Name(SchemaIndex::from("market_cap")))
                    .unwrap()
                    .is_none());

                assert!(records.next().is_none());
                Ok(())
            })),
        ))
        .add_step(Step::new("COMMIT", StepRes::Ok))
        // -- add a record to v2 & v1
        .add_step(Step::new("BEGIN", StepRes::Ok))
        .add_step(Step::new(
            // v2 is the max possible version to insert this record (having market_cap column).
            r#"
            INSERT INTO company (id, name, hq_area, num_employees, market_cap)
              VALUES
              (105, "Scala", "Tokyo", 300, 12900000000);
            "#,
            StepRes::Ok,
        ))
        .add_step(Step::new(
            // v1 is the max possible version to insert this record (not having NOT NULL market_cap column).
            r#"
            INSERT INTO company (id, name, hq_area, num_employees)
              VALUES
              (106, "SOFTBRAIN", "Hokkaido", 500);
            "#,
            StepRes::Ok,
        ))
        .add_step(Step::new(
            r#"SELECT id, name, market_cap FROM company ORDER BY id;"#,
            StepRes::OkQuery(Box::new(|mut records| {
                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("id")))
                        .unwrap()
                        .unwrap(),
                    101
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("name")))
                        .unwrap()
                        .unwrap(),
                    "Sony"
                );
                assert!(r
                    .get::<i64>(&RecordIndex::Name(SchemaIndex::from("market_cap")))
                    .unwrap()
                    .is_none());

                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("id")))
                        .unwrap()
                        .unwrap(),
                    102
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("name")))
                        .unwrap()
                        .unwrap(),
                    "KONAMI"
                );
                assert!(r
                    .get::<i64>(&RecordIndex::Name(SchemaIndex::from("market_cap")))
                    .unwrap()
                    .is_none());

                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("id")))
                        .unwrap()
                        .unwrap(),
                    103
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("name")))
                        .unwrap()
                        .unwrap(),
                    "Hudson"
                );
                assert!(r
                    .get::<i64>(&RecordIndex::Name(SchemaIndex::from("market_cap")))
                    .unwrap()
                    .is_none());

                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("id")))
                        .unwrap()
                        .unwrap(),
                    104
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("name")))
                        .unwrap()
                        .unwrap(),
                    "Ericsson"
                );
                assert!(r
                    .get::<i64>(&RecordIndex::Name(SchemaIndex::from("market_cap")))
                    .unwrap()
                    .is_none());

                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("id")))
                        .unwrap()
                        .unwrap(),
                    105
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("name")))
                        .unwrap()
                        .unwrap(),
                    "Scala"
                );
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("market_cap")))
                        .unwrap()
                        .unwrap(),
                    12900000000
                );

                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("id")))
                        .unwrap()
                        .unwrap(),
                    106
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("name")))
                        .unwrap()
                        .unwrap(),
                    "SOFTBRAIN"
                );
                assert!(r
                    .get::<i64>(&RecordIndex::Name(SchemaIndex::from("market_cap")))
                    .unwrap()
                    .is_none());

                assert!(records.next().is_none());
                Ok(())
            })),
        ))
        .add_step(Step::new("COMMIT", StepRes::Ok))
        // -- filling values into NOT NULL column move the record from v1 to v2
        .add_step(Step::new("BEGIN", StepRes::Ok))
        .add_step(Step::new(
            r#"
            UPDATE company SET market_cap = 50000000000 WHERE id = 101;
            "#,
            StepRes::Ok,
        ))
        .add_step(Step::new(
            r#"
            UPDATE company SET market_cap = 20000000000 WHERE id = 102;
            "#,
            StepRes::Ok,
        ))
        .add_step(Step::new(
            r#"SELECT id, name, market_cap FROM company ORDER BY id;"#,
            StepRes::OkQuery(Box::new(|mut records| {
                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("id")))
                        .unwrap()
                        .unwrap(),
                    101
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("name")))
                        .unwrap()
                        .unwrap(),
                    "Sony"
                );
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("market_cap")))
                        .unwrap()
                        .unwrap(),
                    50000000000
                );

                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("id")))
                        .unwrap()
                        .unwrap(),
                    102
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("name")))
                        .unwrap()
                        .unwrap(),
                    "KONAMI"
                );
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("market_cap")))
                        .unwrap()
                        .unwrap(),
                    20000000000
                );

                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("id")))
                        .unwrap()
                        .unwrap(),
                    103
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("name")))
                        .unwrap()
                        .unwrap(),
                    "Hudson"
                );
                assert!(r
                    .get::<i64>(&RecordIndex::Name(SchemaIndex::from("market_cap")))
                    .unwrap()
                    .is_none());

                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("id")))
                        .unwrap()
                        .unwrap(),
                    104
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("name")))
                        .unwrap()
                        .unwrap(),
                    "Ericsson"
                );
                assert!(r
                    .get::<i64>(&RecordIndex::Name(SchemaIndex::from("market_cap")))
                    .unwrap()
                    .is_none());

                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("id")))
                        .unwrap()
                        .unwrap(),
                    105
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("name")))
                        .unwrap()
                        .unwrap(),
                    "Scala"
                );
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("market_cap")))
                        .unwrap()
                        .unwrap(),
                    12900000000
                );

                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("id")))
                        .unwrap()
                        .unwrap(),
                    106
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("name")))
                        .unwrap()
                        .unwrap(),
                    "SOFTBRAIN"
                );
                assert!(r
                    .get::<i64>(&RecordIndex::Name(SchemaIndex::from("market_cap")))
                    .unwrap()
                    .is_none());

                assert!(records.next().is_none());
                Ok(())
            })),
        ))
        .add_step(Step::new("COMMIT", StepRes::Ok))
        // -- v1's market_cap is returned as NULL, which is purely the same as SQL NULL so ordered lastly.
        .add_step(Step::new("BEGIN", StepRes::Ok))
        .add_step(Step::new(
            r#"SELECT id, name, market_cap FROM company ORDER BY market_cap DESC, id ASC;"#,
            StepRes::OkQuery(Box::new(|mut records| {
                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("id")))
                        .unwrap()
                        .unwrap(),
                    101
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("name")))
                        .unwrap()
                        .unwrap(),
                    "Sony"
                );
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("market_cap")))
                        .unwrap()
                        .unwrap(),
                    50000000000
                );

                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("id")))
                        .unwrap()
                        .unwrap(),
                    102
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("name")))
                        .unwrap()
                        .unwrap(),
                    "KONAMI"
                );
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("market_cap")))
                        .unwrap()
                        .unwrap(),
                    20000000000
                );

                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("id")))
                        .unwrap()
                        .unwrap(),
                    105
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("name")))
                        .unwrap()
                        .unwrap(),
                    "Scala"
                );
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("market_cap")))
                        .unwrap()
                        .unwrap(),
                    12900000000
                );

                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("id")))
                        .unwrap()
                        .unwrap(),
                    103
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("name")))
                        .unwrap()
                        .unwrap(),
                    "Hudson"
                );
                assert!(r
                    .get::<i64>(&RecordIndex::Name(SchemaIndex::from("market_cap")))
                    .unwrap()
                    .is_none());

                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("id")))
                        .unwrap()
                        .unwrap(),
                    104
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("name")))
                        .unwrap()
                        .unwrap(),
                    "Ericsson"
                );
                assert!(r
                    .get::<i64>(&RecordIndex::Name(SchemaIndex::from("market_cap")))
                    .unwrap()
                    .is_none());

                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("id")))
                        .unwrap()
                        .unwrap(),
                    106
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("name")))
                        .unwrap()
                        .unwrap(),
                    "SOFTBRAIN"
                );
                assert!(r
                    .get::<i64>(&RecordIndex::Name(SchemaIndex::from("market_cap")))
                    .unwrap()
                    .is_none());

                assert!(records.next().is_none());
                Ok(())
            })),
        ))
        .add_step(Step::new(
            r#"SELECT id, name, market_cap FROM company ORDER BY market_cap ASC, id ASC;"#,
            StepRes::OkQuery(Box::new(|mut records| {
                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("id")))
                        .unwrap()
                        .unwrap(),
                    105
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("name")))
                        .unwrap()
                        .unwrap(),
                    "Scala"
                );
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("market_cap")))
                        .unwrap()
                        .unwrap(),
                    12900000000
                );

                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("id")))
                        .unwrap()
                        .unwrap(),
                    102
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("name")))
                        .unwrap()
                        .unwrap(),
                    "KONAMI"
                );
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("market_cap")))
                        .unwrap()
                        .unwrap(),
                    20000000000
                );

                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("id")))
                        .unwrap()
                        .unwrap(),
                    101
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("name")))
                        .unwrap()
                        .unwrap(),
                    "Sony"
                );
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("market_cap")))
                        .unwrap()
                        .unwrap(),
                    50000000000
                );

                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("id")))
                        .unwrap()
                        .unwrap(),
                    103
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("name")))
                        .unwrap()
                        .unwrap(),
                    "Hudson"
                );
                assert!(r
                    .get::<i64>(&RecordIndex::Name(SchemaIndex::from("market_cap")))
                    .unwrap()
                    .is_none());

                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("id")))
                        .unwrap()
                        .unwrap(),
                    104
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("name")))
                        .unwrap()
                        .unwrap(),
                    "Ericsson"
                );
                assert!(r
                    .get::<i64>(&RecordIndex::Name(SchemaIndex::from("market_cap")))
                    .unwrap()
                    .is_none());

                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("id")))
                        .unwrap()
                        .unwrap(),
                    106
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("name")))
                        .unwrap()
                        .unwrap(),
                    "SOFTBRAIN"
                );
                assert!(r
                    .get::<i64>(&RecordIndex::Name(SchemaIndex::from("market_cap")))
                    .unwrap()
                    .is_none());

                assert!(records.next().is_none());
                Ok(())
            })),
        ))
        .add_step(Step::new("ABORT", StepRes::Ok))
        .run()
        .await;
}
