use apllodb_storage_engine_interface::StorageEngine;

use crate::access_methods::{
    database_methods_impl::{db_repo::DbRepo, DatabaseMethodsImpl},
    ddl_methods_impl::DDLMethodsImpl,
    dml_methods_impl::DMLMethodsImpl,
    transaction_methods_impl::{tx_repo::TxRepo, TransactionMethodsImpl},
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

impl<'sess> StorageEngine for ApllodbImmutableSchemaEngine<'sess> {
    type Db = DatabaseMethodsImpl<'sess>;
    type Tx = TransactionMethodsImpl<'sess>;
    type DDL = DDLMethodsImpl<'sess>;
    type DML = DMLMethodsImpl<'sess>;

    fn db(&'sess mut self) -> Self::Db {
        DatabaseMethodsImpl::new(&mut self.db_repo)
    }

    fn tx<'caller>(&'caller mut self) -> Self::Tx {
        TransactionMethodsImpl::new(&mut slf.db_repo, &mut slf.tx_repo)
    }

    fn ddl<'caller>(&'caller mut self) -> Self::DDL {
        DDLMethodsImpl::new(&mut slf.tx_repo)
    }

    fn dml<'caller>(&'caller mut self) -> Self::DML {
        DMLMethodsImpl::new(&mut slf.tx_repo)
    }
}
