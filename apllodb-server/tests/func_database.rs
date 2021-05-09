mod sql_test;

use apllodb_immutable_schema_engine_infra::test_support::sqlite_database_cleaner::SqliteDatabaseCleaner;
use apllodb_server::test_support::test_setup;
use apllodb_shared_components::{SqlState, ApllodbResult, DatabaseName};
use sql_test::{SessionAb, SqlTest, SqlTestSessionAb, Step, StepRes};

#[ctor::ctor]
fn setup() {
    test_setup();
}

#[async_std::test]
async fn test_create_database() {
    let db_name = DatabaseName::random();
    let _db_cleaner = SqliteDatabaseCleaner::new(db_name.clone());

    SqlTest::default()
        .add_step(Step::new(
            format!("CREATE DATABASE {}", db_name.as_str()),
            StepRes::Ok,
        ))
        .add_step(Step::new(
            format!("CREATE DATABASE {}", db_name.as_str()),
            StepRes::Err(SqlState::DuplicateDatabase),
        ))
        .run_with_manual_db_control()
        .await;
}

#[async_std::test]
async fn test_use_database() {
    let db_name = DatabaseName::random();
    let _db_cleaner = SqliteDatabaseCleaner::new(db_name.clone());

    SqlTest::default()
        .add_step(Step::new(
            format!("USE DATABASE {}", db_name.as_str()),
            StepRes::Err(SqlState::UndefinedObject),
        ))
        .add_step(Step::new(
            format!("CREATE DATABASE {}", db_name.as_str()),
            StepRes::Ok,
        ))
        .add_step(Step::new(
            format!("USE DATABASE {}", db_name.as_str()),
            StepRes::Ok,
        ))
        .run_with_manual_db_control()
        .await;
}

#[async_std::test]
async fn test_create_database_session_ab() {
    let db_name = DatabaseName::random();
    let _db_cleaner = SqliteDatabaseCleaner::new(db_name.clone());

    SqlTestSessionAb::default()
        .add_step(
            SessionAb::A,
            Step::new(format!("CREATE DATABASE {}", db_name.as_str()), StepRes::Ok),
        )
        .add_step(
            SessionAb::B,
            Step::new(
                format!("CREATE DATABASE {}", db_name.as_str()),
                StepRes::Err(SqlState::DuplicateDatabase),
            ),
        )
        .run_with_manual_db_control()
        .await;
}

#[async_std::test]
async fn test_use_database_session_ab() -> ApllodbResult<()> {
    let db_name = DatabaseName::random();
    let _db_cleaner = SqliteDatabaseCleaner::new(db_name.clone());

    SqlTestSessionAb::default()
        .add_step(
            SessionAb::A,
            Step::new(format!("CREATE DATABASE {}", db_name.as_str()), StepRes::Ok),
        )
        .add_step(
            SessionAb::B,
            Step::new(format!("USE DATABASE {}", db_name.as_str()), StepRes::Ok),
        )
        .add_step(
            SessionAb::A,
            Step::new(format!("USE DATABASE {}", db_name.as_str()), StepRes::Ok),
        )
        .run_with_manual_db_control()
        .await;

    Ok(())
}
