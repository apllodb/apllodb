use super::{id::VTableId, VTable};
use crate::version::active_versions::ActiveVersions;
use apllodb_shared_components::{data_structure::ColumnName, error::ApllodbResult};
use apllodb_storage_engine_interface::StorageEngine;

pub trait VTableRepository<'repo, 'db: 'repo, Engine: StorageEngine<'repo, 'db>> {
    fn new(tx: &'repo Engine::Tx) -> Self;

    /// Create a new table with VTable.
    /// Do nothing for Version.
    ///
    /// # Failures
    ///
    /// - [DuplicateTable](error/enum.ApllodbErrorKind.html#variant.DuplicateTable) when:
    ///   - Table `table_name` is already visible to this transaction.
    fn create(&self, vtable: &VTable) -> ApllodbResult<()>;

    /// Returns table metadata from buffer the transaction instance owns.
    ///
    /// # Failures
    ///
    /// - [UndefinedTable](error/enum.ApllodbErrorKind.html#variant.UndefinedTable) when:
    ///   - Table specified by `vtable_id` is not visible to this transaction.
    fn read(&self, vtable_id: &VTableId) -> ApllodbResult<VTable>;

    /// Overwrite Table's metadata.
    ///
    /// # Failures
    ///
    /// - [UndefinedTable](error/enum.ApllodbErrorKind.html#variant.UndefinedTable) when:
    ///   - Table specified by `vtable.id` is not visible to this transaction.
    fn update(&self, vtable: &VTable) -> ApllodbResult<()>;

    fn full_scan(
        &self,
        vtable_id: &VTableId,
        non_pk_column_names: &[ColumnName],
    ) -> ApllodbResult<Engine::RowIter>;

    fn delete_all(&self, vtable: &VTable) -> ApllodbResult<()>;

    fn active_versions(&self, vtable: &VTable) -> ApllodbResult<ActiveVersions>;
}
