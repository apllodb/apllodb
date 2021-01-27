use apllodb_server::{test_support::test_setup, ApllodbServer, ApllodbSuccess};
use apllodb_shared_components::{ApllodbErrorKind, ApllodbResult, Session, SessionWithoutDb};

#[ctor::ctor]
fn setup() {
    test_setup();
}

#[async_std::test]
async fn test_create_database() -> ApllodbResult<()> {
    let server = ApllodbServer::default();
    let session = Session::WithoutDb(SessionWithoutDb::default());

    let sql = "CREATE DATABASE test_create_database";

    if let ApllodbSuccess::DatabaseResponse { session } =
        server.command(session, sql.to_string()).await?
    {
        let err = server.command(session, sql.to_string()).await.unwrap_err();
        assert_eq!(err.kind(), &ApllodbErrorKind::DuplicateDatabase);
    } else {
        panic!("must be DatabaseResponse")
    }

    Ok(())
}
