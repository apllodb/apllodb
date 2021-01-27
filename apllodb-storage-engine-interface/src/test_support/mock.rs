pub(crate) mod mock_select;

pub use crate::access_methods::{
    with_db_methods::MockWithDbMethods, with_tx_methods::MockWithTxMethods,
    without_db_methods::MockWithoutDbMethods,
};
pub use crate::MockStorageEngine;

use futures::FutureExt;

pub fn default_mock_engine() -> MockStorageEngine {
    let mut engine = MockStorageEngine::new();

    engine.expect_without_db().returning(|| {
        let mut without_db = MockWithoutDbMethods::new();

        without_db
            .expect_create_database()
            .returning(|session, _| async { Ok(session) }.boxed_local());
        without_db
            .expect_use_database()
            .returning(|session, db| async { Ok(session.upgrade(db)) }.boxed_local());

            without_db
    });

    engine.expect_with_db().returning(|| {
        let mut with_db = MockWithDbMethods::new();
        with_db
            .expect_begin_transaction()
            .returning(|session| async { Ok(session.upgrade()) }.boxed_local());
        with_db
    });

    engine
}
