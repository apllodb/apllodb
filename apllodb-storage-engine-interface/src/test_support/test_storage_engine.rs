use crate::access_methods::{
    methods_with_db::MockMethodsWithDb, methods_with_tx::MockMethodsWithTx,
    methods_without_db::MockMethodsWithoutDb,
};
use crate::StorageEngine;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct TestStorageEngine;

impl<'sess> StorageEngine<'sess> for TestStorageEngine {
    type MethWithoutDb = MockMethodsWithoutDb;
    type MethWithDb = MockMethodsWithDb;
    type MethWithTx = MockMethodsWithTx;

    fn without_db(&'sess self) -> Self::MethWithoutDb {
        todo!()
    }

    fn with_db(&'sess self) -> Self::MethWithDb {
        todo!()
    }

    fn with_tx(&'sess self) -> Self::MethWithTx {
        todo!()
    }
}
