use apllodb_shared_components::{
    data_structure::{ColumnName, TableName},
    error::ApllodbResult,
};
use apllodb_storage_manager_interface::TxCtxLike;

/// Operations a transaction implementation for Immutable Schema must have.
pub(crate) trait ImmutableSchemaTx: TxCtxLike {
    type Tbl;
    type Ver;
    type RowIter: Iterator;

    /// Create a new table with Table and its Version v1 the transaction instance owns.
    ///
    /// # Failures
    ///
    /// - [DuplicateTable](error/enum.ApllodbErrorKind.html#variant.DuplicateTable) when:
    ///   - Table `table_name` is already visible to this transaction.
    fn create_table(&self, table: &Self::Tbl) -> ApllodbResult<()>;

    /// Returns table metadata from buffer the transaction instance owns.
    ///
    /// # Failures
    ///
    /// - [UndefinedTable](error/enum.ApllodbErrorKind.html#variant.UndefinedTable) when:
    ///   - Table `table_name` is not visible to this transaction.
    fn get_table(&self, table_name: &TableName) -> ApllodbResult<Self::Tbl>;

    /// Do the following:
    ///
    /// - Overwrite Table's metadata.
    /// - Create new Version.
    /// - Auto-upgrade.
    /// - Deactivate Version's.
    ///
    /// # Failures
    ///
    /// - [UndefinedTable](error/enum.ApllodbErrorKind.html#variant.UndefinedTable) when:
    ///   - Table `table_name` is not visible to this transaction.
    fn alter_table(&self, table: &Self::Tbl) -> ApllodbResult<()>;

    /// Scan version.
    ///
    /// - Resolves each column's ColumnDataType from active versions.
    /// - Issue SELECT to `version` and get rows.
    ///
    /// # Failures
    ///
    /// - [UndefinedColumn](error/enum.ApllodbErrorKind.html#variant.UndefinedColumn) when:
    ///   - At least one `column_names` are not included in this `version`.
    fn full_scan(
        &self,
        version: &Self::Ver,
        column_names: &[ColumnName],
    ) -> ApllodbResult<Self::RowIter>;
}
