use apllodb_immutable_schema_engine_domain::abstract_types::ImmutableSchemaAbstractTypes;

use crate::external_interface::ApllodbImmutableSchemaEngine;

use super::{
    row_iterator::SqliteRowIterator,
    transaction::sqlite_tx::repository::{
        version_repository_impl::VersionRepositoryImpl,
        vtable_repository_impl::VTableRepositoryImpl,
    },
};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SqliteTypes;

impl<'tx, 'db: 'tx> ImmutableSchemaAbstractTypes<'tx, 'db, ApllodbImmutableSchemaEngine>
    for SqliteTypes
{
    type VersionRowIter = SqliteRowIterator;

    type VersionRepo = VersionRepositoryImpl<'tx, 'db>;
    type VTableRepo = VTableRepositoryImpl<'tx, 'db>;
}
