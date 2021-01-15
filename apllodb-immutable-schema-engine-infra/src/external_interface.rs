use apllodb_shared_components::{
    ApllodbResult, ColumnDefinition, DatabaseName, SessionWithDb, SessionWithTx, SessionWithoutDb,
    TableConstraints, TableName,
};
use apllodb_storage_engine_interface::StorageEngine;

/// Storage engine implementation.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ApllodbImmutableSchemaEngine;

#[tarpc::server]
impl StorageEngine for ApllodbImmutableSchemaEngine {
    async fn use_database(
        session: SessionWithoutDb,
        database: DatabaseName,
    ) -> ApllodbResult<SessionWithDb> {
        todo!()
    }

    async fn begin_transaction(session: SessionWithDb) -> ApllodbResult<SessionWithTx> {
        todo!()
    }

    async fn commit_transaction(session: SessionWithTx) -> ApllodbResult<()> {
        todo!()
    }

    async fn abort_transaction(session: SessionWithTx) -> ApllodbResult<()> {
        todo!()
    }

    async fn create_table(
        session: SessionWithTx,
        table_name: TableName,
        table_constraints: TableConstraints,
        column_definitions: Vec<ColumnDefinition>,
    ) -> ApllodbResult<SessionWithTx> {
        todo!()
    }
}
