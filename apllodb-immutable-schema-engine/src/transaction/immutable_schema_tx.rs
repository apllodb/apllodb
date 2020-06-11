use apllodb_shared_components::{data_structure::TableName, error::ApllodbResult};
use apllodb_storage_manager_interface::TxCtxLike;

/// Operations a transaction implementation for Immutable Schema must have.
pub(crate) trait ImmutableSchemaTx: TxCtxLike {
    type Tbl;

    /// Create a new table with Table and its Version v1 the transaction instance owns.
    ///
    /// # Failures
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
    fn alter_table(&self, table: &Self::Tbl) -> ApllodbResult<()>;
}
