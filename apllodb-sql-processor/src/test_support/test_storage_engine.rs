use apllodb_shared_components::{
    ApllodbResult, DatabaseName, SessionWithDb, SessionWithTx, SessionWithoutDb,
};
use apllodb_storage_engine_interface::StorageEngine;
use tarpc::context;

#[derive(Clone, Debug, Default)]
pub(crate) struct TestStorageEngine;

#[tarpc::server]
impl StorageEngine for TestStorageEngine {
    async fn use_database(
        self,
        _: context::Context,
        _session: SessionWithoutDb,
        _database: DatabaseName,
    ) -> ApllodbResult<SessionWithDb> {
        todo!()
    }

    async fn begin_transaction(
        self,
        _: context::Context,
        _session: SessionWithDb,
    ) -> ApllodbResult<SessionWithTx> {
        todo!()
    }
}
