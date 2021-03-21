mod sql_test;

use apllodb_server::test_support::test_setup;
use sql_test::{SqlTest, Step, StepRes, Steps};

#[ctor::ctor]
fn setup() {
    test_setup();
}

#[async_std::test]
async fn test_add_column() {
    SqlTest::default()
        .add_steps(Steps::CreateTablePeople)
        .add_step(Step::new("BEGIN", StepRes::Ok))
        .add_step(Step::new(
            "ALTER TABLE people ADD COLUMN c1 INTEGER",
            StepRes::Ok,
        ))
        .add_step(Step::new(
            "ALTER TABLE people ADD COLUMN c2 BIGINT NOT NULL",
            StepRes::Ok,
        ))
        .add_step(Step::new("ALTER TABLE people ADD c3 BIGINT", StepRes::Ok))
        .run()
        .await;
}
