use std::{cell::RefCell, rc::Rc};

use crate::sqlite::{
    sqlite_resource_pool::{db_pool::SqliteDatabasePool, tx_pool::SqliteTxPool},
    transaction::sqlite_tx::SqliteTx,
};
use apllodb_shared_components::{
    ApllodbResult, ApllodbSessionError, Session, SessionWithDb, SessionWithTx,
};
use apllodb_storage_engine_interface::WithDbMethods;
use futures::FutureExt;

use super::FutRes;

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
    fn begin_transaction(self, session: SessionWithDb) -> FutRes<SessionWithTx> {
        async fn helper(
            slf: WithDbMethodsImpl,
            session: SessionWithDb,
        ) -> ApllodbResult<SessionWithTx> {
            let sid = session.get_id().clone();
            let db_pool = slf.db_pool.borrow();

            let db = db_pool.get_db(session.get_id())?;
            let tx = SqliteTx::begin(db).await?;
            slf.tx_pool.borrow_mut().insert_tx(&sid, tx)?;
            Ok(session.upgrade())
        }

        async move {
            helper(self, session)
                .await
                .map_err(|e| ApllodbSessionError::new(e, Session::from(session)))
        }
        .boxed_local()
    }
}
