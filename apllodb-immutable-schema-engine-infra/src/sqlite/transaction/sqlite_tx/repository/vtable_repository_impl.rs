use std::collections::VecDeque;

use crate::{
    external_interface::ApllodbImmutableSchemaEngine,
    immutable_schema_row_iter::ImmutableSchemaRowIter,
    sqlite::{
        row_iterator::SqliteRowIterator,
        sqlite_rowid::SqliteRowid,
        sqlite_types::SqliteTypes,
        transaction::{
            sqlite_tx::{
                dao::{SqliteMasterDao, VersionDao},
                version_revision_resolver::VersionRevisionResolverImpl,
                SqliteTx,
            },
            VTableDao,
        },
    },
};
use apllodb_immutable_schema_engine_domain::{
    entity::Entity,
    row::column::non_pk_column::column_name::NonPKColumnName,
    row_iter::ImmutableSchemaRowIterator,
    version::active_versions::ActiveVersions,
    version_revision_resolver::vrr_entries::VRREntries,
    version_revision_resolver::VersionRevisionResolver,
    vtable::repository::VTableRepository,
    vtable::{id::VTableId, VTable},
};
use apllodb_shared_components::error::ApllodbResult;

#[derive(Debug)]
pub struct VTableRepositoryImpl<'repo, 'db: 'repo> {
    tx: &'repo SqliteTx<'db>,
}

impl<'repo, 'db: 'repo> VTableRepository<'repo, 'db, ApllodbImmutableSchemaEngine, SqliteTypes>
    for VTableRepositoryImpl<'repo, 'db>
{
    fn new(tx: &'repo SqliteTx<'db>) -> Self {
        Self { tx }
    }

    /// # Failures
    ///
    /// - [DuplicateTable](error/enum.ApllodbErrorKind.html#variant.DuplicateTable) when:
    ///   - Table `table_name` is already visible to this transaction.
    /// - Errors from [TableDao::create()](foobar.html).
    fn create(&self, vtable: &VTable) -> ApllodbResult<()> {
        self.vtable_dao().insert(&vtable)?;
        self.vrr().create_table(&vtable)?;
        Ok(())
    }

    /// # Failures
    ///
    /// - [IoError](error/enum.ApllodbErrorKind.html#variant.IoError) when:
    ///   - rusqlite raises an error.
    /// - [UndefinedTable](error/enum.ApllodbErrorKind.html#variant.UndefinedTable) when:
    ///   - Table `table_name` is not visible to this transaction.
    fn read(&self, vtable_id: &VTableId) -> ApllodbResult<VTable> {
        self.vtable_dao().select(&vtable_id)
    }

    /// # Failures
    ///
    /// - [UndefinedTable](error/enum.ApllodbErrorKind.html#variant.UndefinedTable) when:
    ///   - Table `table_name` is not visible to this transaction.
    /// - [IoError](error/enum.ApllodbErrorKind.html#variant.IoError) when:
    ///   - rusqlite raises an error.
    fn update(&self, _vtable: &VTable) -> ApllodbResult<()> {
        // TODO update VTable on TableWideConstraints change.
        Ok(())
    }

    fn full_scan(
        &self,
        vtable_id: &VTableId,
        projection: &[NonPKColumnName],
    ) -> ApllodbResult<ImmutableSchemaRowIter> {
        let vrr_entries = self.vrr().scan(&vtable_id)?;
        self.probe_vrr_entries(vrr_entries, projection)
    }

    fn probe_vrr_entries(
        &self,
        vrr_entries: VRREntries<'repo, 'db, ApllodbImmutableSchemaEngine, SqliteTypes>,
        projection: &[NonPKColumnName],
    ) -> ApllodbResult<ImmutableSchemaRowIter> {
        let mut ver_row_iters: VecDeque<SqliteRowIterator> = VecDeque::new();

        let vtable = self.vtable_dao().select(&vrr_entries.vtable_id())?;

        for (version_id, vrr_entries) in vrr_entries.group_by_version_id() {
            let version = self
                .sqlite_master_dao()
                .select_active_version(&vtable, &version_id)?;

            let ver_row_iter = self.version_dao().join_with_navi(
                &vtable,
                &version,
                &vrr_entries
                    .map(|e| e.id().clone())
                    .collect::<Vec<SqliteRowid>>(),
                projection,
            )?;
            ver_row_iters.push_back(ver_row_iter);
        }

        Ok(ImmutableSchemaRowIter::chain_versions(ver_row_iters))
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

impl<'repo, 'db: 'repo> VTableRepositoryImpl<'repo, 'db> {
    fn vrr(&self) -> VersionRevisionResolverImpl {
        VersionRevisionResolverImpl::new(self.tx)
    }

    fn vtable_dao(&self) -> VTableDao<'repo, 'db> {
        VTableDao::new(&self.tx)
    }

    fn version_dao(&self) -> VersionDao<'repo, 'db> {
        VersionDao::new(&self.tx)
    }

    fn sqlite_master_dao(&self) -> SqliteMasterDao<'repo, 'db> {
        SqliteMasterDao::new(&self.tx)
    }
}
