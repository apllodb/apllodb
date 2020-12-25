use crate::{
    immutable_schema_row_iter::ImmutableSchemaRowIter,
    sqlite::transaction::{sqlite_tx::SqliteTx, tx_id::TxId},
};
use apllodb_immutable_schema_engine_domain::row::immutable_row::ImmutableRow;
use apllodb_shared_components::{ApllodbResult, DatabaseName};
use apllodb_storage_engine_interface::StorageEngine;

pub use crate::sqlite::database::SqliteDatabase as ApllodbImmutableSchemaDb;

/// Storage engine implementation.
#[derive(Hash, Debug)]
pub struct ApllodbImmutableSchemaEngine;

impl<'tx, 'db: 'tx> StorageEngine<'tx, 'db> for ApllodbImmutableSchemaEngine {
    type Tx = SqliteTx<'db>;
    type TID = TxId;
    type Db = ApllodbImmutableSchemaDb;
    type R = ImmutableRow;
    type RowIter = ImmutableSchemaRowIter;

    // TODO UndefinedDatabase error.
    fn use_database(database_name: &DatabaseName) -> ApllodbResult<ApllodbImmutableSchemaDb> {
        ApllodbImmutableSchemaDb::new(database_name.clone())
    }
}
