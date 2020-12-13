mod navi_dao;

use std::collections::VecDeque;

use apllodb_immutable_schema_engine_domain::{
    entity::Entity,
    row::pk::{apparent_pk::ApparentPrimaryKey, full_pk::revision::Revision},
    version::id::VersionId,
    version_revision_resolver::VersionRevisionResolver,
    vtable::id::VTableId,
    vtable::VTable,
};
use apllodb_shared_components::error::{ApllodbError, ApllodbErrorKind, ApllodbResult};

use crate::{
    external_interface::ApllodbImmutableSchemaEngine,
    sqlite::sqlite_types::{SqliteTypes, VRREntries, VRREntry},
};

use self::navi_dao::{Navi, NaviDao};

use super::SqliteTx;

#[derive(Debug)]
pub(crate) struct VersionRevisionResolverImpl<'vrr, 'db: 'vrr> {
    tx: &'vrr SqliteTx<'db>,
}

impl<'vrr, 'db: 'vrr> VersionRevisionResolver<'vrr, 'db, ApllodbImmutableSchemaEngine, SqliteTypes>
    for VersionRevisionResolverImpl<'vrr, 'db>
{
    fn create_table(&self, vtable: &VTable) -> ApllodbResult<()> {
        self.navi_dao().create_table(vtable)
    }

    fn probe(
        &self,
        vtable_id: &VTableId,
        pks: Vec<ApparentPrimaryKey>,
    ) -> ApllodbResult<VRREntries<'vrr, 'db>> {
        // TODO solve N+1 problem
        let mut entries = VecDeque::<VRREntry>::new();

        for pk in pks {
            let navi = self.navi_dao().probe_latest_revision(vtable_id, &pk)?;
            match navi {
                Navi::Exist(existing_navi) => {
                    let version_id = VersionId::new(&vtable_id, &existing_navi.version_number);
                    let entry = VRREntry::new(
                        existing_navi.rowid.clone(),
                        pk,
                        version_id,
                        existing_navi.revision.clone(),
                    );
                    entries.push_back(entry);
                }
                _ => {}
            }
        }
        Ok(VRREntries::new(vtable_id.clone(), entries))
    }

    fn scan(&self, vtable: &VTable) -> ApllodbResult<VRREntries<'vrr, 'db>> {
        let mut entries = VecDeque::<VRREntry>::new();

        for navi in self.navi_dao().full_scan_latest_revision(vtable)? {
            let version_id = VersionId::new(&vtable.id(), &navi.navi.version_number);
            let entry = VRREntry::new(
                navi.navi.rowid.clone(),
                navi.pk,
                version_id,
                navi.navi.revision.clone(),
            );
            entries.push_back(entry);
        }
        Ok(VRREntries::new(vtable.id().clone(), entries))
    }

    fn register(
        &self,
        version_id: &VersionId,
        pk: ApparentPrimaryKey,
    ) -> ApllodbResult<VRREntry<'vrr, 'db>> {
        let revision = match self
            .navi_dao()
            .probe_latest_revision(version_id.vtable_id(), &pk)?
        {
            Navi::Exist { .. } => Err(ApllodbError::new(
                ApllodbErrorKind::UniqueViolation,
                format!("record with the same primary key already exists: {:?}", pk),
                None,
            )),
            Navi::NotExist => Ok(Revision::initial()),
            Navi::Deleted { revision, .. } => Ok(revision.next()),
        }?;

        let rowid = self.navi_dao().insert(&pk, &revision, &version_id)?;
        Ok(VRREntry::new(rowid, pk, version_id.clone(), revision))
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
