use super::{id::VTableId, VTable};
use crate::{
    abstract_types::ImmutableSchemaAbstractTypes, query_result::projection::ProjectionResult,
    row::pk::apparent_pk::ApparentPrimaryKey, version::active_versions::ActiveVersions,
};
use apllodb_shared_components::{ApllodbResult, SchemaIndex, SqlValue};
use apllodb_storage_engine_interface::Rows;
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

    async fn full_scan(&self, vtable: &VTable, projection: ProjectionResult)
        -> ApllodbResult<Rows>;

    /// Simple probe (e.g. `c1 = 777`)
    async fn probe(
        &self,
        vtable: &VTable,
        projection: ProjectionResult,
        probe_index: &SchemaIndex,
        probe_value: &SqlValue,
    ) -> ApllodbResult<Rows>;

    async fn delete(&self, vtable: &VTable, pks: &[ApparentPrimaryKey]) -> ApllodbResult<()>;

    /// Use this function instead of `delete()` for performance.
    async fn delete_all(&self, vtable: &VTable) -> ApllodbResult<()>;

    async fn active_versions(&self, vtable: &VTable) -> ApllodbResult<ActiveVersions>;
}
