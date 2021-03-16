use apllodb_storage_engine_interface::{StorageEngine, WithDbMethods, WithoutDbMethods};
use apllodb_shared_components::{
    ApllodbResult, DatabaseName, Session, SessionWithDb, SessionWithTx, SessionWithoutDb,
};

use super::sqlite_database_cleaner::SqliteDatabaseCleaner;

pub async fn session_with_db<Engine: StorageEngine>(
    engine: &Engine,
) -> ApllodbResult<SessionWithDb> {
    let db_name = DatabaseName::random();
    let _db_cleaner = SqliteDatabaseCleaner::new(db_name.clone());

    let _ = engine
        .without_db()
        .create_database(Session::from(SessionWithoutDb::default()), db_name.clone())
        .await?;

    let session = engine
        .without_db()
        .use_database(SessionWithoutDb::default(), db_name)
        .await?;

    Ok(session)
}

pub async fn session_with_tx<Engine: StorageEngine>(
    engine: &Engine,
) -> ApllodbResult<SessionWithTx> {
    let session = session_with_db(engine).await?;
    let session = engine.with_db().begin_transaction(session).await?;

    Ok(session)
}
