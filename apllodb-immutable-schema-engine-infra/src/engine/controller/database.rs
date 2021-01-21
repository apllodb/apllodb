use crate::{engine::ApllodbImmutableSchemaEngine, sqlite::database::SqliteDatabase};
use apllodb_shared_components::{DatabaseName, SessionWithDb, SessionWithoutDb};
use apllodb_storage_engine_interface::StorageEngine;
use futures::future::FutureExt;
use tarpc::context;

use super::BoxFutResult;

impl StorageEngine for ApllodbImmutableSchemaEngine {
    type UseDatabaseFut = BoxFutResult<SessionWithDb>;

    fn use_database(
        self,
        _: context::Context,
        session: SessionWithoutDb,
        database: DatabaseName,
    ) -> Self::UseDatabaseFut {
        async move {
            let db = SqliteDatabase::use_database(database.clone()).await?;
            self.pool.borrow_mut().register_db(session.get_id(), db);

            Ok(session.upgrade(database))
        }
        .boxed_local()
    }
}
