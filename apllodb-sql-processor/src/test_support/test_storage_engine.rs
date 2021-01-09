use super::mock_ddl::MockDDL;
use super::mock_dml::MockDML;
use apllodb_shared_components::{
    ApllodbResult, Database, DatabaseName, Transaction, TransactionId,
};
use apllodb_storage_engine_interface::StorageEngine;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) struct TestDatabase;
impl Database for TestDatabase {
    fn name(&self) -> &apllodb_shared_components::DatabaseName {
        unimplemented!()
    }

    fn use_database(_name: DatabaseName) -> ApllodbResult<Self> {
        unimplemented!()
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) struct TestTx;
impl Transaction for TestTx {
    type Db = TestDatabase;
    type RefDb = TestDatabase;
    type TID = TestTransactionId;

    fn id(&self) -> &Self::TID {
        &TestTransactionId
    }

    fn begin(_db: Self::RefDb) -> ApllodbResult<Self> {
        Ok(TestTx)
    }

    fn commit(self) -> ApllodbResult<()> {
        Ok(())
    }

    fn abort(self) -> ApllodbResult<()> {
        Ok(())
    }

    fn database_name(&self) -> &DatabaseName {
        todo!()
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) struct TestTransactionId;
impl TransactionId for TestTransactionId {}

#[derive(Debug, Default)]
pub(crate) struct TestStorageEngine;
impl StorageEngine for TestStorageEngine {
    type Tx = TestTx;
    type Db = TestDatabase;
    type DDL = MockDDL;
    type DML = MockDML;
}
