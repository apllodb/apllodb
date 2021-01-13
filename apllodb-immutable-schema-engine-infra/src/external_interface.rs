use apllodb_shared_components::{SessionWithDb, SessionWithTx, SessionWithoutDb};
use apllodb_storage_engine_interface::StorageEngine;

use crate::{
    access_methods::{
        methods_with_db_impl::MethodsWithDbImpl, methods_with_tx_impl::MethodsWithTxImpl,
        methods_without_db_impl::MethodsWithoutDbImpl,
    },
    db_repo::DbRepo,
    tx_repo::TxRepo,
};

/// Storage engine implementation.
///
/// # Lifetime parameters
///
/// - `'sess`: shorthand for `'session`.
#[derive(Debug, Default)]
pub struct ApllodbImmutableSchemaEngine<'sess> {
    db_repo: DbRepo,
    tx_repo: TxRepo<'sess>,
}

impl<'sess> StorageEngine<'sess> for ApllodbImmutableSchemaEngine<'sess> {
    type MethWithoutDb = MethodsWithoutDbImpl<'sess>;
    type MethWithDb = MethodsWithDbImpl<'sess>;
    type MethWithTx = MethodsWithTxImpl<'sess>;

    fn without_db(&'sess self, session: &'sess SessionWithoutDb) -> Self::MethWithoutDb {
        MethodsWithoutDbImpl::new(session, &mut self.db_repo)
    }

    fn with_db(&'sess self, session: &'sess SessionWithDb) -> Self::MethWithDb {
        MethodsWithDbImpl::new(session, &mut self.db_repo, &mut self.tx_repo)
    }

    fn with_tx(&'sess self, session: &'sess SessionWithTx) -> Self::MethWithTx {
        MethodsWithTxImpl::new(session, &mut self.tx_repo)
    }
}
