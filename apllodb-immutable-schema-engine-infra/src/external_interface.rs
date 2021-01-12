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
    type RefSelf = &'sess mut Self;

    fn db(slf: &'sess mut Self) -> Self::Db {
        DatabaseMethodsImpl::new(&mut slf.db_repo)
    }

    fn tx(slf: &'sess mut Self) -> Self::Tx {
        TransactionMethodsImpl::new(&mut slf.db_repo, &mut slf.tx_repo)
    }

    fn ddl(slf: &'sess mut Self) -> Self::DDL {
        DDLMethodsImpl::new(&mut slf.tx_repo)
    }

    fn dml(slf: &'sess mut Self) -> Self::DML {
        DMLMethodsImpl::new(&mut slf.tx_repo)
    }
}
