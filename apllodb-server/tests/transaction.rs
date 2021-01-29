mod sql_test;

use apllodb_server::{test_support::test_setup, ApllodbCommandSuccess, ApllodbServer};
use apllodb_shared_components::{ApllodbErrorKind, ApllodbResult, Session};
use sql_test::{SqlTest, Step, StepRes, Steps};

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
        .add_step(Step::new(
            "COMMIT",
            StepRes::Err(ApllodbErrorKind::InvalidTransactionState),
        ))
        .run()
        .await;
}

#[async_std::test]
async fn test_abort() -> ApllodbResult<()> {
    let server = ApllodbServer::default();
    let session = server.session_with_tx().await?;

    let sql = "ABORT";

    matches!(
        server
            .command(Session::from(session), sql.to_string())
            .await?,
        ApllodbCommandSuccess::TransactionEndResponse {..}
    );

    Ok(())
}
