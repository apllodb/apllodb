use std::marker::PhantomData;

use crate::sqlite::sqlite_types::SqliteTypes;
use apllodb_immutable_schema_engine_interface_adapter::TransactionController;
use apllodb_shared_components::{data_structure::DatabaseName, error::ApllodbResult};
use apllodb_storage_engine_interface::StorageEngine;

pub use crate::sqlite::database::SqliteDatabase as ApllodbImmutableSchemaDb;

/// Storage engine implementation.
#[derive(Hash, Debug)]
pub struct ApllodbImmutableSchemaEngine<'db>(PhantomData<&'db ()>);

impl<'db> StorageEngine for ApllodbImmutableSchemaEngine<'db> {
    type Tx = TransactionController<Self, SqliteTypes<'db>>;

    // TODO UndefinedDatabase error.
    fn use_database(database_name: &DatabaseName) -> ApllodbResult<ApllodbImmutableSchemaDb> {
        ApllodbImmutableSchemaDb::new(database_name.clone())
    }
}
