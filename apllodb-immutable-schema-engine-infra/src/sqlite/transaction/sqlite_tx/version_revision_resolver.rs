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
use apllodb_shared_components::{ApllodbError, ApllodbErrorKind, ApllodbResult};

use crate::{
    external_interface::ApllodbImmutableSchemaEngine,
    sqlite::sqlite_types::{SqliteTypes, VRREntries, VRREntry},
};

use self::navi_dao::{navi::Navi, NaviDao};

use super::SqliteTx;

#[derive(Debug)]
pub(crate) struct VersionRevisionResolverImpl<'vrr, 'sqcn: 'vrr> {
    tx: &'vrr SqliteTx<'sqcn>,
}

impl<'vrr, 'sqcn: 'vrr>
    VersionRevisionResolver<ApllodbImmutableSchemaEngine<'sqcn>, SqliteTypes<'vrr, 'sqcn>>
    for VersionRevisionResolverImpl<'vrr, 'sqcn>
{
    fn create_table(&self, vtable: &VTable) -> ApllodbResult<()> {
        self.navi_dao().create_table(vtable)
    }

    /// Every PK column is included in resulting row although it is not specified in `projection`.
    ///
    /// FIXME Exclude unnecessary PK column in resulting row for performance.
    fn probe(
        &self,
        vtable_id: &VTableId,
        pks: Vec<ApparentPrimaryKey>,
    ) -> ApllodbResult<VRREntries<'vrr, 'sqcn>> {
        let mut entries = VecDeque::<VRREntry>::new();

        for pk in pks {
            // FIXME solve N+1 problem
            let navi = self.navi_dao().probe_latest_revision(vtable_id, &pk)?;
            if let Navi::Exist(existing_navi) = navi {
                let version_id = VersionId::new(&vtable_id, &existing_navi.version_number);
                let entry = VRREntry::new(
                    existing_navi.rowid.clone(),
                    pk,
                    version_id,
                    existing_navi.revision.clone(),
                );
                entries.push_back(entry);
            }
        }
        Ok(VRREntries::new(vtable_id.clone(), entries))
    }

    /// Every PK column is included in resulting row although it is not specified in `projection`.
    ///
    /// FIXME Exclude unnecessary PK column in resulting row for performance.
    fn scan(&self, vtable: &VTable) -> ApllodbResult<VRREntries<'vrr, 'sqcn>> {
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
    ) -> ApllodbResult<VRREntry<'vrr, 'sqcn>> {
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

    fn deregister(&self, _vtable_id: &VTableId, _pks: &[ApparentPrimaryKey]) -> ApllodbResult<()> {
        todo!()
    }

    fn deregister_all(&self, vtable: &VTable) -> ApllodbResult<()> {
        self.navi_dao().insert_deleted_records_all(vtable)
    }
}

impl<'vrr, 'sqcn: 'vrr> VersionRevisionResolverImpl<'vrr, 'sqcn> {
    pub(crate) fn new(tx: &'vrr SqliteTx<'sqcn>) -> Self {
        Self { tx }
    }

    fn navi_dao(&self) -> NaviDao<'vrr, 'sqcn> {
        NaviDao::new(&self.tx)
    }
}
