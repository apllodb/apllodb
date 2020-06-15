//#![deny(warnings, missing_docs, missing_debug_implementations)]

//! Infrastructure layer of apllodb-immutable-schema-engine.

mod sqlite;

use apllodb_immutable_schema_engine_interface_adapter::TransactionController;
use apllodb_shared_components::{data_structure::DatabaseName, error::ApllodbResult};
use apllodb_storage_engine_interface::StorageEngine;
use sqlite::{SqliteDatabase, SqliteRowIterator, SqliteTx};
use std::marker::PhantomData;

/// Storage engine implementation.
#[derive(Hash, Debug)]
pub struct ApllodbImmutableSchemaEngine;

impl<'db> StorageEngine<'db> for ApllodbImmutableSchemaEngine {
    type Tx = TransactionController<'db, SqliteTx<'db>, SqliteRowIterator<'db>>;

    fn use_database(database_name: &DatabaseName) -> ApllodbResult<SqliteDatabase> {
        todo!()
    }
}
