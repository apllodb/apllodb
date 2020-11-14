use apllodb_immutable_schema_engine_domain::{
    row::pk::apparent_pk::ApparentPrimaryKey, version::id::VersionId,
    version_revision_resolver::vrr_entry::VRREntry,
    version_revision_resolver::VersionRevisionResolver, vtable::id::VTableId,
};
use apllodb_shared_components::error::ApllodbResult;

// #[derive(Debug)]
// pub(crate) struct VersionRevisionResolverImpl<'tx, 'db: 'tx> {
//     sqlite_tx: &'tx SqliteTx<'db>,
// }

#[derive(Debug)]
pub(crate) struct VersionRevisionResolverImpl {}

impl VersionRevisionResolver for VersionRevisionResolverImpl {
    fn probe(
        &self,
        _vtable_id: &VTableId,
        _pks: Vec<ApparentPrimaryKey>,
    ) -> ApllodbResult<Vec<VRREntry>> {
        todo!()
    }

    fn scan(&self, _vtable_id: &VTableId) -> ApllodbResult<Vec<VRREntry>> {
        todo!()
    }

    fn register(
        &self,
        _version_id: &VersionId,
        _pk: ApparentPrimaryKey,
    ) -> ApllodbResult<VRREntry> {
        todo!()
    }

    fn unregister(&self, _version_id: &VersionId, _pk: &ApparentPrimaryKey) -> ApllodbResult<()> {
        todo!()
    }
}

impl VersionRevisionResolverImpl {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
