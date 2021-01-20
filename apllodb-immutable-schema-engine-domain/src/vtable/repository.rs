use super::{id::VTableId, VTable};
use crate::{
    abstract_types::ImmutableSchemaAbstractTypes, query::projection::ProjectionResult,
    version::active_versions::ActiveVersions,
};
use apllodb_shared_components::ApllodbResult;
use apllodb_storage_engine_interface::StorageEngine;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait VTableRepository<Engine: StorageEngine, Types: ImmutableSchemaAbstractTypes<Engine>> {
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

    async fn full_scan(
        &self,
        vtable: &VTable,
        projection: ProjectionResult,
    ) -> ApllodbResult<Types::ImmutableSchemaRowIter>;

    async fn delete_all(&self, vtable: &VTable) -> ApllodbResult<()>;

    async fn active_versions(&self, vtable: &VTable) -> ApllodbResult<ActiveVersions>;
}
