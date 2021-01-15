use std::sync::Arc;

use apllodb_shared_components::{
    ApllodbResult, ColumnDefinition, DatabaseName, SessionWithDb, SessionWithTx, SessionWithoutDb,
    TableConstraints, TableName,
};
use apllodb_storage_engine_interface::StorageEngine;
use tarpc::context;
use tokio::sync::Mutex;

use crate::sqlite::sqlite_resource_pool::SqliteResourcePool;

/// Storage engine implementation.
///
/// # Lifetime parameters
///
/// - `'sqcn`: shorthand of `'sqlite_connection`.
#[derive(Clone, Debug, Default)]
pub struct ApllodbImmutableSchemaEngine<'sqcn> {
    pool: Arc<Mutex<SqliteResourcePool<'sqcn>>>,
}

#[tarpc::server]
impl<'sqcn> StorageEngine for ApllodbImmutableSchemaEngine<'sqcn> {
    async fn use_database(
        self,
        _: context::Context,
        session: SessionWithoutDb,
        database: DatabaseName,
    ) -> ApllodbResult<SessionWithDb> {
        todo!()
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
    ) -> ApllodbResult<SessionWithTx> {
        todo!()
    }
}
