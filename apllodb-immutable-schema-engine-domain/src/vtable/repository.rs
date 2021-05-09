use super::{id::VTableId, VTable};
use crate::{
    abstract_types::ImmutableSchemaAbstractTypes,
    row_projection_result::RowProjectionResult,
    row_selection_plan::RowSelectionPlan,
    version::active_versions::ActiveVersions,
    version_revision_resolver::{vrr_entries::VrrEntries, VersionRevisionResolver},
};
use apllodb_shared_components::ApllodbResult;
use apllodb_storage_engine_interface::{RowSelectionQuery, Rows};
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait VTableRepository<Types: ImmutableSchemaAbstractTypes> {
    /// Create a new table with VTable.
    /// Do nothing for Version.
    ///
    /// # Failures
    ///
    /// - [DuplicateTable](apllodb_shared_components::SqlState::DuplicateTable) when:
    ///   - Table `table_name` is already visible to this transaction.
    async fn create(&self, vtable: &VTable) -> ApllodbResult<()>;

    /// Returns table metadata from buffer the transaction instance owns.
    ///
    /// # Failures
    ///
    /// - [UndefinedTable](apllodb_shared_components::SqlState::UndefinedTable) when:
    ///   - Table specified by `vtable_id` is not visible to this transaction.
    async fn read(&self, vtable_id: &VTableId) -> ApllodbResult<VTable>;

    /// Overwrite Table's metadata.
    ///
    /// # Failures
    ///
    /// - [UndefinedTable](apllodb_shared_components::SqlState::UndefinedTable) when:
    ///   - Table specified by `vtable.id` is not visible to this transaction.
    async fn update(&self, vtable: &VTable) -> ApllodbResult<()>;

    /// Plans most efficient selection for SELECT/UPDATE/DELETE statements.
    async fn plan_selection(
        &self,
        vtable: &VTable,
        selection_query: RowSelectionQuery,
    ) -> ApllodbResult<RowSelectionPlan<Types>>;

    async fn select(
        &self,
        vtable: &VTable,
        projection: RowProjectionResult,
        selection_plan: RowSelectionPlan<Types>,
    ) -> ApllodbResult<Rows>
    where
        Types: 'async_trait,
    {
        let rows = match selection_plan {
            RowSelectionPlan::FullScan => self._full_scan(vtable, projection).await,
            RowSelectionPlan::VrrProbe(vrr_entries) => {
                self.probe_vrr_entries(vrr_entries, projection).await
            }
        }?;
        Ok(rows)
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

    async fn delete(
        &self,
        vtable: &VTable,
        selection_plan: RowSelectionPlan<Types>,
    ) -> ApllodbResult<()>
    where
        Types: 'async_trait,
    {
        match selection_plan {
            RowSelectionPlan::FullScan => self.vrr().deregister_all(vtable).await?,
            RowSelectionPlan::VrrProbe(vrr_entries) => {
                self.vrr().deregister(vtable, vrr_entries).await?
            }
        };
        Ok(())
    }

    async fn active_versions(&self, vtable: &VTable) -> ApllodbResult<ActiveVersions>;

    async fn probe_vrr_entries(
        &self,
        vrr_entries: VrrEntries<Types>,
        projection: RowProjectionResult,
    ) -> ApllodbResult<Rows>;

    fn vrr(&self) -> Types::Vrr;
}
