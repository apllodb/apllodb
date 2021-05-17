mod sql_test;

use apllodb_server::{test_support::test_setup, SqlState};
use sql_test::{SqlTest, Step, StepRes, Steps};

#[ctor::ctor]
fn setup() {
    test_setup();
}

#[async_std::test]
async fn test_alter_table_add_column() {
    SqlTest::default()
        .add_steps(Steps::CreateTablePeople)
        .add_step(Step::new("BEGIN", StepRes::Ok))
        // v1
        .add_step(Step::new(
            "INSERT INTO people (id, age) VALUES (1, 10)",
            StepRes::Ok,
        ))
        .add_step(Step::new(
            "ALTER TABLE people ADD COLUMN c1 INTEGER NOT NULL",
            StepRes::Ok,
        ))
        // v2
        .add_step(Step::new(
            "INSERT INTO people (id, age, c1) VALUES (2, 20, 200)",
            StepRes::Ok,
        ))
        // v1
        .add_step(Step::new(
            "INSERT INTO people (id, age) VALUES (3, 30)",
            StepRes::Ok,
        ))
        .add_step(
            // https://github.com/eukarya-inc/apllodb/issues/191
            Step::new(
                "INSERT INTO people (id, age, c1) VALUES (4, 40, NULL)",
                StepRes::Err(SqlState::IntegrityConstraintNotNullViolation),
            ),
        )
        .run()
        .await;
}
