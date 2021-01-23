use crate::{MockWithDbMethods, MockWithTxMethods, MockWithoutDbMethods, StorageEngine};

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
