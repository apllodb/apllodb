use std::marker::PhantomData;

use apllodb_storage_engine_interface::StorageEngine;

// Hide SQLite (implementation detail)
pub use crate::sqlite::database::SqliteDatabase as ApllodbImmutableSchemaDb;
pub use crate::sqlite::transaction::sqlite_tx::SqliteTx as ApllodbImmutableSchemaTx;

pub use crate::access_methods::ddl_methods_impl::DDLMethodsImpl as ApllodbImmutableSchemaDDL;
pub use crate::access_methods::dml_methods_impl::DMLMethodsImpl as ApllodbImmutableSchemaDML;

/// Storage engine implementation.
#[derive(Hash, Debug, Default)]
pub struct ApllodbImmutableSchemaEngine<'db> {
    _marker: PhantomData<&'db ()>,
}

impl<'db> StorageEngine for ApllodbImmutableSchemaEngine<'db> {
    type Db = ApllodbImmutableSchemaDb;
    type Tx = ApllodbImmutableSchemaTx<'db>;
    type DDL = ApllodbImmutableSchemaDDL;
    type DML = ApllodbImmutableSchemaDML;
}
