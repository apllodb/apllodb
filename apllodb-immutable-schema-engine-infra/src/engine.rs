use std::{cell::RefCell, rc::Rc};

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

impl ApllodbImmutableSchemaEngine {
    pub fn new() -> Self {
        let db_pool = Rc::new(RefCell::new(SqliteDatabasePool::default()));
        let tx_pool = Rc::new(RefCell::new(SqliteTxPool::default()));
        Self { db_pool, tx_pool }
    }

    pub fn without_db_methods(&self) -> WithoutDbMethodsImpl {
        WithoutDbMethodsImpl::new(self.db_pool.clone())
    }

    pub fn with_db_methods(&self) -> WithDbMethodsImpl {
        WithDbMethodsImpl::new(self.db_pool.clone(), self.tx_pool.clone())
    }

    pub fn with_tx_methods(&self) -> WithTxMethodsImpl {
        WithTxMethodsImpl::new(self.tx_pool.clone())
    }
}
