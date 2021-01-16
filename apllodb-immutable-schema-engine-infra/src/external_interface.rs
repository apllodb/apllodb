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

impl<'sqcn> StorageEngine for ApllodbImmutableSchemaEngine<'sqcn> {
    type UseDatabaseFut = Ready<ApllodbResult<SessionWithDb>>;
    type BeginTransactionFut = Ready<ApllodbResult<SessionWithTx>>;
    type CommitTransactionFut = Ready<ApllodbResult<()>>;
    type AbortTransactionFut = Ready<ApllodbResult<()>>;
    type CreateTableFut = Ready<ApllodbResult<SessionWithTx>>;

    fn use_database(
        self,
        _: context::Context,
        session: SessionWithoutDb,
        database: DatabaseName,
    ) -> Self::UseDatabaseFut {
        // FIXME actually not async at all because rusqlite::Connection internally has `RefCell: !Sync`.
        // So here giving up using `async fn` (whose body must be `Send`).

        let db = SqliteDatabase::use_database(database.clone()).unwrap();
        let mut pool = self.pool.write().unwrap(); // TODO use ?
        let db_idx = pool.db_arena.insert(db);
        pool.sess_db.insert(session.get_id().clone(), db_idx);

        future::ready(Ok(session.upgrade(database)))
    }

    fn begin_transaction(
        self,
        _: context::Context,
        session: SessionWithDb,
    ) -> Self::BeginTransactionFut {
        todo!()
    }

    fn commit_transaction(
        self,
        _: context::Context,
        session: SessionWithTx,
    ) -> Self::CommitTransactionFut {
        todo!()
    }

    fn abort_transaction(
        self,
        _: context::Context,
        session: SessionWithTx,
    ) -> Self::AbortTransactionFut {
        todo!()
    }

    fn create_table(
        self,
        _: context::Context,
        session: SessionWithTx,
        table_name: TableName,
        table_constraints: TableConstraints,
        column_definitions: Vec<ColumnDefinition>,
    ) -> Self::CreateTableFut {
        todo!()
    }
}
