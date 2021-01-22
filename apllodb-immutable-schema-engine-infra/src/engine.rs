use std::{cell::RefCell, rc::Rc};

use crate::{
    access_methods::without_db_methods_impl::WithoutDbMethodsImpl,
    sqlite::sqlite_resource_pool::{db_pool::SqliteDatabasePool, tx_pool::SqliteTxPool},
};

/// Storage engine implementation.
#[derive(Clone, Debug)]
pub struct ApllodbImmutableSchemaEngine {
    db_pool: Rc<RefCell<SqliteDatabasePool>>,
    tx_pool: Rc<RefCell<SqliteTxPool>>,
}

impl ApllodbImmutableSchemaEngine {
    pub fn new(
        db_pool: Rc<RefCell<SqliteDatabasePool>>,
        tx_pool: Rc<RefCell<SqliteTxPool>>,
    ) -> Self {
        Self { db_pool, tx_pool }
    }

    pub fn without_db_methods(&self) -> WithoutDbMethodsImpl {
        WithoutDbMethodsImpl::new(self.db_pool.clone())
    }
}
