pub mod vrr_entries;
pub mod vrr_entry;
pub mod vrr_id;

use apllodb_shared_components::error::ApllodbResult;
use apllodb_storage_engine_interface::StorageEngine;

use crate::{
    abstract_types::ImmutableSchemaAbstractTypes, row::pk::apparent_pk::ApparentPrimaryKey,
    version::id::VersionId, vtable::id::VTableId, vtable::VTable,
};

use self::{vrr_entries::VRREntries, vrr_entry::VRREntry};

/// Resolves latest revision among rows with the same PK.
pub trait VersionRevisionResolver<
    'vrr,
    'db: 'vrr,
    Engine: StorageEngine<'vrr, 'db>,
    Types: ImmutableSchemaAbstractTypes<'vrr, 'db, Engine>,
>
{
    fn create_table(&self, vtable: &VTable) -> ApllodbResult<()>;

    // probe : PKをキーにして、最新revisionであるものの「VRR-ID, version, revision」(optional) を返す。
    // pks の指定順序で返却。
    // TODO: 範囲選択に対応するためのI/F
    fn probe(
        &self,
        _vtable_id: &VTableId,
        _pks: Vec<ApparentPrimaryKey>,
    ) -> ApllodbResult<VRREntries<'vrr, 'db, Engine, Types>>;

    // scan : PKでグルーピングした時に最新のrevisionであるものの「VRR-ID, PK, version, revision」を返す。
    // PKの昇順で返却。
    fn scan(&self, _vtable_id: &VTableId) -> ApllodbResult<VRREntries<'vrr, 'db, Engine, Types>>;

    // register : 「PK, version」を受け取り、それをそのPKにおける新revisionとして登録し、VRR-IDを発行する。
    fn register(
        &self,
        _version_id: &VersionId,
        _pk: ApparentPrimaryKey,
    ) -> ApllodbResult<VRREntry<'vrr, 'db, Engine, Types>>;

    // deregister : 「PK」を受け取り、そのPKのレコードを亡き者とする
    fn deregister(&self, _vtable_id: &VTableId, _pk: &ApparentPrimaryKey) -> ApllodbResult<()>;

    fn deregister_all(&self, _vtable: &VTable) -> ApllodbResult<()> {
        todo!()
    }
}
