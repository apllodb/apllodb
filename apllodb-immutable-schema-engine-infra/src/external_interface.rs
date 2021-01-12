use std::marker::PhantomData;

use apllodb_storage_engine_interface::StorageEngine;

use crate::access_methods::{
    database_methods_impl::{db_repo::DbRepo, DatabaseMethodsImpl},
    transaction_methods_impl::{tx_repo::TxRepo, TransactionMethodsImpl},
};
// Hide SQLite (implementation detail)

pub use crate::access_methods::ddl_methods_impl::DDLMethodsImpl as ApllodbImmutableSchemaDDL;
pub use crate::access_methods::dml_methods_impl::DMLMethodsImpl as ApllodbImmutableSchemaDML;

/// Storage engine implementation.
#[derive(Debug, Default)]
pub struct ApllodbImmutableSchemaEngine<'acc, 'sess: 'acc> {
    db_repo: DbRepo,
    tx_repo: TxRepo<'sess>,

    marker_: PhantomData<&'acc ()>,
}

impl<'acc, 'sess: 'acc> StorageEngine for ApllodbImmutableSchemaEngine<'acc, 'sess> {
    type Db = DatabaseMethodsImpl<'acc>;
    type Tx = TransactionMethodsImpl<'acc, 'sess>;
    type DDL = ApllodbImmutableSchemaDDL<'sess>;
    type DML = ApllodbImmutableSchemaDML<'sess>;
    type RefSelf = &'acc mut Self;

    fn db(slf: &'acc mut Self) -> Self::Db {
        DatabaseMethodsImpl::new(&mut slf.db_repo)
    }

    fn tx(slf: &'acc mut Self) -> Self::Tx {
        TransactionMethodsImpl::new(&mut slf.db_repo, &mut slf.tx_repo)
    }

    fn ddl(slf: &'acc mut Self) -> Self::DDL {
        todo!()
    }

    fn dml(slf: &'acc mut Self) -> Self::DML {
        todo!()
    }
}
