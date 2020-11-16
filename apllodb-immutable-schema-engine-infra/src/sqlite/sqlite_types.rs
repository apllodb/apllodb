use apllodb_immutable_schema_engine_domain::abstract_types::ImmutableSchemaAbstractTypes;

use crate::external_interface::ApllodbImmutableSchemaEngine;

use super::{
    row_iterator::SqliteRowIterator,
    sqlite_rowid::SqliteRowid,
    transaction::sqlite_tx::repository::{
        version_repository_impl::VersionRepositoryImpl,
        vtable_repository_impl::VTableRepositoryImpl,
    },
};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SqliteTypes;

impl<'repo, 'db: 'repo> ImmutableSchemaAbstractTypes<'repo, 'db, ApllodbImmutableSchemaEngine>
    for SqliteTypes
{
    type VRRId = SqliteRowid;

    type VersionRowIter = SqliteRowIterator;

    type VersionRepo = VersionRepositoryImpl<'repo, 'db>;
    type VTableRepo = VTableRepositoryImpl<'repo, 'db>;
}
