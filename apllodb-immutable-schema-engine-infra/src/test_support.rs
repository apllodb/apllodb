use apllodb_shared_components::{ApllodbResult, DatabaseName, SessionWithTx, SessionWithoutDb};
use apllodb_storage_engine_interface::{WithDbMethods, WithoutDbMethods};
use uuid::Uuid;

use crate::ApllodbImmutableSchemaEngine;

pub async fn session_with_tx(
    engine: &ApllodbImmutableSchemaEngine,
) -> ApllodbResult<SessionWithTx> {
    let db_name = format!("{}", Uuid::new_v4());
    let db_name = DatabaseName::new(db_name)?;

    let session = engine
        .without_db_methods()
        .use_database(SessionWithoutDb::default(), db_name)
        .await?;

    let session = engine.with_db_methods().begin_transaction(session).await?;

    Ok(session)
}
