pub mod vrr_entries;
pub mod vrr_entries_in_version;
pub mod vrr_entry;
pub mod vrr_id;

use apllodb_shared_components::ApllodbResult;

use crate::{
    abstract_types::ImmutableSchemaAbstractTypes, row::pk::apparent_pk::ApparentPrimaryKey,
    version::id::VersionId, vtable::id::VTableId, vtable::VTable,
};

use self::{vrr_entries::VrrEntries, vrr_entry::VrrEntry};
use async_trait::async_trait;

/// Resolves latest revision among rows with the same PK.
#[async_trait(?Send)]
pub trait VersionRevisionResolver<Types: ImmutableSchemaAbstractTypes> {
    /// Returns undefined order of VrrEntry
    async fn probe(
        &self,
        vtable_id: &VTableId,
        pks: Vec<ApparentPrimaryKey>,
    ) -> ApllodbResult<VrrEntries<Types>>;

    /// Returns undefined order of VrrEntry
    async fn scan(&self, vtable: &VTable) -> ApllodbResult<VrrEntries<Types>>;

    async fn register(
        &self,
        version_id: &VersionId,
        pk: ApparentPrimaryKey,
    ) -> ApllodbResult<VrrEntry<Types>>;

    async fn deregister(
        &self,
        vtable_id: &VTableId,
        pks: &[ApparentPrimaryKey],
    ) -> ApllodbResult<()>;

    async fn deregister_all(&self, vtable: &VTable) -> ApllodbResult<()>;
}
