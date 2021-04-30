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
            CREATE TABLE 会社 (
              ID BIGINT NOT NULL,
              名前 TEXT NOT NULL,
              本社の地域 TEXT NOT NULL,
              従業員数 INTEGER NOT NULL,

              PRIMARY KEY (ID)
            );"#,
            StepRes::Ok,
        ))
        .add_step(Step::new(
            r#"
            CREATE TABLE 会社合併史 (
              ID BIGINT NOT NULL,
              存続会社ID BIGINT NOT NULL,
              消滅会社ID BIGINT NOT NULL,

              PRIMARY KEY (ID)
            );"#,
            StepRes::Ok,
        ))
        .add_step(Step::new("COMMIT", StepRes::Ok))
        // -- prepare initial data
        .add_step(Step::new("BEGIN", StepRes::Ok))
        .add_step(Step::new(
            r#"
            INSERT INTO 会社 (ID, 名前, 本社の地域, 従業員数)
            VALUES
              (101, "ソニー", "東京", 3000),
              (102, "コナミ", "東京", 1000),
              (103, "ハドソン", "東京", 500),
              (104, "エリクソン", "スウェーデン", 1200);
            "#,
            StepRes::Ok,
        ))
        .add_step(Step::new(
            r#"
            INSERT INTO 会社合併史 (ID, 存続会社ID, 消滅会社ID)
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
            r#"SELECT ID, 名前, 本社の地域, 従業員数 FROM 会社 ORDER BY ID;"#,
            StepRes::OkQuery(Box::new(|mut rec_iter| {
                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("ID")))
                        .unwrap()
                        .unwrap(),
                    101
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("名前")))
                        .unwrap()
                        .unwrap(),
                    "ソニー"
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("本社の地域")))
                        .unwrap()
                        .unwrap(),
                    "東京"
                );
                assert_eq!(
                    r.get::<i32>(&RecordIndex::Name(SchemaIndex::from("従業員数")))
                        .unwrap()
                        .unwrap(),
                    3000
                );

                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("ID")))
                        .unwrap()
                        .unwrap(),
                    102
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("名前")))
                        .unwrap()
                        .unwrap(),
                    "コナミ"
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("本社の地域")))
                        .unwrap()
                        .unwrap(),
                    "東京"
                );
                assert_eq!(
                    r.get::<i32>(&RecordIndex::Name(SchemaIndex::from("従業員数")))
                        .unwrap()
                        .unwrap(),
                    1000
                );

                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("ID")))
                        .unwrap()
                        .unwrap(),
                    103
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("名前")))
                        .unwrap()
                        .unwrap(),
                    "ハドソン"
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("本社の地域")))
                        .unwrap()
                        .unwrap(),
                    "東京"
                );
                assert_eq!(
                    r.get::<i32>(&RecordIndex::Name(SchemaIndex::from("従業員数")))
                        .unwrap()
                        .unwrap(),
                    500
                );

                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("ID")))
                        .unwrap()
                        .unwrap(),
                    104
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("名前")))
                        .unwrap()
                        .unwrap(),
                    "エリクソン"
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("本社の地域")))
                        .unwrap()
                        .unwrap(),
                    "スウェーデン"
                );
                assert_eq!(
                    r.get::<i32>(&RecordIndex::Name(SchemaIndex::from("従業員数")))
                        .unwrap()
                        .unwrap(),
                    1200
                );

                assert!(rec_iter.next().is_none());
                Ok(())
            })),
        ))
        .add_step(Step::new(
            r#"SELECT ID, 名前 FROM 会社 WHERE 本社の地域 = "東京" ORDER BY 名前 ASC;"#,
            StepRes::OkQuery(Box::new(|mut rec_iter| {
                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("ID")))
                        .unwrap()
                        .unwrap(),
                    102
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("名前")))
                        .unwrap()
                        .unwrap(),
                    "コナミ"
                );

                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("ID")))
                        .unwrap()
                        .unwrap(),
                    101
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("名前")))
                        .unwrap()
                        .unwrap(),
                    "ソニー"
                );

                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("ID")))
                        .unwrap()
                        .unwrap(),
                    103
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("名前")))
                        .unwrap()
                        .unwrap(),
                    "ハドソン"
                );

                assert!(rec_iter.next().is_none());
                Ok(())
            })),
        ))
        .add_step(Step::new(
            r#"
            SELECT 会社合併史.存続会社ID, 会社.名前, 会社.本社の地域, 会社.従業員数
              FROM 会社 INNER JOIN 会社合併史 ON 会社.ID = 会社合併史.存続会社ID
              ORDER BY 会社合併史.ID;
            "#,
            StepRes::OkQuery(Box::new(|mut rec_iter| {
                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from(
                        "会社合併史.存続会社ID"
                    )))
                    .unwrap()
                    .unwrap(),
                    102
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("会社.名前")))
                        .unwrap()
                        .unwrap(),
                    "コナミ"
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("会社.本社の地域")))
                        .unwrap()
                        .unwrap(),
                    "東京"
                );
                assert_eq!(
                    r.get::<i32>(&RecordIndex::Name(SchemaIndex::from("会社.従業員数")))
                        .unwrap()
                        .unwrap(),
                    1000
                );

                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from(
                        "会社合併史.存続会社ID"
                    )))
                    .unwrap()
                    .unwrap(),
                    101
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("会社.名前")))
                        .unwrap()
                        .unwrap(),
                    "ソニー"
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("会社.本社の地域")))
                        .unwrap()
                        .unwrap(),
                    "東京"
                );
                assert_eq!(
                    r.get::<i32>(&RecordIndex::Name(SchemaIndex::from("会社.従業員数")))
                        .unwrap()
                        .unwrap(),
                    3000
                );

                assert!(rec_iter.next().is_none());
                Ok(())
            })),
        ))
        .add_step(Step::new("ABORT", StepRes::Ok))
        // -- ALTER TABLE ADD COLUMN to non-empty table, which is impossible for normal RDBMS.
        .add_step(Step::new("BEGIN", StepRes::Ok))
        .add_step(Step::new(
            r#"
            ALTER TABLE 会社
              ADD COLUMN
                時価総額 BIGINT NOT NULL;
            "#,
            StepRes::Ok,
        ))
        .add_step(Step::new(
            r#"SELECT ID, 名前, 時価総額 FROM 会社 ORDER BY ID;"#,
            StepRes::OkQuery(Box::new(|mut rec_iter| {
                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("ID")))
                        .unwrap()
                        .unwrap(),
                    101
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("名前")))
                        .unwrap()
                        .unwrap(),
                    "ソニー"
                );
                assert!(r
                    .get::<i64>(&RecordIndex::Name(SchemaIndex::from("時価総額")))
                    .unwrap()
                    .is_none());

                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("ID")))
                        .unwrap()
                        .unwrap(),
                    102
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("名前")))
                        .unwrap()
                        .unwrap(),
                    "コナミ"
                );
                assert!(r
                    .get::<i64>(&RecordIndex::Name(SchemaIndex::from("時価総額")))
                    .unwrap()
                    .is_none());

                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("ID")))
                        .unwrap()
                        .unwrap(),
                    103
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("名前")))
                        .unwrap()
                        .unwrap(),
                    "ハドソン"
                );
                assert!(r
                    .get::<i64>(&RecordIndex::Name(SchemaIndex::from("時価総額")))
                    .unwrap()
                    .is_none());

                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("ID")))
                        .unwrap()
                        .unwrap(),
                    104
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("名前")))
                        .unwrap()
                        .unwrap(),
                    "エリクソン"
                );
                assert!(r
                    .get::<i64>(&RecordIndex::Name(SchemaIndex::from("時価総額")))
                    .unwrap()
                    .is_none());

                assert!(rec_iter.next().is_none());
                Ok(())
            })),
        ))
        .add_step(Step::new("COMMIT", StepRes::Ok))
        // -- add a record to v2 & v1
        .add_step(Step::new("BEGIN", StepRes::Ok))
        .add_step(Step::new(
            // v2 is the max possible version to insert this record (having 時価総額 column).
            r#"
            INSERT INTO 会社 (ID, 名前, 本社の地域, 従業員数, 時価総額)
              VALUES
              (105, "スカラ", "東京", 300, 12900000000);
            "#,
            StepRes::Ok,
        ))
        .add_step(Step::new(
            // v1 is the max possible version to insert this record (not having NOT NULL 時価総額 column).
            r#"
            INSERT INTO 会社 (ID, 名前, 本社の地域, 従業員数)
              VALUES
              (106, "ソフトブレーン", "北海道", 500);
            "#,
            StepRes::Ok,
        ))
        .add_step(Step::new(
            r#"SELECT ID, 名前, 時価総額 FROM 会社 ORDER BY ID;"#,
            StepRes::OkQuery(Box::new(|mut rec_iter| {
                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("ID")))
                        .unwrap()
                        .unwrap(),
                    101
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("名前")))
                        .unwrap()
                        .unwrap(),
                    "ソニー"
                );
                assert!(r
                    .get::<i64>(&RecordIndex::Name(SchemaIndex::from("時価総額")))
                    .unwrap()
                    .is_none());

                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("ID")))
                        .unwrap()
                        .unwrap(),
                    102
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("名前")))
                        .unwrap()
                        .unwrap(),
                    "コナミ"
                );
                assert!(r
                    .get::<i64>(&RecordIndex::Name(SchemaIndex::from("時価総額")))
                    .unwrap()
                    .is_none());

                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("ID")))
                        .unwrap()
                        .unwrap(),
                    103
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("名前")))
                        .unwrap()
                        .unwrap(),
                    "ハドソン"
                );
                assert!(r
                    .get::<i64>(&RecordIndex::Name(SchemaIndex::from("時価総額")))
                    .unwrap()
                    .is_none());

                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("ID")))
                        .unwrap()
                        .unwrap(),
                    104
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("名前")))
                        .unwrap()
                        .unwrap(),
                    "エリクソン"
                );
                assert!(r
                    .get::<i64>(&RecordIndex::Name(SchemaIndex::from("時価総額")))
                    .unwrap()
                    .is_none());

                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("ID")))
                        .unwrap()
                        .unwrap(),
                    105
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("名前")))
                        .unwrap()
                        .unwrap(),
                    "スカラ"
                );
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("時価総額")))
                        .unwrap()
                        .unwrap(),
                    12900000000
                );

                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("ID")))
                        .unwrap()
                        .unwrap(),
                    106
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("名前")))
                        .unwrap()
                        .unwrap(),
                    "ソフトブレーン"
                );
                assert!(r
                    .get::<i64>(&RecordIndex::Name(SchemaIndex::from("時価総額")))
                    .unwrap()
                    .is_none());

                assert!(rec_iter.next().is_none());
                Ok(())
            })),
        ))
        .add_step(Step::new("COMMIT", StepRes::Ok))
        // -- filling values into NOT NULL column move the record from v1 to v2
        .add_step(Step::new("BEGIN", StepRes::Ok))
        .add_step(Step::new(
            r#"
            UPDATE 会社 SET 時価総額 = 50000000000 WHERE ID = 101;
            "#,
            StepRes::Ok,
        ))
        .add_step(Step::new(
            r#"
            UPDATE 会社 SET 時価総額 = 20000000000 WHERE ID = 102;
            "#,
            StepRes::Ok,
        ))
        .add_step(Step::new(
            r#"SELECT ID, 名前, 時価総額 FROM 会社 ORDER BY ID;"#,
            StepRes::OkQuery(Box::new(|mut rec_iter| {
                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("ID")))
                        .unwrap()
                        .unwrap(),
                    101
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("名前")))
                        .unwrap()
                        .unwrap(),
                    "ソニー"
                );
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("時価総額")))
                        .unwrap()
                        .unwrap(),
                    50000000000
                );

                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("ID")))
                        .unwrap()
                        .unwrap(),
                    102
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("名前")))
                        .unwrap()
                        .unwrap(),
                    "コナミ"
                );
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("時価総額")))
                        .unwrap()
                        .unwrap(),
                    20000000000
                );

                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("ID")))
                        .unwrap()
                        .unwrap(),
                    103
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("名前")))
                        .unwrap()
                        .unwrap(),
                    "ハドソン"
                );
                assert!(r
                    .get::<i64>(&RecordIndex::Name(SchemaIndex::from("時価総額")))
                    .unwrap()
                    .is_none());

                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("ID")))
                        .unwrap()
                        .unwrap(),
                    104
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("名前")))
                        .unwrap()
                        .unwrap(),
                    "エリクソン"
                );
                assert!(r
                    .get::<i64>(&RecordIndex::Name(SchemaIndex::from("時価総額")))
                    .unwrap()
                    .is_none());

                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("ID")))
                        .unwrap()
                        .unwrap(),
                    105
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("名前")))
                        .unwrap()
                        .unwrap(),
                    "スカラ"
                );
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("時価総額")))
                        .unwrap()
                        .unwrap(),
                    12900000000
                );

                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("ID")))
                        .unwrap()
                        .unwrap(),
                    106
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("名前")))
                        .unwrap()
                        .unwrap(),
                    "ソフトブレーン"
                );
                assert!(r
                    .get::<i64>(&RecordIndex::Name(SchemaIndex::from("時価総額")))
                    .unwrap()
                    .is_none());

                assert!(rec_iter.next().is_none());
                Ok(())
            })),
        ))
        .add_step(Step::new("COMMIT", StepRes::Ok))
        // -- v1's 時価総額 is returned as NULL, which is purely the same as SQL NULL so ordered lastly.
        .add_step(Step::new("BEGIN", StepRes::Ok))
        .add_step(Step::new(
            r#"SELECT ID, 名前, 時価総額 FROM 会社 ORDER BY 時価総額 DESC, ID ASC;"#,
            StepRes::OkQuery(Box::new(|mut rec_iter| {
                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("ID")))
                        .unwrap()
                        .unwrap(),
                    101
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("名前")))
                        .unwrap()
                        .unwrap(),
                    "ソニー"
                );
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("時価総額")))
                        .unwrap()
                        .unwrap(),
                    50000000000
                );

                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("ID")))
                        .unwrap()
                        .unwrap(),
                    102
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("名前")))
                        .unwrap()
                        .unwrap(),
                    "コナミ"
                );
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("時価総額")))
                        .unwrap()
                        .unwrap(),
                    20000000000
                );

                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("ID")))
                        .unwrap()
                        .unwrap(),
                    105
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("名前")))
                        .unwrap()
                        .unwrap(),
                    "スカラ"
                );
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("時価総額")))
                        .unwrap()
                        .unwrap(),
                    12900000000
                );

                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("ID")))
                        .unwrap()
                        .unwrap(),
                    103
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("名前")))
                        .unwrap()
                        .unwrap(),
                    "ハドソン"
                );
                assert!(r
                    .get::<i64>(&RecordIndex::Name(SchemaIndex::from("時価総額")))
                    .unwrap()
                    .is_none());

                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("ID")))
                        .unwrap()
                        .unwrap(),
                    104
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("名前")))
                        .unwrap()
                        .unwrap(),
                    "エリクソン"
                );
                assert!(r
                    .get::<i64>(&RecordIndex::Name(SchemaIndex::from("時価総額")))
                    .unwrap()
                    .is_none());

                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("ID")))
                        .unwrap()
                        .unwrap(),
                    106
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("名前")))
                        .unwrap()
                        .unwrap(),
                    "ソフトブレーン"
                );
                assert!(r
                    .get::<i64>(&RecordIndex::Name(SchemaIndex::from("時価総額")))
                    .unwrap()
                    .is_none());

                assert!(rec_iter.next().is_none());
                Ok(())
            })),
        ))
        .add_step(Step::new(
            r#"SELECT ID, 名前, 時価総額 FROM 会社 ORDER BY 時価総額 ASC, ID ASC;"#,
            StepRes::OkQuery(Box::new(|mut rec_iter| {
                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("ID")))
                        .unwrap()
                        .unwrap(),
                    105
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("名前")))
                        .unwrap()
                        .unwrap(),
                    "スカラ"
                );
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("時価総額")))
                        .unwrap()
                        .unwrap(),
                    12900000000
                );

                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("ID")))
                        .unwrap()
                        .unwrap(),
                    102
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("名前")))
                        .unwrap()
                        .unwrap(),
                    "コナミ"
                );
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("時価総額")))
                        .unwrap()
                        .unwrap(),
                    20000000000
                );

                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("ID")))
                        .unwrap()
                        .unwrap(),
                    101
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("名前")))
                        .unwrap()
                        .unwrap(),
                    "ソニー"
                );
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("時価総額")))
                        .unwrap()
                        .unwrap(),
                    50000000000
                );

                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("ID")))
                        .unwrap()
                        .unwrap(),
                    103
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("名前")))
                        .unwrap()
                        .unwrap(),
                    "ハドソン"
                );
                assert!(r
                    .get::<i64>(&RecordIndex::Name(SchemaIndex::from("時価総額")))
                    .unwrap()
                    .is_none());

                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("ID")))
                        .unwrap()
                        .unwrap(),
                    104
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("名前")))
                        .unwrap()
                        .unwrap(),
                    "エリクソン"
                );
                assert!(r
                    .get::<i64>(&RecordIndex::Name(SchemaIndex::from("時価総額")))
                    .unwrap()
                    .is_none());

                let r = rec_iter.next().unwrap();
                assert_eq!(
                    r.get::<i64>(&RecordIndex::Name(SchemaIndex::from("ID")))
                        .unwrap()
                        .unwrap(),
                    106
                );
                assert_eq!(
                    r.get::<String>(&RecordIndex::Name(SchemaIndex::from("名前")))
                        .unwrap()
                        .unwrap(),
                    "ソフトブレーン"
                );
                assert!(r
                    .get::<i64>(&RecordIndex::Name(SchemaIndex::from("時価総額")))
                    .unwrap()
                    .is_none());

                assert!(rec_iter.next().is_none());
                Ok(())
            })),
        ))
        .add_step(Step::new("ABORT", StepRes::Ok))
        .run()
        .await;
}
