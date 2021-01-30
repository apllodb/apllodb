mod sql_test;

use apllodb_server::test_support::test_setup;
use apllodb_shared_components::ApllodbErrorKind;
use sql_test::{SessionAB, SqlTest, SqlTestSessionAB, Step, StepRes};

#[ctor::ctor]
fn setup() {
    test_setup();
}

#[async_std::test]
async fn test_create_database() {
    SqlTest::default()
        .add_step(Step::new(
            "CREATE DATABASE test_create_database",
            StepRes::Ok,
        ))
        .add_step(Step::new(
            "CREATE DATABASE test_create_database",
            StepRes::Err(ApllodbErrorKind::DuplicateDatabase),
        ))
        .run()
        .await;
}

#[async_std::test]
async fn test_use_database() {
    SqlTest::default()
        .add_step(Step::new(
            "USE DATABASE test_use_database",
            StepRes::Err(ApllodbErrorKind::UndefinedObject),
        ))
        .add_step(Step::new("CREATE DATABASE test_use_database", StepRes::Ok))
        .add_step(Step::new("USE DATABASE test_use_database", StepRes::Ok))
        .run()
        .await;
}

#[async_std::test]
async fn test_create_database_session_ab() {
    SqlTestSessionAB::default()
        .add_step(
            SessionAB::A,
            Step::new(
                "CREATE DATABASE test_create_database_session_ab",
                StepRes::Ok,
            ),
        )
        .add_step(
            SessionAB::B,
            Step::new(
                "CREATE DATABASE test_create_database_session_ab",
                StepRes::Err(ApllodbErrorKind::DuplicateDatabase),
            ),
        )
        .run()
        .await;
}
