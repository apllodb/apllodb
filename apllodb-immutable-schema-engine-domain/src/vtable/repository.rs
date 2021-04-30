use super::{id::VTableId, VTable};
use crate::{
    abstract_types::ImmutableSchemaAbstractTypes, row_projection_result::RowProjectionResult,
    row_selection_plan::RowSelectionPlan, version::active_versions::ActiveVersions,
    version_revision_resolver::vrr_entries::VrrEntries,
};
use apllodb_shared_components::{ApllodbError, ApllodbResult};
use apllodb_storage_engine_interface::{RowSelectionQuery, Rows};
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait VTableRepository<Types: ImmutableSchemaAbstractTypes> {
    /// Create a new table with VTable.
    /// Do nothing for Version.
    ///
    /// # Failures
    ///
    /// - [DuplicateTable](apllodb_shared_components::ApllodbErrorKind::DuplicateTable) when:
    ///   - Table `table_name` is already visible to this transaction.
    async fn create(&self, vtable: &VTable) -> ApllodbResult<()>;

    /// Returns table metadata from buffer the transaction instance owns.
    ///
    /// # Failures
    ///
    /// - [UndefinedTable](apllodb_shared_components::ApllodbErrorKind::UndefinedTable) when:
    ///   - Table specified by `vtable_id` is not visible to this transaction.
    async fn read(&self, vtable_id: &VTableId) -> ApllodbResult<VTable>;

    /// Overwrite Table's metadata.
    ///
    /// # Failures
    ///
    /// - [UndefinedTable](apllodb_shared_components::ApllodbErrorKind::UndefinedTable) when:
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
        selection_plan: &RowSelectionPlan<Types>,
    ) -> ApllodbResult<Rows> {
        let rows = match selection_plan {
            RowSelectionPlan::FullScan => self._full_scan(vtable, projection).await,
            RowSelectionPlan::VrrProbe(_vrr_entries) => Err(ApllodbError::feature_not_supported(
                "SELECT ... WHERE ... in storage engine is not supported currently",
            )),
        }?;
        Ok(rows)
    }

    async fn _full_scan(
        &self,
        vtable: &VTable,
        projection: RowProjectionResult,
    ) -> ApllodbResult<Rows>;

    async fn delete(
        &self,
        vtable: &VTable,
        selection_plan: &RowSelectionPlan<Types>,
    ) -> ApllodbResult<()> {
        match selection_plan {
            RowSelectionPlan::FullScan => self._delete_all(vtable).await?,
            RowSelectionPlan::VrrProbe(vrr_entries) => {
                self._delete_probe(vtable, vrr_entries).await?
            }
        };
        Ok(())
    }

    async fn _delete_all(&self, vtable: &VTable) -> ApllodbResult<()>;

    async fn _delete_probe(
        &self,
        vtable: &VTable,
        vrr_entries: &VrrEntries<Types>,
    ) -> ApllodbResult<()>;

    async fn active_versions(&self, vtable: &VTable) -> ApllodbResult<ActiveVersions>;
}
