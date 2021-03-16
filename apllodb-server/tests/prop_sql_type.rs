mod sql_test;

use apllodb_immutable_schema_engine_infra::test_support::clean_test_sqlite3;
use async_std::task::block_on;
use proptest::prelude::*;

use apllodb_server::test_support::test_setup;
use sql_test::{SqlTest, Step, StepRes, Steps};

#[ctor::ctor]
fn setup() {
    test_setup();
}

proptest! {
    #[test]
    fn proptest_insert_small_int(v: i16) {
        clean_test_sqlite3().unwrap();

        block_on(async {
            SqlTest::default()
            .add_steps(Steps::BeginTransaction)
            .add_step(Step::new(
                "CREATE TABLE t (c SMALLINT, PRIMARY KEY (c))",
                StepRes::Ok,
            ))
            .add_step(Step::new(
                format!("INSERT INTO t (c) VALUES ({})", v),
                StepRes::Ok,
            ))
            .run()
            .await;
        })
    }
}
