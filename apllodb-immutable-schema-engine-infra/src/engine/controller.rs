use crate::{
    engine::ApllodbImmutableSchemaEngine,
    sqlite::{database::SqliteDatabase, transaction::sqlite_tx::SqliteTx},
};
use apllodb_shared_components::ApllodbResult;
use apllodb_shared_components::{DatabaseName, SessionWithDb, SessionWithTx, SessionWithoutDb};
use apllodb_storage_engine_interface::StorageEngine;
use futures::future::FutureExt;
use futures::Future;
use std::pin::Pin;
use tarpc::context;

type BoxFutResult<S> = Pin<Box<dyn Future<Output = ApllodbResult<S>>>>;

// Cannot use `#[tarpc::server]` because it forces return future type with `Send`.
// See: <https://github.com/google/tarpc/issues/338>
impl StorageEngine for ApllodbImmutableSchemaEngine {
    type UseDatabaseFut = BoxFutResult<SessionWithDb>;
    type BeginTransactionFut = BoxFutResult<SessionWithTx>;

    fn use_database(
        self,
        _: context::Context,
        session: SessionWithoutDb,
        database: DatabaseName,
    ) -> Self::UseDatabaseFut {
        async move {
            let db = SqliteDatabase::use_database(database.clone()).await?;
            self.pool.borrow_mut().insert_db(session.get_id(), db);

            Ok(session.upgrade(database))
        }
        .boxed_local()
    }

    fn begin_transaction(
        self,
        _: context::Context,
        session: SessionWithDb,
    ) -> Self::BeginTransactionFut {
        async move {
            let db = self.pool.borrow_mut().get_db_mut(session.get_id())?;
            let tx = SqliteTx::begin(db).await?;
            self.pool.borrow_mut().insert_tx(session.get_id(), tx);

            Ok(session.upgrade())
        }
        .boxed_local()
    }
}
