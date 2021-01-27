use apllodb_server::{test_support::test_setup, ApllodbServer, ApllodbSuccess};
use apllodb_shared_components::{ApllodbErrorKind, ApllodbResult, Session};

#[ctor::ctor]
fn setup() {
    test_setup();
}

#[async_std::test]
async fn test_begin() -> ApllodbResult<()> {
    let server = ApllodbServer::default();
    let session = server.session_with_db().await?;

    let sql = "BEGIN";

    if let ApllodbSuccess::BeginTransactionResponse { session } = server
        .command(Session::WithDb(session), sql.to_string())
        .await?
    {
        let err = server
            .command(Session::WithTx(session), sql.to_string())
            .await
            .unwrap_err();
        assert_eq!(err.kind(), &ApllodbErrorKind::InvalidTransactionState);
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
            .command(Session::WithTx(session), sql.to_string())
            .await?,
        ApllodbSuccess::TransactionEndResponse {..}
    );

    Ok(())
}
