use super::mock_tx::MockTx;
use apllodb_shared_components::{ApllodbResult, Database, DatabaseName};
use apllodb_storage_engine_interface::{
    StorageEngine, Transaction, TransactionBuilder, TransactionId,
};

pub(crate) struct StubDatabase;
impl Database for StubDatabase {
    fn name(&self) -> &apllodb_shared_components::DatabaseName {
        unimplemented!()
    }
}
impl StubDatabase {
    pub(super) fn new() -> Self {
        Self
    }
}

#[derive(Clone, PartialEq, Debug, new)]
pub(crate) struct StubTxBuilder;
impl TransactionBuilder for StubTxBuilder {}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) struct StubTransactionId;
impl TransactionId for StubTransactionId {}

#[derive(Debug)]
pub(crate) struct StubStorageEngine;
impl StorageEngine for StubStorageEngine {
    type Tx = MockTx;
    type TxBuilder = StubTxBuilder;
    type TID = StubTransactionId;
    type Db = StubDatabase;

    fn use_database(_database_name: &DatabaseName) -> ApllodbResult<StubDatabase> {
        Ok(StubDatabase::new())
    }
}
impl StubStorageEngine {
    pub(crate) fn begin() -> ApllodbResult<MockTx> {
        let ctx = MockTx::begin_context();
        ctx.expect().returning(|_| Ok(MockTx::new()));

        MockTx::begin(StubTxBuilder::new())
    }
}
