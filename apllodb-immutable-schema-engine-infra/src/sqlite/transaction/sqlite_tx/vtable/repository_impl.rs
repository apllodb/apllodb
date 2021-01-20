use std::{cell::RefCell, collections::VecDeque};

use super::{dao::VTableDao, sqlite_master::dao::SqliteMasterDao};
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
use async_trait::async_trait;

#[derive(Debug)]
pub struct VTableRepositoryImpl<'sqcn> {
    // internal sqlx::Transaction implements `Send` but I suspect it is really safe to send SQLite's transaction
    // to another thread.
    // Fortunately, immutable-schema-engine will remove dependency to SQLite in the future so for now I use RefCell and choose not to use multi-threading model.
    tx: RefCell<SqliteTx<'sqcn>>,
}

impl<'sqcn> VTableRepositoryImpl<'sqcn> {
    pub fn new(tx: RefCell<SqliteTx<'sqcn>>) -> Self {
        Self { tx }
    }
}

#[async_trait(?Send)]
impl<'sqcn> VTableRepository<SqliteTypes<'sqcn>> for VTableRepositoryImpl<'sqcn> {
    /// # Failures
    ///
    /// - [DuplicateTable](apllodb_shared_components::ApllodbErrorKind::DuplicateTable) when:
    ///   - Table `table_name` is already visible to this transaction.
    /// - Errors from [TableDao::create()](foobar.html).
    async fn create(&self, vtable: &VTable) -> ApllodbResult<()> {
        self.vtable_dao().insert(&vtable)?;
        self.vrr().create_table(&vtable).await?;
        Ok(())
    }

    /// # Failures
    ///
    /// - [IoError](apllodb_shared_components::ApllodbErrorKind::IoError) when:
    ///   - rusqlite raises an error.
    /// - [UndefinedTable](apllodb_shared_components::ApllodbErrorKind::UndefinedTable) when:
    ///   - Table `table_name` is not visible to this transaction.
    async fn read(&self, vtable_id: &VTableId) -> ApllodbResult<VTable> {
        self.vtable_dao().select(&vtable_id)
    }

    /// # Failures
    ///
    /// - [UndefinedTable](apllodb_shared_components::ApllodbErrorKind::UndefinedTable) when:
    ///   - Table `table_name` is not visible to this transaction.
    /// - [IoError](apllodb_shared_components::ApllodbErrorKind::IoError) when:
    ///   - rusqlite raises an error.
    async fn update(&self, _vtable: &VTable) -> ApllodbResult<()> {
        // TODO update VTable on TableWideConstraints change.
        Ok(())
    }

    /// Every PK column is included in resulting row although it is not specified in `projection`.
    ///
    /// FIXME Exclude unnecessary PK column in resulting row for performance.
    async fn full_scan(
        &self,
        vtable: &VTable,
        projection: ProjectionResult,
    ) -> ApllodbResult<ImmutableSchemaRowIter> {
        let vrr_entries = self.vrr().scan(&vtable).await?;
        self.probe_vrr_entries(vrr_entries, projection)
    }

    async fn delete_all(&self, vtable: &VTable) -> ApllodbResult<()> {
        self.vrr().deregister_all(vtable).await?;
        Ok(())
    }

    async fn active_versions(&self, vtable: &VTable) -> ApllodbResult<ActiveVersions> {
        let active_versions = self.sqlite_master_dao().select_active_versions(vtable)?;
        Ok(ActiveVersions::from(active_versions))
    }
}

impl<'sqcn> VTableRepositoryImpl<'sqcn> {
    fn vrr(&self) -> VersionRevisionResolverImpl {
        VersionRevisionResolverImpl::new(self.tx)
    }

    fn vtable_dao(&self) -> VTableDao<'sqcn> {
        VTableDao::new(&self.tx)
    }

    fn version_dao(&self) -> VersionDao<'sqcn> {
        VersionDao::new(&self.tx)
    }

    fn sqlite_master_dao(&self) -> SqliteMasterDao<'sqcn> {
        SqliteMasterDao::new(&self.tx)
    }

    fn probe_vrr_entries(
        &self,
        vrr_entries: VRREntries<'sqcn>,
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
