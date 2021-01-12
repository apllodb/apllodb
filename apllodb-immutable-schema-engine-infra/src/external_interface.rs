use std::marker::PhantomData;

use apllodb_storage_engine_interface::StorageEngine;

use crate::access_methods::{
    database_methods_impl::DatabaseMethodsImpl, transaction_methods_impl::TransactionMethodsImpl,
};
// Hide SQLite (implementation detail)

pub use crate::access_methods::ddl_methods_impl::DDLMethodsImpl as ApllodbImmutableSchemaDDL;
pub use crate::access_methods::dml_methods_impl::DMLMethodsImpl as ApllodbImmutableSchemaDML;

/// Storage engine implementation.
#[derive(Hash, Debug, Default)]
pub struct ApllodbImmutableSchemaEngine<'sess> {
    _marker: PhantomData<&'sess ()>,
}

impl<'sess> StorageEngine for ApllodbImmutableSchemaEngine<'sess> {
    type Db = DatabaseMethodsImpl;
    type Tx = TransactionMethodsImpl<'sess>;
    type DDL = ApllodbImmutableSchemaDDL<'sess>;
    type DML = ApllodbImmutableSchemaDML<'sess>;
}
