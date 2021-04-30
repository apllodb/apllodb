use std::{cell::RefCell, rc::Rc};

use super::vtable_metadata_dao::VTableMetadataDao;
use crate::sqlite::{
    rows::chain_rows::ChainRows,
    sqlite_types::{RowSelectionPlan, SqliteTypes, VrrEntries},
    transaction::sqlite_tx::{
        version::dao::{version_dao::VersionDao, version_metadata_dao::VersionMetadataDao},
        version_revision_resolver::VersionRevisionResolverImpl,
        SqliteTx,
    },
};
use apllodb_immutable_schema_engine_domain::{
    entity::Entity,
    row_projection_result::RowProjectionResult,
    version::active_versions::ActiveVersions,
    version_revision_resolver::VersionRevisionResolver,
    vtable::repository::VTableRepository,
    vtable::{id::VTableId, VTable},
};
use apllodb_shared_components::{ApllodbError, ApllodbResult};
use apllodb_storage_engine_interface::{
    Row, RowSchema, RowSelectionQuery, Rows, SingleTableCondition,
};
use async_trait::async_trait;

#[derive(Debug)]
pub struct VTableRepositoryImpl {
    tx: Rc<RefCell<SqliteTx>>,
}

impl VTableRepositoryImpl {
    pub(crate) fn new(tx: Rc<RefCell<SqliteTx>>) -> Self {
        Self { tx }
    }
}

#[async_trait(?Send)]
impl VTableRepository<SqliteTypes> for VTableRepositoryImpl {
    /// # Failures
    ///
    /// - [DuplicateTable](apllodb_shared_components::ApllodbErrorKind::DuplicateTable) when:
    ///   - Table `table_name` is already visible to this transaction.
    /// - Errors from [TableDao::create()](foobar.html).
    async fn create(&self, vtable: &VTable) -> ApllodbResult<()> {
        self.vtable_metadata_dao().insert(&vtable).await?;
        self.vrr().create_table(&vtable).await?;
        Ok(())
    }

    /// # Failures
    ///
    /// - [IoError](apllodb_shared_components::ApllodbErrorKind::IoError) when:
    ///   - sqlx raises an error.
    /// - [UndefinedTable](apllodb_shared_components::ApllodbErrorKind::UndefinedTable) when:
    ///   - Table `table_name` is not visible to this transaction.
    async fn read(&self, vtable_id: &VTableId) -> ApllodbResult<VTable> {
        self.vtable_metadata_dao().select(&vtable_id).await
    }

    /// # Failures
    ///
    /// - [UndefinedTable](apllodb_shared_components::ApllodbErrorKind::UndefinedTable) when:
    ///   - Table `table_name` is not visible to this transaction.
    /// - [IoError](apllodb_shared_components::ApllodbErrorKind::IoError) when:
    ///   - sqlx raises an error.
    async fn update(&self, _vtable: &VTable) -> ApllodbResult<()> {
        // TODO update VTable on TableWideConstraints change.
        Ok(())
    }

    async fn plan_selection(
        &self,
        vtable: &VTable,
        selection_query: RowSelectionQuery,
    ) -> ApllodbResult<RowSelectionPlan> {
        match selection_query {
            RowSelectionQuery::FullScan => Ok(RowSelectionPlan::FullScan),
            RowSelectionQuery::Condition(c) => self.plan_selection_from_condition(vtable, c).await,
        }
    }

    /// Every PK column is included in resulting row although it is not specified in `projection`.
    ///
    /// FIXME Exclude unnecessary PK column in resulting row for performance.
    async fn _full_scan(
        &self,
        vtable: &VTable,
        projection: RowProjectionResult,
    ) -> ApllodbResult<Rows> {
        let vrr_entries = self.vrr().scan(&vtable).await?;
        self.probe_vrr_entries(vrr_entries, projection).await
    }

    async fn _delete_all(&self, vtable: &VTable) -> ApllodbResult<()> {
        self.vrr().deregister_all(vtable).await?;
        Ok(())
    }

    async fn _delete_probe(
        &self,
        _vtable: &VTable,
        _vrr_entries: &VrrEntries,
    ) -> ApllodbResult<()> {
        Err(ApllodbError::feature_not_supported(
            "DELETE ... WHERE ... is not supported currently",
        ))
    }

    async fn active_versions(&self, vtable: &VTable) -> ApllodbResult<ActiveVersions> {
        let active_versions = self
            .version_metadata_dao()
            .select_active_versions(vtable.id())
            .await?;
        Ok(ActiveVersions::from(active_versions))
    }
}

impl VTableRepositoryImpl {
    fn vrr(&self) -> VersionRevisionResolverImpl {
        VersionRevisionResolverImpl::new(self.tx.clone())
    }

    fn vtable_metadata_dao(&self) -> VTableMetadataDao {
        VTableMetadataDao::new(self.tx.clone())
    }

    fn version_dao(&self) -> VersionDao {
        VersionDao::new(self.tx.clone())
    }

    fn version_metadata_dao(&self) -> VersionMetadataDao {
        VersionMetadataDao::new(self.tx.clone())
    }

    async fn probe_vrr_entries(
        &self,
        vrr_entries: VrrEntries,
        projection: RowProjectionResult,
    ) -> ApllodbResult<Rows> {
        let vtable = self
            .vtable_metadata_dao()
            .select(&vrr_entries.vtable_id())
            .await?;

        let mut all_ver_rows = Vec::<Rows>::new();

        for vrr_entries_in_version in vrr_entries.group_by_version_id() {
            let version = self
                .version_metadata_dao()
                .select_active_version(&vtable.id(), vrr_entries_in_version.version_id())
                .await?;

            let ver_rows = self
                .version_dao()
                .probe_in_version(&version, vrr_entries_in_version, &projection)
                .await?;

            all_ver_rows.push(ver_rows);
        }

        if all_ver_rows.is_empty() {
            Ok(Rows::new(RowSchema::from(projection), Vec::<Row>::new()))
        } else {
            let rows = ChainRows::chain(all_ver_rows);
            Ok(rows)
        }
    }

    async fn plan_selection_from_condition(
        &self,
        _vtable: &VTable,
        _condition: SingleTableCondition,
    ) -> ApllodbResult<RowSelectionPlan> {
        Err(ApllodbError::feature_not_supported(
            "storage-engine selection is not supported currently",
        ))
    }
}
