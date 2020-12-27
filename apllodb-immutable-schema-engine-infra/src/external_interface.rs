use std::marker::PhantomData;

use crate::{
    immutable_schema_row_iter::ImmutableSchemaRowIter,
    sqlite::transaction::{
        sqlite_tx::{SqliteTx, SqliteTxBuilder},
        tx_id::TxId,
    },
};
use apllodb_immutable_schema_engine_domain::row::immutable_row::ImmutableRow;
use apllodb_shared_components::{ApllodbResult, DatabaseName};
use apllodb_storage_engine_interface::{StorageEngine, Transaction};

pub use crate::sqlite::database::SqliteDatabase as ApllodbImmutableSchemaDb;

/// Storage engine implementation.
#[derive(Hash, Debug)]
pub struct ApllodbImmutableSchemaEngine<'db> {
    _marker: PhantomData<&'db ()>,
}

impl<'db> StorageEngine for ApllodbImmutableSchemaEngine<'db> {
    type Tx = SqliteTx<'db>;
    type TxBuilder = SqliteTxBuilder<'db>;
    type TID = TxId;
    type Db = ApllodbImmutableSchemaDb;
    type R = ImmutableRow;
    type RowIter = ImmutableSchemaRowIter;

    // TODO UndefinedDatabase error.
    fn use_database(database_name: &DatabaseName) -> ApllodbResult<ApllodbImmutableSchemaDb> {
        ApllodbImmutableSchemaDb::new(database_name.clone())
    }
}

impl<'db> ApllodbImmutableSchemaEngine<'db> {
    /// Starts transaction and get transaction object.
    pub fn begin_transaction(db: &'db mut ApllodbImmutableSchemaDb) -> ApllodbResult<SqliteTx> {
        let builder = SqliteTxBuilder::new(db);
        SqliteTx::begin(builder)
    }
}
