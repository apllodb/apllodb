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
pub struct SqliteTypes<'repo, 'sqcn: 'repo> {
    marker_: PhantomData<&'repo &'sqcn ()>,
}

impl<'repo, 'sqcn: 'repo> ImmutableSchemaAbstractTypes for SqliteTypes<'repo, 'sqcn> {
    type VRRId = SqliteRowid;

    type ImmutableSchemaRowIter = ImmutableSchemaRowIter;
    type VersionRowIter = SqliteRowIterator;

    type VTableRepo = VTableRepositoryImpl<'repo, 'sqcn>;
    type VersionRepo = VersionRepositoryImpl<'repo, 'sqcn>;
}

// Fill structs' type parameters in domain / application layers.
pub(crate) type VRREntriesInVersion<'vrr, 'sqcn> =
    apllodb_immutable_schema_engine_domain::version_revision_resolver::vrr_entries_in_version::VRREntriesInVersion<SqliteTypes<'vrr, 'sqcn>>;
pub(crate) type VRREntries<'vrr, 'sqcn> =
    apllodb_immutable_schema_engine_domain::version_revision_resolver::vrr_entries::VRREntries<
        SqliteTypes<'vrr, 'sqcn>,
    >;
pub(crate) type VRREntry<'vrr, 'sqcn> =
    apllodb_immutable_schema_engine_domain::version_revision_resolver::vrr_entry::VRREntry<
        SqliteTypes<'vrr, 'sqcn>,
    >;
