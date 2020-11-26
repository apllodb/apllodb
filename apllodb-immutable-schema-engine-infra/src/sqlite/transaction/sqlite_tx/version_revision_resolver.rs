mod navi_dao;

use apllodb_immutable_schema_engine_domain::{
    row::pk::apparent_pk::ApparentPrimaryKey, version::id::VersionId,
    version_revision_resolver::VersionRevisionResolver, vtable::id::VTableId, vtable::VTable,
};
use apllodb_shared_components::error::ApllodbResult;

use crate::{
    external_interface::ApllodbImmutableSchemaEngine,
    sqlite::sqlite_types::{SqliteTypes, VRREntries, VRREntry},
};

use self::navi_dao::NaviDao;

use super::SqliteTx;

// #[derive(Debug)]
// pub(crate) struct VersionRevisionResolverImpl<'tx, 'db: 'tx> {
//     sqlite_tx: &'tx SqliteTx<'db>,
// }

#[derive(Debug)]
pub(crate) struct VersionRevisionResolverImpl<'vrr, 'db: 'vrr> {
    tx: &'vrr SqliteTx<'db>,
}

impl<'vrr, 'db: 'vrr> VersionRevisionResolver<'vrr, 'db, ApllodbImmutableSchemaEngine, SqliteTypes>
    for VersionRevisionResolverImpl<'vrr, 'db>
{
    fn create_table(&self, vtable: &VTable) -> ApllodbResult<()> {
        todo!()
    }

    fn probe(
        &self,
        _vtable_id: &VTableId,
        _pks: Vec<ApparentPrimaryKey>,
    ) -> ApllodbResult<VRREntries<'vrr, 'db>> {
        todo!()
    }

    fn scan(&self, _vtable_id: &VTableId) -> ApllodbResult<VRREntries<'vrr, 'db>> {
        todo!()
    }

    fn register(
        &self,
        _version_id: &VersionId,
        _pk: ApparentPrimaryKey,
    ) -> ApllodbResult<VRREntry<'vrr, 'db>> {
        // let revision = match self
        //     .navi_dao()
        //     .probe_latest_revision(version_id.vtable_id(), &apparent_pk)?
        // {
        //     Navi::Exist { .. } => Err(ApllodbError::new(
        //         ApllodbErrorKind::UniqueViolation,
        //         format!(
        //             "record with the same primary key already exists: {:?}",
        //             apparent_pk
        //         ),
        //         None,
        //     )),
        //     Navi::NotExist => Ok(Revision::initial()),
        //     Navi::Deleted { revision, .. } => Ok(revision.next()),
        // }?;

        // let rowid = self.navi_dao().insert(apparent_pk, revision, &version_id)?;

        todo!()
    }

    fn deregister(&self, _vtable_id: &VTableId, _pk: &ApparentPrimaryKey) -> ApllodbResult<()> {
        todo!()
    }

    fn deregister_all(&self, vtable: &VTable) -> ApllodbResult<()> {
        self.navi_dao().insert_deleted_records_all(vtable)
    }
}

impl<'vrr, 'db: 'vrr> VersionRevisionResolverImpl<'vrr, 'db> {
    pub(crate) fn new(tx: &'vrr SqliteTx<'db>) -> Self {
        Self { tx }
    }

    fn navi_dao(&self) -> NaviDao<'vrr, 'db> {
        NaviDao::new(&self.tx)
    }
}
