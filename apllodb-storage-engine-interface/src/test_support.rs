

use crate::StorageEngine;
use apllodb_shared_components::{
    ApllodbResult, ColumnDefinition, DatabaseName, SessionWithDb, SessionWithTx, SessionWithoutDb,
    TableConstraints, TableName,
};
use tarpc::context;

/// A compile-ready storage engine implementation.
///
/// Each method does not trigger any side effects (nothing happens on `use_database`, for example).
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct TestStorageEngineImpl;

#[tarpc::server]
impl StorageEngine for TestStorageEngineImpl {
    async fn use_database(
        self,
        _: context::Context,
        session: SessionWithoutDb,
        database: DatabaseName,
    ) -> ApllodbResult<SessionWithDb> {
        Ok(session.upgrade(database))
    }

    async fn begin_transaction(
        self,
        _: context::Context,
        session: SessionWithDb,
    ) -> ApllodbResult<SessionWithTx> {
        Ok(session.upgrade())
    }

    async fn commit_transaction(
        self,
        _: context::Context,
        _session: SessionWithTx,
    ) -> ApllodbResult<()> {
        Ok(())
    }

    async fn abort_transaction(
        self,
        _: context::Context,
        _session: SessionWithTx,
    ) -> ApllodbResult<()> {
        Ok(())
    }

    async fn create_table(
        self,
        _: context::Context,
        session: SessionWithTx,
        _table_name: TableName,
        _table_constraints: TableConstraints,
        _column_definitions: Vec<ColumnDefinition>,
    ) -> ApllodbResult<SessionWithTx> {
        Ok(session)
    }
}
