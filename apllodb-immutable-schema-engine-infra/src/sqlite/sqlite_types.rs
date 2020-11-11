use std::marker::PhantomData;

use apllodb_immutable_schema_engine_domain::abstract_types::ImmutableSchemaAbstractTypes;

use crate::{
    external_interface::ApllodbImmutableSchemaEngine,
    immutable_schema_row_iter::ImmutableSchemaRowIter,
};

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
pub struct SqliteTypes<'db>(PhantomData<&'db ()>);

impl<'tx, 'db: 'tx> ImmutableSchemaAbstractTypes<ApllodbImmutableSchemaEngine> for SqliteTypes<'db> {
    type VersionRowIter = SqliteRowIterator;
    type ImmutableSchemaRowIter = ImmutableSchemaRowIter;

    type ImmutableSchemaTx = SqliteTx<'db>;

    type VersionRepo = VersionRepositoryImpl<'tx, 'db>;
    type VTableRepo = VTableRepositoryImpl<'tx, 'db>;
}
