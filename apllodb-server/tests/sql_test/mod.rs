mod sql_test;
mod sql_test_session_ab;
mod step;

use apllodb_server::{ApllodbCommandSuccess, ApllodbServer};
use apllodb_shared_components::{
    ApllodbErrorKind, DatabaseName, Session, SessionWithDb, SessionWithoutDb,
};
pub use sql_test::SqlTest;
pub use sql_test_session_ab::{SessionAB, SqlTestSessionAB};
pub use step::{step_res::StepRes, steps::Steps, Step};

async fn session_with_db(server: &ApllodbServer, database_name: DatabaseName) -> SessionWithDb {
    let session = server
        .command(
            Session::from(SessionWithoutDb::default()),
            format!("CREATE DATABASE {}", database_name.as_str()),
        )
        .await
        .map_or_else(
            |e| {
                assert_eq!(e.err.kind(), &ApllodbErrorKind::DuplicateDatabase);
                e.session
            },
            |success| match success {
                ApllodbCommandSuccess::CreateDatabaseResponse { session } => session,
                _ => panic!(),
            },
        );

    match server
        .command(session, format!("USE DATABASE {}", database_name.as_str()))
        .await
        .unwrap()
    {
        ApllodbCommandSuccess::UseDatabaseResponse { session } => session,
        _ => panic!(),
    }
}
