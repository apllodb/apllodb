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
pub struct SqliteTypes<'repo, 'sess: 'repo> {
    marker_: PhantomData<&'repo &'sess ()>,
}

impl<'repo, 'sess: 'repo> ImmutableSchemaAbstractTypes<ApllodbImmutableSchemaEngine<'repo, 'sess>>
    for SqliteTypes<'repo, 'sess>
{
    type VRRId = SqliteRowid;

    type ImmutableSchemaRowIter = ImmutableSchemaRowIter;
    type VersionRowIter = SqliteRowIterator;

    type VTableRepo = VTableRepositoryImpl<'repo, 'sess>;
    type VersionRepo = VersionRepositoryImpl<'repo, 'sess>;
}

// Fill structs' type parameters in domain / application layers.
pub(crate) type VRREntriesInVersion<'vrr, 'sess> =
    apllodb_immutable_schema_engine_domain::version_revision_resolver::vrr_entries_in_version::VRREntriesInVersion< ApllodbImmutableSchemaEngine<'vrr, 'sess>, SqliteTypes<'vrr, 'sess>>;
pub(crate) type VRREntries<'vrr, 'sess> =
    apllodb_immutable_schema_engine_domain::version_revision_resolver::vrr_entries::VRREntries<
        ApllodbImmutableSchemaEngine<'vrr, 'sess>,
        SqliteTypes<'vrr, 'sess>,
    >;
pub(crate) type VRREntry<'vrr, 'sess> =
    apllodb_immutable_schema_engine_domain::version_revision_resolver::vrr_entry::VRREntry<
        ApllodbImmutableSchemaEngine<'vrr, 'sess>,
        SqliteTypes<'vrr, 'sess>,
    >;
