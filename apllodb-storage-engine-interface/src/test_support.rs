pub use crate::access_methods::{
    with_db_methods::MockWithDbMethods, with_tx_methods::MockWithTxMethods,
    without_db_methods::MockWithoutDbMethods,
};
use crate::StorageEngine;

#[derive(Debug, Default)]
pub struct TestStorageEngine;

impl StorageEngine for TestStorageEngine {
    type WithoutDb = MockWithoutDbMethods;
    type WithDb = MockWithDbMethods;
    type WithTx = MockWithTxMethods;

    fn without_db(&self) -> Self::WithoutDb {
        MockWithoutDbMethods::new()
    }

    fn with_db(&self) -> Self::WithDb {
        MockWithDbMethods::new()
    }

    fn with_tx(&self) -> Self::WithTx {
        MockWithTxMethods::new()
    }
}
