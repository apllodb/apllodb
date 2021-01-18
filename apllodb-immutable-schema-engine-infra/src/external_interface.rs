use apllodb_shared_components::{
    ApllodbResult, ColumnDefinition, DatabaseName, SessionWithDb, SessionWithTx, SessionWithoutDb,
    TableConstraints, TableName,
};
use apllodb_storage_engine_interface::StorageEngine;
use tarpc::context;

use crate::sqlite::{database::SqliteDatabase, sqlite_resource_pool::SqliteResourcePool};

/// Storage engine implementation.
#[derive(Clone, Debug, Default)]
pub struct ApllodbImmutableSchemaEngine {}

#[tarpc::server]
impl StorageEngine for ApllodbImmutableSchemaEngine {
    async fn use_database(
        self,
        _: context::Context,
        session: SessionWithoutDb,
        database: DatabaseName,
    ) -> ApllodbResult<SessionWithDb> {
        let db = SqliteDatabase::use_database(database.clone())
            .await
            .unwrap();
        SqliteResourcePool::register_db(session.get_id(), db);

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
    ) -> ApllodbResult<SessionWithTx> {
        todo!()
    }
}
