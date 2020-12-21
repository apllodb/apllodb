use apllodb_immutable_schema_engine_domain::abstract_types::ImmutableSchemaAbstractTypes;

use crate::{
    external_interface::ApllodbImmutableSchemaEngine,
    immutable_schema_row_iter::ImmutableSchemaRowIter,
};

use super::{
    row_iterator::SqliteRowIterator,
    sqlite_rowid::SqliteRowid,
    transaction::sqlite_tx::{
        version::repository_impl::VersionRepositoryImpl,
        vtable::repository_impl::VTableRepositoryImpl,
    },
};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SqliteTypes;

impl<'repo, 'db: 'repo> ImmutableSchemaAbstractTypes<'repo, 'db, ApllodbImmutableSchemaEngine>
    for SqliteTypes
{
    type VRRId = SqliteRowid;

    type ImmutableSchemaRowIter = ImmutableSchemaRowIter;
    type VersionRowIter = SqliteRowIterator;

    type VTableRepo = VTableRepositoryImpl<'repo, 'db>;
    type VersionRepo = VersionRepositoryImpl<'repo, 'db>;
}

// Fill structs' type parameters in domain / application layers.
pub(crate) type VRREntriesInVersion<'vrr, 'db> =
    apllodb_immutable_schema_engine_domain::version_revision_resolver::vrr_entries_in_version::VRREntriesInVersion<'vrr, 'db, ApllodbImmutableSchemaEngine, SqliteTypes>;
pub(crate) type VRREntries<'vrr, 'db> =
    apllodb_immutable_schema_engine_domain::version_revision_resolver::vrr_entries::VRREntries<
        'vrr,
        'db,
        ApllodbImmutableSchemaEngine,
        SqliteTypes,
    >;
pub(crate) type VRREntry<'vrr, 'db> =
    apllodb_immutable_schema_engine_domain::version_revision_resolver::vrr_entry::VRREntry<
        'vrr,
        'db,
        ApllodbImmutableSchemaEngine,
        SqliteTypes,
    >;
pub(crate) type ProjectionResult<'prj, 'db> =
    apllodb_immutable_schema_engine_domain::query::projection::ProjectionResult<
        'prj,
        'db,
        ApllodbImmutableSchemaEngine,
        SqliteTypes,
    >;
