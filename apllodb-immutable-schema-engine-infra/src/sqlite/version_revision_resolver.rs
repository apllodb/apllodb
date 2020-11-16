use apllodb_immutable_schema_engine_domain::{
    row::pk::apparent_pk::ApparentPrimaryKey,
    version::id::VersionId,
    version_revision_resolver::VersionRevisionResolver,
    version_revision_resolver::{vrr_entries::VRREntries, vrr_entry::VRREntry},
    vtable::id::VTableId,
};
use apllodb_shared_components::error::ApllodbResult;

use crate::external_interface::ApllodbImmutableSchemaEngine;

use super::sqlite_types::SqliteTypes;

// #[derive(Debug)]
// pub(crate) struct VersionRevisionResolverImpl<'tx, 'db: 'tx> {
//     sqlite_tx: &'tx SqliteTx<'db>,
// }

#[derive(Debug)]
pub(crate) struct VersionRevisionResolverImpl {}

impl<'vrr, 'db: 'vrr> VersionRevisionResolver<'vrr, 'db, ApllodbImmutableSchemaEngine, SqliteTypes>
    for VersionRevisionResolverImpl
{
    fn probe(
        &self,
        _vtable_id: &VTableId,
        _pks: Vec<ApparentPrimaryKey>,
    ) -> ApllodbResult<VRREntries<'vrr, 'db, ApllodbImmutableSchemaEngine, SqliteTypes>> {
        todo!()
    }

    fn scan(
        &self,
        _vtable_id: &VTableId,
    ) -> ApllodbResult<VRREntries<'vrr, 'db, ApllodbImmutableSchemaEngine, SqliteTypes>> {
        todo!()
    }

    fn register(
        &self,
        _version_id: &VersionId,
        _pk: ApparentPrimaryKey,
    ) -> ApllodbResult<VRREntry<'vrr, 'db, ApllodbImmutableSchemaEngine, SqliteTypes>> {
        todo!()
    }

    fn deregister(&self, _version_id: &VersionId, _pk: &ApparentPrimaryKey) -> ApllodbResult<()> {
        todo!()
    }
}

impl VersionRevisionResolverImpl {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
