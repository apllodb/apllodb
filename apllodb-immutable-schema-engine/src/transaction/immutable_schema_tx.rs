use crate::{
    row::ImmutableSchemaRowIter,
    version::{ActiveVersion, VersionNumber},
};
use apllodb_shared_components::{
    data_structure::{ColumnName, TableName},
    error::ApllodbResult,
};
use apllodb_storage_manager_interface::TxCtxLike;

/// Operations a transaction implementation for Immutable Schema must have.
/// Only has primitive operations.
pub(crate) trait ImmutableSchemaTx: TxCtxLike {
    /// Resolve [Table](foobar.html)'s lifetime in concrete implementation.
    type Tbl;

    type RowIter: ImmutableSchemaRowIter;

    /// Create a new table with Table.
    /// Do nothing for Version.
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
    fn read_table(&self, table_name: &TableName) -> ApllodbResult<Self::Tbl>;

    /// Overwrite Table's metadata.
    ///
    /// # Failures
    ///
    /// - [UndefinedTable](error/enum.ApllodbErrorKind.html#variant.UndefinedTable) when:
    ///   - Table `table_name` is not visible to this transaction.
    fn update_table(&self, table: &Self::Tbl) -> ApllodbResult<()>;

    fn create_version(&self, table: &Self::Tbl, version: &ActiveVersion) -> ApllodbResult<()>;

    fn deactivate_version(&self, table: &Self::Tbl, version_number: &VersionNumber);

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
        version: &ActiveVersion,
        column_names: &[ColumnName],
    ) -> ApllodbResult<Self::RowIter>;
}
