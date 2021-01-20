use std::{cell::RefCell, pin::Pin, rc::Rc};

use apllodb_shared_components::{
    ApllodbResult, ColumnDefinition, DatabaseName, SessionWithDb, SessionWithTx, SessionWithoutDb,
    TableConstraints, TableName,
};
use apllodb_storage_engine_interface::StorageEngine;
use futures::{future::FutureExt, Future};
use tarpc::context;

use crate::sqlite::{database::SqliteDatabase, sqlite_resource_pool::SqliteResourcePool};

type BoxFutResult<S> = Pin<Box<dyn Future<Output = ApllodbResult<S>>>>;

/// Storage engine implementation.
#[derive(Clone, Debug, Default)]
pub struct ApllodbImmutableSchemaEngine {
    // FIXME Consider sharding by SessionId to avoid writer contention using something like dashmap.
    // see: <https://tokio.rs/tokio/tutorial/shared-state#tasks-threads-and-contention>
    pool: Rc<RefCell<SqliteResourcePool<'static>>>,
}

impl StorageEngine for ApllodbImmutableSchemaEngine {
    type UseDatabaseFut = BoxFutResult<SessionWithDb>;

    fn use_database(
        self,
        _: context::Context,
        session: SessionWithoutDb,
        database: DatabaseName,
    ) -> Self::UseDatabaseFut {
        async move {
            let db = SqliteDatabase::use_database(database.clone())
                .await
                .unwrap();
            self.pool.borrow_mut().register_db(session.get_id(), db);

            Ok(session.upgrade(database))
        }
        .boxed_local()
    }
}
