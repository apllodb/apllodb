use apllodb_server::{test_support::test_setup, ApllodbCommandSuccess, ApllodbServer};
use apllodb_shared_components::{ApllodbError, ApllodbErrorKind, ApllodbResult, Session};

#[ctor::ctor]
fn setup() {
    test_setup();
}

#[async_std::test]
async fn test_begin() -> ApllodbResult<()> {
    let server = ApllodbServer::default();
    let session = server.session_with_db().await?;

    let sql = "BEGIN";

    if let ApllodbCommandSuccess::BeginTransactionResponse { session } = server
        .command(Session::from(session), sql.to_string())
        .await?
    {
        let err = server
            .command(Session::from(session), sql.to_string())
            .await
            .unwrap_err();
        assert_eq!(
            ApllodbError::from(err).kind(),
            &ApllodbErrorKind::InvalidTransactionState
        );
    } else {
        panic!("must be BeginTransactionResponse")
    }

    Ok(())
}

#[async_std::test]
async fn test_commit() -> ApllodbResult<()> {
    let server = ApllodbServer::default();
    let session = server.session_with_tx().await?;

    let sql = "COMMIT";

    matches!(
        server
            .command(Session::from(session), sql.to_string())
            .await?,
        ApllodbCommandSuccess::TransactionEndResponse {..}
    );

    Ok(())
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
