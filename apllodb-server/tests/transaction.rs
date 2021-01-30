mod sql_test;

use apllodb_server::test_support::test_setup;
use apllodb_shared_components::{ApllodbErrorKind, ApllodbResult};
use sql_test::{SessionAB, SqlTest, SqlTestSessionAB, Step, StepRes, Steps};

#[ctor::ctor]
fn setup() {
    test_setup();
}

#[async_std::test]
async fn test_begin() -> ApllodbResult<()> {
    SqlTest::default()
        .add_steps(Steps::UseDatabase)
        .add_step(Step::new("BEGIN", StepRes::Ok))
        .add_step(Step::new(
            "BEGIN",
            StepRes::Err(ApllodbErrorKind::InvalidTransactionState),
        ))
        .run()
        .await;

    Ok(())
}

#[async_std::test]
async fn test_commit() {
    SqlTest::default()
        .add_steps(Steps::BeginTransaction)
        .add_step(Step::new("COMMIT", StepRes::Ok))
        .add_step(Step::new("BEGIN", StepRes::Ok))
        .add_step(Step::new("COMMIT", StepRes::Ok))
        .add_step(Step::new(
            "COMMIT",
            StepRes::Err(ApllodbErrorKind::InvalidTransactionState),
        ))
        .run()
        .await;
}

#[async_std::test]
async fn test_abort() {
    SqlTest::default()
        .add_steps(Steps::BeginTransaction)
        .add_step(Step::new("ABORT", StepRes::Ok))
        .add_step(Step::new("BEGIN", StepRes::Ok))
        .add_step(Step::new("ABORT", StepRes::Ok))
        .add_step(Step::new(
            "ABORT",
            StepRes::Err(ApllodbErrorKind::InvalidTransactionState),
        ))
        .run()
        .await;
}

#[async_std::test]
async fn test_begin_session_ab() -> ApllodbResult<()> {
    SqlTestSessionAB::default()
        .add_steps(SessionAB::A, Steps::UseDatabase)
        .add_steps(SessionAB::B, Steps::UseDatabase)
        .add_step(SessionAB::A, Step::new("BEGIN", StepRes::Ok))
        .add_step(SessionAB::B, Step::new("BEGIN", StepRes::Ok))
        .add_step(
            SessionAB::A,
            Step::new(
                "BEGIN",
                StepRes::Err(ApllodbErrorKind::InvalidTransactionState),
            ),
        )
        .run()
        .await;

    Ok(())
}
