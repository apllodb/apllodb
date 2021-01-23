use crate::{StorageEngine, WithDbMethods, WithoutDbMethods};
use apllodb_shared_components::{ApllodbResult, DatabaseName, SessionWithTx, SessionWithoutDb};
use uuid::Uuid;

pub async fn session_with_tx<Engine: StorageEngine>(
    engine: &Engine,
) -> ApllodbResult<SessionWithTx> {
    let db_name = format!("{}", Uuid::new_v4());
    let db_name = DatabaseName::new(db_name)?;

    let session = engine
        .without_db()
        .use_database(SessionWithoutDb::default(), db_name)
        .await?;

    let session = engine.with_db().begin_transaction(session).await?;

    Ok(session)
}
