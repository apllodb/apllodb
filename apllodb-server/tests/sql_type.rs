mod sql_test;

use apllodb_server::test_support::test_setup;
use apllodb_shared_components::{ApllodbErrorKind, ColumnReference, FieldIndex};
use sql_test::{SqlTest, Step, StepRes, Steps};

#[ctor::ctor]
fn setup() {
    test_setup();
}

#[async_std::test]
async fn test_small_int() {
    SqlTest::default()
        .add_steps(Steps::BeginTransaction)
        .add_step(Step::new(
            "CREATE TABLE t (c SMALLINT, PRIMARY KEY (c))",
            StepRes::Ok,
        ))
        .add_step(Step::new(
            format!("INSERT INTO t (c) VALUES ({})", i16::MIN),
            StepRes::Ok,
        ))
        .add_step(Step::new(
            "SELECT c FROM t",
            StepRes::OkQuery(Box::new(|mut records| {
                let field = FieldIndex::factory_colref(ColumnReference::factory("t", "c"));

                let r = records.next().unwrap();
                assert_eq!(r.get::<i16>(&field).unwrap().unwrap(), i16::MIN);
                assert_eq!(r.get::<i32>(&field).unwrap().unwrap(), i16::MIN as i32);
                assert_eq!(r.get::<i64>(&field).unwrap().unwrap(), i16::MIN as i64);
                Ok(())
            })),
        ))
        .run()
        .await;
}

#[async_std::test]
async fn test_integer() {
    SqlTest::default()
        .add_steps(Steps::BeginTransaction)
        .add_step(Step::new(
            "CREATE TABLE t (c INTEGER, PRIMARY KEY (c))",
            StepRes::Ok,
        ))
        .add_step(Step::new(
            format!("INSERT INTO t (c) VALUES ({})", i32::MIN),
            StepRes::Ok,
        ))
        .add_step(Step::new(
            "SELECT c FROM t",
            StepRes::OkQuery(Box::new(|mut records| {
                let field = FieldIndex::factory_colref(ColumnReference::factory("t", "c"));

                let r = records.next().unwrap();
                assert_eq!(
                    r.get::<i16>(&field).unwrap_err().kind(),
                    &ApllodbErrorKind::DatatypeMismatch
                );
                assert_eq!(r.get::<i32>(&field).unwrap().unwrap(), i32::MIN);
                assert_eq!(r.get::<i64>(&field).unwrap().unwrap(), i32::MIN as i64);
                Ok(())
            })),
        ))
        .run()
        .await;
}
