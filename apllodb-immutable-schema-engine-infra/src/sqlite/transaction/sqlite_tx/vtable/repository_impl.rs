use std::collections::VecDeque;

use crate::{
    external_interface::ApllodbImmutableSchemaEngine,
    immutable_schema_row_iter::ImmutableSchemaRowIter,
    sqlite::{
        row_iterator::SqliteRowIterator,
        sqlite_types::{SqliteTypes, VRREntries},
        transaction::sqlite_tx::version::dao::VersionDao,
        transaction::sqlite_tx::{
            version_revision_resolver::VersionRevisionResolverImpl, SqliteTx,
        },
    },
};
use apllodb_immutable_schema_engine_domain::{
    query::projection::ProjectionResult,
    row_iter::ImmutableSchemaRowIterator,
    version::active_versions::ActiveVersions,
    version_revision_resolver::VersionRevisionResolver,
    vtable::repository::VTableRepository,
    vtable::{id::VTableId, VTable},
};
use apllodb_shared_components::ApllodbResult;

use super::{dao::VTableDao, sqlite_master::dao::SqliteMasterDao};

#[derive(Debug)]
pub struct VTableRepositoryImpl<'repo, 'sess: 'repo> {
    tx: &'repo SqliteTx<'sess>,
}

impl<'repo, 'sess> VTableRepositoryImpl<'repo, 'sess> {
    pub fn new(tx: &'repo SqliteTx<'sess>) -> Self {
        Self { tx }
    }
}

impl<'repo, 'sess: 'repo> VTableRepository<ApllodbImmutableSchemaEngine<'sess>, SqliteTypes<'repo, 'sess>>
    for VTableRepositoryImpl<'repo, 'sess>
{
    /// # Failures
    ///
    /// - [DuplicateTable](apllodb_shared_components::ApllodbErrorKind::DuplicateTable) when:
    ///   - Table `table_name` is already visible to this transaction.
    /// - Errors from [TableDao::create()](foobar.html).
    fn create(&self, vtable: &VTable) -> ApllodbResult<()> {
        self.vtable_dao().insert(&vtable)?;
        self.vrr().create_table(&vtable)?;
        Ok(())
    }

    /// # Failures
    ///
    /// - [IoError](apllodb_shared_components::ApllodbErrorKind::IoError) when:
    ///   - rusqlite raises an error.
    /// - [UndefinedTable](apllodb_shared_components::ApllodbErrorKind::UndefinedTable) when:
    ///   - Table `table_name` is not visible to this transaction.
    fn read(&self, vtable_id: &VTableId) -> ApllodbResult<VTable> {
        self.vtable_dao().select(&vtable_id)
    }

    /// # Failures
    ///
    /// - [UndefinedTable](apllodb_shared_components::ApllodbErrorKind::UndefinedTable) when:
    ///   - Table `table_name` is not visible to this transaction.
    /// - [IoError](apllodb_shared_components::ApllodbErrorKind::IoError) when:
    ///   - rusqlite raises an error.
    fn update(&self, _vtable: &VTable) -> ApllodbResult<()> {
        // TODO update VTable on TableWideConstraints change.
        Ok(())
    }

    /// Every PK column is included in resulting row although it is not specified in `projection`.
    ///
    /// FIXME Exclude unnecessary PK column in resulting row for performance.
    fn full_scan(
        &self,
        vtable: &VTable,
        projection: ProjectionResult,
    ) -> ApllodbResult<ImmutableSchemaRowIter> {
        let vrr_entries = self.vrr().scan(&vtable)?;
        self.probe_vrr_entries(vrr_entries, projection)
    }

    fn delete_all(&self, vtable: &VTable) -> ApllodbResult<()> {
        self.vrr().deregister_all(vtable)?;
        Ok(())
    }

    fn active_versions(&self, vtable: &VTable) -> ApllodbResult<ActiveVersions> {
        let active_versions = self.sqlite_master_dao().select_active_versions(vtable)?;
        Ok(ActiveVersions::from(active_versions))
    }
}

impl<'repo, 'sess: 'repo> VTableRepositoryImpl<'repo, 'sess> {
    fn vrr(&self) -> VersionRevisionResolverImpl {
        VersionRevisionResolverImpl::new(self.tx)
    }

    fn vtable_dao(&self) -> VTableDao<'repo, 'sess> {
        VTableDao::new(&self.tx)
    }

    fn version_dao(&self) -> VersionDao<'repo, 'sess> {
        VersionDao::new(&self.tx)
    }

    fn sqlite_master_dao(&self) -> SqliteMasterDao<'repo, 'sess> {
        SqliteMasterDao::new(&self.tx)
    }

    fn probe_vrr_entries(
        &self,
        vrr_entries: VRREntries<'_, 'sess>,
        projection: ProjectionResult,
    ) -> ApllodbResult<ImmutableSchemaRowIter> {
        let mut ver_row_iters: VecDeque<SqliteRowIterator> = VecDeque::new();

        let vtable = self.vtable_dao().select(&vrr_entries.vtable_id())?;

        for vrr_entries_in_version in vrr_entries.group_by_version_id() {
            let version = self
                .sqlite_master_dao()
                .select_active_version(&vtable, vrr_entries_in_version.version_id())?;

            let ver_row_iter = self.version_dao().probe_in_version(
                &version,
                vrr_entries_in_version,
                &projection,
            )?;
            ver_row_iters.push_back(ver_row_iter);
        }

        Ok(ImmutableSchemaRowIter::chain_versions(ver_row_iters))
    }
}
