use apllodb_immutable_schema_engine_domain::abstract_types::ImmutableSchemaAbstractTypes;

use super::{
    sqlite_rowid::SqliteRowid,
    transaction::sqlite_tx::{
        version::repository_impl::VersionRepositoryImpl,
        version_revision_resolver::VersionRevisionResolverImpl,
        vtable::repository_impl::VTableRepositoryImpl,
    },
};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub(crate) struct SqliteTypes;

impl ImmutableSchemaAbstractTypes for SqliteTypes {
    type VrrId = SqliteRowid;

    type VTableRepo = VTableRepositoryImpl;
    type VersionRepo = VersionRepositoryImpl;

    type Vrr = VersionRevisionResolverImpl;
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
pub(crate) type RowSelectionPlan =
    apllodb_immutable_schema_engine_domain::row_selection_plan::RowSelectionPlan<SqliteTypes>;
