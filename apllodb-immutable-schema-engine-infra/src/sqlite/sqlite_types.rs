use std::marker::PhantomData;

use apllodb_immutable_schema_engine_domain::abstract_types::ImmutableSchemaAbstractTypes;

use crate::{
    engine::ApllodbImmutableSchemaEngine, immutable_schema_row_iter::ImmutableSchemaRowIter,
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
pub struct SqliteTypes<'sqcn> {
    marker_: PhantomData<&'sqcn ()>,
}

impl<'sqcn> ImmutableSchemaAbstractTypes for SqliteTypes<'sqcn> {
    type VRRId = SqliteRowid;

    type ImmutableSchemaRowIter = ImmutableSchemaRowIter;
    type VersionRowIter = SqliteRowIterator;

    type VTableRepo = VTableRepositoryImpl<'sqcn>;
    type VersionRepo = VersionRepositoryImpl<'sqcn>;
}

// Fill structs' type parameters in domain / application layers.
pub(crate) type VRREntriesInVersion<'sqcn> =
    apllodb_immutable_schema_engine_domain::version_revision_resolver::vrr_entries_in_version::VRREntriesInVersion<SqliteTypes< 'sqcn>>;
pub(crate) type VRREntries<'sqcn> =
    apllodb_immutable_schema_engine_domain::version_revision_resolver::vrr_entries::VRREntries<
        SqliteTypes<'sqcn>,
    >;
pub(crate) type VRREntry<'sqcn> =
    apllodb_immutable_schema_engine_domain::version_revision_resolver::vrr_entry::VRREntry<
        SqliteTypes<'sqcn>,
    >;
