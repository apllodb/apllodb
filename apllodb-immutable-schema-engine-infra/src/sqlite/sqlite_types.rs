use std::marker::PhantomData;

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
pub struct SqliteTypes<'repo, 'db: 'repo> {
    marker_: PhantomData<&'repo &'db ()>,
}

impl<'repo, 'db: 'repo> ImmutableSchemaAbstractTypes<ApllodbImmutableSchemaEngine>
    for SqliteTypes<'repo, 'db>
{
    type VRRId = SqliteRowid;

    type ImmutableSchemaRowIter = ImmutableSchemaRowIter;
    type VersionRowIter = SqliteRowIterator;

    type VTableRepo = VTableRepositoryImpl<'repo, 'db>;
    type VersionRepo = VersionRepositoryImpl<'repo, 'db>;
}

// Fill structs' type parameters in domain / application layers.
pub(crate) type VRREntriesInVersion<'vrr, 'db> =
    apllodb_immutable_schema_engine_domain::version_revision_resolver::vrr_entries_in_version::VRREntriesInVersion< ApllodbImmutableSchemaEngine, SqliteTypes<'vrr, 'db>>;
pub(crate) type VRREntries<'vrr, 'db> =
    apllodb_immutable_schema_engine_domain::version_revision_resolver::vrr_entries::VRREntries<
        ApllodbImmutableSchemaEngine,
        SqliteTypes<'vrr, 'db>,
    >;
pub(crate) type VRREntry<'vrr, 'db> =
    apllodb_immutable_schema_engine_domain::version_revision_resolver::vrr_entry::VRREntry<
        ApllodbImmutableSchemaEngine,
        SqliteTypes<'vrr, 'db>,
    >;
