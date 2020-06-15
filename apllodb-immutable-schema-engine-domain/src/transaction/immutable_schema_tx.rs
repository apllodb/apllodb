use crate::{
    version::{ActiveVersion, VersionNumber},
    VTable, VersionRowIter,
};
use apllodb_shared_components::{
    data_structure::{ColumnName, DatabaseName, TableName},
    error::ApllodbResult,
    traits::Database,
};
use std::fmt::Debug;

/// Operations a transaction implementation for Immutable Schema must have.
///
/// Meant to be called from implementations of [Transaction](foo.html) (logical transaction interface) internally as physical transaction.
pub trait ImmutableSchemaTx<'db>: Debug {
    type Db: Database + 'db;

    /// Row iterator from a single version.
    type VerRowIter: VersionRowIter;

    fn begin(db: &'db mut Self::Db) -> ApllodbResult<Self>
    where
        Self: Sized;

    fn commit(self) -> ApllodbResult<()>
    where
        Self: Sized;

    fn abort(self) -> ApllodbResult<()>
    where
        Self: Sized;

    fn database_name(&self) -> &DatabaseName;

    /// Create a new table with VTable.
    /// Do nothing for Version.
    ///
    /// # Failures
    ///
    /// - [DuplicateTable](error/enum.ApllodbErrorKind.html#variant.DuplicateTable) when:
    ///   - Table `table_name` is already visible to this transaction.
    fn create_vtable(&self, table: &VTable) -> ApllodbResult<()>;

    /// Returns table metadata from buffer the transaction instance owns.
    ///
    /// # Failures
    ///
    /// - [UndefinedTable](error/enum.ApllodbErrorKind.html#variant.UndefinedTable) when:
    ///   - Table `table_name` is not visible to this transaction.
    fn read_vtable(&self, table_name: &TableName) -> ApllodbResult<VTable>;

    /// Overwrite Table's metadata.
    ///
    /// # Failures
    ///
    /// - [UndefinedTable](error/enum.ApllodbErrorKind.html#variant.UndefinedTable) when:
    ///   - Table `table_name` is not visible to this transaction.
    fn update_vtable(&self, table: &VTable) -> ApllodbResult<()>;

    /// Create a version.
    fn create_version(&self, version: &ActiveVersion) -> ApllodbResult<()>;

    /// Deactivate a version.
    fn deactivate_version(
        &self,
        table: &VTable,
        version_number: &VersionNumber,
    ) -> ApllodbResult<()>;

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
    ) -> ApllodbResult<Self::VerRowIter>;
}
