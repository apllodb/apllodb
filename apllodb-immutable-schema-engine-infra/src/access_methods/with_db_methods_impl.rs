use std::{cell::RefCell, rc::Rc};

use crate::sqlite::{
    sqlite_resource_pool::{db_pool::SqliteDatabasePool, tx_pool::SqliteTxPool},
    transaction::sqlite_tx::SqliteTx,
};
use apllodb_shared_components::SessionId;
use apllodb_storage_engine_interface::WithDbMethods;
use futures::FutureExt;

use super::BoxFutRes;

#[derive(Clone, Debug, Default)]
pub struct WithDbMethodsImpl {
    db_pool: Rc<RefCell<SqliteDatabasePool>>,
    tx_pool: Rc<RefCell<SqliteTxPool>>,
}

impl WithDbMethodsImpl {
    pub(crate) fn new(
        db_pool: Rc<RefCell<SqliteDatabasePool>>,
        tx_pool: Rc<RefCell<SqliteTxPool>>,
    ) -> Self {
        Self { db_pool, tx_pool }
    }
}

impl WithDbMethods for WithDbMethodsImpl {
    fn begin_transaction_core(self, sid: SessionId) -> BoxFutRes<()> {
        async move {
            let db_pool = self.db_pool.borrow();

            let db = db_pool.get_db(&sid)?;
            let tx = SqliteTx::begin(db).await?;

            self.tx_pool.borrow_mut().insert_tx(&sid, tx)?;

            Ok(())
        }
        .boxed_local()
    }
}
