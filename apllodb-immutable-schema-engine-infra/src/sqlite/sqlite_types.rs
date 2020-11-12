use apllodb_immutable_schema_engine_domain::abstract_types::ImmutableSchemaAbstractTypes;

use crate::immutable_schema_row_iter::ImmutableSchemaRowIter;

use super::{
    database::SqliteDatabase,
    row_iterator::SqliteRowIterator,
    transaction::sqlite_tx::SqliteTx,
    transaction::{
        sqlite_tx::repository::{
            version_repository_impl::VersionRepositoryImpl,
            vtable_repository_impl::VTableRepositoryImpl,
        },
        tx_id::TxId,
    },
};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SqliteTypes;

impl<'tx, 'db: 'tx> ImmutableSchemaAbstractTypes<'tx, 'db, Engine> for SqliteTypes {
    type VersionRowIter = SqliteRowIterator;

    type ImmutableSchemaRowIter = ImmutableSchemaRowIter;

    type TID = TxId;

    type Tx = SqliteTx<'db>;

    type Db = SqliteDatabase;

    type VersionRepo = VersionRepositoryImpl<'tx, 'db>;

    type VTableRepo = VTableRepositoryImpl<'tx, 'db>;
}
