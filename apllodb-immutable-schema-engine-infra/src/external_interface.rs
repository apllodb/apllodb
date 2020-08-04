use crate::sqlite::transaction::sqlite_tx::SqliteTx;
use apllodb_immutable_schema_engine_interface_adapter::TransactionController;
use apllodb_shared_components::{data_structure::DatabaseName, error::ApllodbResult};
use apllodb_storage_engine_interface::StorageEngine;

pub use crate::sqlite::database::SqliteDatabase as ApllodbImmutableSchemaDb;

/// Storage engine implementation.
#[derive(Hash, Debug)]
pub struct ApllodbImmutableSchemaEngine;

impl<'tx, 'db: 'tx> StorageEngine<'tx, 'db> for ApllodbImmutableSchemaEngine {
    type Tx = TransactionController<'tx, 'db, SqliteTx<'db>>;

    // TODO UndefinedDatabase error.
    fn use_database(database_name: &DatabaseName) -> ApllodbResult<ApllodbImmutableSchemaDb> {
        ApllodbImmutableSchemaDb::new(database_name.clone())
    }
}
