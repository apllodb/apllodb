pub mod vrr_entries;
pub mod vrr_entries_in_version;
pub mod vrr_entry;
pub mod vrr_id;

use apllodb_shared_components::ApllodbResult;
use apllodb_storage_engine_interface::StorageEngine;

use crate::{
    abstract_types::ImmutableSchemaAbstractTypes, row::pk::apparent_pk::ApparentPrimaryKey,
    version::id::VersionId, vtable::id::VTableId, vtable::VTable,
};

use self::{vrr_entries::VRREntries, vrr_entry::VRREntry};

/// Resolves latest revision among rows with the same PK.
pub trait VersionRevisionResolver<
    'sess,
    Engine: StorageEngine<'sess>,
    Types: ImmutableSchemaAbstractTypes<'sess, Engine>,
>
{
    fn create_table(&self, vtable: &VTable) -> ApllodbResult<()>;

    /// Returns undefined order of VRREntry
    fn probe(
        &self,
        vtable_id: &VTableId,
        pks: Vec<ApparentPrimaryKey>,
    ) -> ApllodbResult<VRREntries<'sess, Engine, Types>>;

    /// Returns undefined order of VRREntry
    fn scan(&self, vtable: &VTable) -> ApllodbResult<VRREntries<'sess, Engine, Types>>;

    fn register(
        &self,
        version_id: &VersionId,
        pk: ApparentPrimaryKey,
    ) -> ApllodbResult<VRREntry<'sess, Engine, Types>>;

    fn deregister(&self, vtable_id: &VTableId, pks: &[ApparentPrimaryKey]) -> ApllodbResult<()>;

    fn deregister_all(&self, vtable: &VTable) -> ApllodbResult<()>;
}
