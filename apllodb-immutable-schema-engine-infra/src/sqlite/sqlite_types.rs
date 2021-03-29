use apllodb_immutable_schema_engine_domain::abstract_types::ImmutableSchemaAbstractTypes;

use crate::immutable_schema_row_iter::ImmutableSchemaRowIter;

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

impl ImmutableSchemaAbstractTypes for SqliteTypes {
    type VrrId = SqliteRowid;

    type ImmutableSchemaRowIter = ImmutableSchemaRowIter;
    type VersionRowIter = SqliteRowIterator;

    type VTableRepo = VTableRepositoryImpl;
    type VersionRepo = VersionRepositoryImpl;
}

// Fill structs' type parameters in domain / application layers.
pub(crate) type VrrEntriesInVersion =
    apllodb_immutable_schema_engine_domain::version_revision_resolver::vrr_entries_in_version::VrrEntriesInVersion<SqliteTypes>;
pub(crate) type VrrEntries =
    apllodb_immutable_schema_engine_domain::version_revision_resolver::vrr_entries::VrrEntries<
        SqliteTypes,
    >;
pub(crate) type VrrEntry =
    apllodb_immutable_schema_engine_domain::version_revision_resolver::vrr_entry::VrrEntry<
        SqliteTypes,
    >;
