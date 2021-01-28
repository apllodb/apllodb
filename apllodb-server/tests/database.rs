mod sql_test;

use apllodb_server::{test_support::test_setup, ApllodbServer};
use apllodb_shared_components::{
    ApllodbError, ApllodbErrorKind, ApllodbResult, Session, SessionWithoutDb,
};
use sql_test::{SqlTest, Step, StepRes};

#[ctor::ctor]
fn setup() {
    test_setup();
}

#[async_std::test]
async fn test_create_database() -> ApllodbResult<()> {
    let mut t = SqlTest::default();
    t.add_step(Step::new(
        "CREATE DATABASE test_create_database",
        StepRes::Ok,
    ));
    t.add_step(Step::new(
        "CREATE DATABASE test_create_database",
        StepRes::Err(ApllodbErrorKind::DuplicateDatabase),
    ));
    t.run().await;

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
