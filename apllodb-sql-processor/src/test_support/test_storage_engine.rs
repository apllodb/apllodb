use super::mock_tx::MockTx;
use apllodb_shared_components::{ApllodbResult, Database, DatabaseName};
use apllodb_storage_engine_interface::{
    StorageEngine, Transaction, TransactionBuilder, TransactionId,
};

pub(crate) struct TestDatabase;
impl Database for TestDatabase {
    fn name(&self) -> &apllodb_shared_components::DatabaseName {
        unimplemented!()
    }
}
impl TestDatabase {
    pub(super) fn new() -> Self {
        Self
    }
}

#[derive(Clone, PartialEq, Debug, new)]
pub(crate) struct TestTxBuilder;
impl TransactionBuilder for TestTxBuilder {}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) struct TestTransactionId;
impl TransactionId for TestTransactionId {}

#[derive(Debug)]
pub(crate) struct TestStorageEngine;
impl StorageEngine for TestStorageEngine {
    type Tx = MockTx;
    type TxBuilder = TestTxBuilder;
    type TID = TestTransactionId;
    type Db = TestDatabase;

    fn use_database(_database_name: &DatabaseName) -> ApllodbResult<TestDatabase> {
        Ok(TestDatabase::new())
    }
}
impl TestStorageEngine {
    pub(crate) fn begin() -> ApllodbResult<MockTx> {
        let ctx = MockTx::begin_context();
        ctx.expect().returning(|_| Ok(MockTx::new()));

        MockTx::begin(TestTxBuilder::new())
    }
}
