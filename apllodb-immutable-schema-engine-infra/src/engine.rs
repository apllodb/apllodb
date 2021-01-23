use std::{cell::RefCell, rc::Rc};

use apllodb_storage_engine_interface::StorageEngine;

use crate::{
    access_methods::{
        with_db_methods_impl::WithDbMethodsImpl, with_tx_methods_impl::WithTxMethodsImpl,
        without_db_methods_impl::WithoutDbMethodsImpl,
    },
    sqlite::sqlite_resource_pool::{db_pool::SqliteDatabasePool, tx_pool::SqliteTxPool},
};

/// Storage engine implementation.
#[derive(Clone, Debug)]
pub struct ApllodbImmutableSchemaEngine {
    db_pool: Rc<RefCell<SqliteDatabasePool>>,
    tx_pool: Rc<RefCell<SqliteTxPool>>,
}

impl StorageEngine for ApllodbImmutableSchemaEngine {
    type WithoutDb = WithoutDbMethodsImpl;
    type WithDb = WithDbMethodsImpl;
    type WithTx = WithTxMethodsImpl;

    fn without_db(&self) -> WithoutDbMethodsImpl {
        WithoutDbMethodsImpl::new(self.db_pool.clone())
    }

    fn with_db(&self) -> WithDbMethodsImpl {
        WithDbMethodsImpl::new(self.db_pool.clone(), self.tx_pool.clone())
    }

    fn with_tx(&self) -> WithTxMethodsImpl {
        WithTxMethodsImpl::new(self.db_pool.clone(), self.tx_pool.clone())
    }
}

impl ApllodbImmutableSchemaEngine {
    pub fn new() -> Self {
        let db_pool = Rc::new(RefCell::new(SqliteDatabasePool::default()));
        let tx_pool = Rc::new(RefCell::new(SqliteTxPool::default()));
        Self { db_pool, tx_pool }
    }
}
