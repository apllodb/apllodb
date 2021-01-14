//! Provides a sample implementation of [apllodb-storage-engine-interface::StorageEngine](apllodb-storage-engine-interface::StorageEngine).

use crate::access_methods::{
    methods_with_db::MockMethodsWithDb, methods_with_tx::MockMethodsWithTx,
    methods_without_db::MockMethodsWithoutDb,
};
use crate::StorageEngine;

/// A storage engine implementation.
///
/// Access methods (`Mock*`) can be mocked using [mockall](https://docs.rs/mockall/) crate.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct TestStorageEngine;

impl<'sess> StorageEngine<'sess> for TestStorageEngine {
    type MethWithoutDb = MockMethodsWithoutDb;
    type MethWithDb = MockMethodsWithDb;
    type MethWithTx = MockMethodsWithTx;

    fn without_db(&'sess self) -> Self::MethWithoutDb {
        MockMethodsWithoutDb::new()
    }

    fn with_db(&'sess self) -> Self::MethWithDb {
        MockMethodsWithDb::new()
    }

    fn with_tx(&'sess self) -> Self::MethWithTx {
        MockMethodsWithTx::new()
    }
}
