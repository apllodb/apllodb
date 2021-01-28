use apllodb_server::{test_support::test_setup, ApllodbCommandSuccess, ApllodbServer};
use apllodb_shared_components::{
    ApllodbError, ApllodbErrorKind, ApllodbResult, Session, SessionWithoutDb,
};

#[ctor::ctor]
fn setup() {
    test_setup();
}

#[async_std::test]
async fn test_create_database() -> ApllodbResult<()> {
    let t = SqlTest::default();

    let server = ApllodbServer::default();

    let sql = "CREATE DATABASE test_create_database";

    if let ApllodbCommandSuccess::CreateDatabaseResponse { session } = server
        .command(Session::from(SessionWithoutDb::default()), sql.to_string())
        .await?
    {
        let err = server.command(session, sql.to_string()).await.unwrap_err();
        assert_eq!(
            ApllodbError::from(err).kind(),
            &ApllodbErrorKind::DuplicateDatabase
        );
    } else {
        panic!("must be DatabaseResponse")
    }

    Ok(())
}

#[async_std::test]
async fn test_use_database() -> ApllodbResult<()> {
    let server = ApllodbServer::default();

    let sql = "USE DATABASE test_use_database";

    // cannot USE before CREATE
    let err = server
        .command(Session::from(SessionWithoutDb::default()), sql.to_string())
        .await
        .unwrap_err();
    assert_eq!(
        ApllodbError::from(err).kind(),
        &ApllodbErrorKind::UndefinedObject
    );
    //

    let _ = server
        .command(
            Session::from(SessionWithoutDb::default()),
            "CREATE DATABASE test_use_database".to_string(),
        )
        .await?;

    let _ = server
        .command(Session::from(SessionWithoutDb::default()), sql.to_string())
        .await?;

    Ok(())
}
