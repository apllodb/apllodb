use apllodb_immutable_schema_engine_domain::{
    row::pk::apparent_pk::ApparentPrimaryKey, version::id::VersionId,
    version_revision_resolver::vrr_entry::VRREntry,
    version_revision_resolver::VersionRevisionResolver, vtable::id::VTableId,
};
use apllodb_shared_components::error::ApllodbResult;

use super::transaction::sqlite_tx::SqliteTx;

// #[derive(Debug)]
// pub(crate) struct VersionRevisionResolverImpl<'tx, 'db: 'tx> {
//     sqlite_tx: &'tx SqliteTx<'db>,
// }

#[derive(Debug)]
pub(crate) struct VersionRevisionResolverImpl {}

impl<'tx, 'db: 'tx> VersionRevisionResolver<'tx, 'db> for VersionRevisionResolverImpl {
    type Tx = SqliteTx<'db>;

    fn probe(
        &self,
        vtable_id: &VTableId,
        pks: Vec<ApparentPrimaryKey>,
    ) -> ApllodbResult<Vec<VRREntry>> {
        todo!()
    }

    fn scan(&self, vtable_id: &VTableId) -> ApllodbResult<Vec<VRREntry>> {
        todo!()
    }

    fn register(&self, version_id: &VersionId, pk: ApparentPrimaryKey) -> ApllodbResult<VRREntry> {
        todo!()
    }

    fn unregister(&self, version_id: &VersionId, pk: &ApparentPrimaryKey) -> ApllodbResult<()> {
        todo!()
    }
}

impl VersionRevisionResolverImpl {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
