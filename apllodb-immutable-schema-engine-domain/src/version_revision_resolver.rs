pub mod vrr_entry;
pub mod vrr_id;

use apllodb_shared_components::error::ApllodbResult;

use crate::{
    row::pk::apparent_pk::ApparentPrimaryKey, version::id::VersionId, vtable::id::VTableId,
};

use self::vrr_entry::VRREntry;

/// Resolves latest revision among rows with the same PK.
pub trait VersionRevisionResolver {
    // probe : PKをキーにして、最新revisionであるものの「VRR-ID, version, revision」(optional) を返す。
    // pks の指定順序で返却。
    // TODO: 範囲選択に対応するためのI/F
    fn probe(
        &self,
        _vtable_id: &VTableId,
        _pks: Vec<ApparentPrimaryKey>,
    ) -> ApllodbResult<Vec<VRREntry>> {
        unimplemented!()
    }

    // scan : PKでグルーピングした時に最新のrevisionであるものの「VRR-ID, PK, version, revision」を返す。
    // PKの昇順で返却。
    fn scan(&self, _vtable_id: &VTableId) -> ApllodbResult<Vec<VRREntry>> {
        unimplemented!()
    }

    // register : 「PK, version」を受け取り、それをそのPKにおける新revisionとして登録し、VRR-IDを発行する。
    fn register(
        &self,
        _version_id: &VersionId,
        _pk: ApparentPrimaryKey,
    ) -> ApllodbResult<VRREntry> {
        unimplemented!()
    }

    // unregister : 「PK」を受け取り、そのPKのレコードを亡き者とする
    fn unregister(&self, _version_id: &VersionId, _pk: &ApparentPrimaryKey) -> ApllodbResult<()> {
        unimplemented!()
    }
}
