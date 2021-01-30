mod sql_test;

use apllodb_server::test_support::test_setup;
use apllodb_shared_components::ApllodbErrorKind;
use sql_test::{SqlTest, Step, StepRes, Steps};

#[ctor::ctor]
fn setup() {
    test_setup();
}

#[async_std::test]
async fn test_error_on_dup_pk() {
    SqlTest::default()
        .add_steps(Steps::CreateTablePeople)
        .add_step(Step::new(
            "INSERT INTO people (id, age) VALUES (1, 10)",
            StepRes::Ok,
        ))
        .add_step(Step::new(
            "INSERT INTO people (id, age) VALUES (1, 20)",
            StepRes::Err(ApllodbErrorKind::UniqueViolation),
        ))
        .run()
        .await;
}
