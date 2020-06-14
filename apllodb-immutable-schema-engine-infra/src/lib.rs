#![deny(warnings, missing_docs, missing_debug_implementations)]
//! Infrastructure layer of apllodb-immutable-schema-engine.

mod transaction;

pub use transaction::SqliteTx;

use apllodb_storage_engine_interface::StorageEngine;

/// Storage engine implementation.
#[derive(Hash, Debug)]
pub struct ApllodbImmutableSchemaEngine;
impl StorageEngine for ApllodbImmutableSchemaEngine {
    type Tx = SqliteTx;
    fn use_database(database_name: &DatabaseName) -> ApllodbResult<apllodb_storage_engine_interface::Transaction::Db> {
        todo!()
    }
    fn begin_transaction(db: &mut apllodb_storage_engine_interface::Transaction::Db) -> ApllodbResult<Self::Tx> {
        todo!()
    }
}
