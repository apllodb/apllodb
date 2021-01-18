use std::{
    future::{self, Ready},
    sync::{Arc, RwLock},
};

use apllodb_shared_components::{
    ApllodbResult, ColumnDefinition, DatabaseName, SessionWithDb, SessionWithTx, SessionWithoutDb,
    TableConstraints, TableName,
};
use apllodb_storage_engine_interface::StorageEngine;
use tarpc::context;

use crate::sqlite::{database::SqliteDatabase, sqlite_resource_pool::SqliteResourcePool};

/// Storage engine implementation.
///
/// # Lifetime parameters
///
/// - `'sqcn`: shorthand of `'sqlite_connection`.
#[derive(Clone, Debug, Default)]
pub struct ApllodbImmutableSchemaEngine<'sqcn> {
    // FIXME Consider sharding by SessionId to avoid writer contention using something like dashmap.
    // see: <https://tokio.rs/tokio/tutorial/shared-state#tasks-threads-and-contention>
    pool: Arc<RwLock<SqliteResourcePool<'sqcn>>>,
}

#[tarpc::server]
impl<'sqcn> StorageEngine for ApllodbImmutableSchemaEngine<'sqcn> {
    async fn use_database(
        self,
        _: context::Context,
        session: SessionWithoutDb,
        database: DatabaseName,
    ) -> ApllodbResult<SessionWithDb> {
        // FIXME actually not async at all because rusqlite::Connection internally has `RefCell: !Sync`.
        // So here giving up using `async fn` (whose body must be `Send`).

        let db = SqliteDatabase::use_database(database.clone())
            .await
            .unwrap();
        let mut pool = self.pool.write().unwrap(); // TODO use ?
        let db_idx = pool.db_arena.insert(db);
        pool.sess_db.insert(session.get_id().clone(), db_idx);

        Ok(session.upgrade(database))
    }

    async fn begin_transaction(
        self,
        _: context::Context,
        session: SessionWithDb,
    ) -> ApllodbResult<SessionWithTx> {
        todo!()
    }

    async fn commit_transaction(
        self,
        _: context::Context,
        session: SessionWithTx,
    ) -> ApllodbResult<()> {
        todo!()
    }

    async fn abort_transaction(
        self,
        _: context::Context,
        session: SessionWithTx,
    ) -> ApllodbResult<()> {
        todo!()
    }

    async fn create_table(
        self,
        _: context::Context,
        session: SessionWithTx,
        table_name: TableName,
        table_constraints: TableConstraints,
        column_definitions: Vec<ColumnDefinition>,
    ) -> ApllodbResult<(SessionWithTx)> {
        todo!()
    }
}
