use std::marker::PhantomData;

use crate::sqlite::transaction::tx_id::TxId;
use apllodb_shared_components::{ApllodbResult, DatabaseName};
use apllodb_storage_engine_interface::StorageEngine;

// Hide SQLite (implementation detail)
pub use crate::sqlite::database::SqliteDatabase as ApllodbImmutableSchemaDb;
pub use crate::sqlite::transaction::sqlite_tx::SqliteTx as ApllodbImmutableSchemaTx;

/// Storage engine implementation.
#[derive(Hash, Debug)]
pub struct ApllodbImmutableSchemaEngine<'db> {
    _marker: PhantomData<&'db ()>,
}

impl<'db> StorageEngine for ApllodbImmutableSchemaEngine<'db> {
    type Db = ApllodbImmutableSchemaDb;
    type Tx = ApllodbImmutableSchemaTx<'db>;
    type TID = TxId;

    // TODO UndefinedDatabase error.
    fn use_database(database_name: &DatabaseName) -> ApllodbResult<ApllodbImmutableSchemaDb> {
        ApllodbImmutableSchemaDb::new(database_name.clone())
    }
}
