use crate::Table;
use apllodb_shared_components::data_structure::{
    AlterTableAction, ColumnDefinition, TableConstraints, TableName,
};
use apllodb_shared_components::error::ApllodbResult;

/// Access methods for DDL.
///
/// A storage engine must implement interface functions.
pub trait AccessMethodsDdl {
    // TODO async とかつけような

    /// CREATE TABLE command.
    ///
    /// # Failures
    ///
    /// - Errors from [Table::new](foobar.html).
    /// - Errors from [ActiveVersion::create_initial](foobar.html).
    /// - Errors from [materialize_table](method.materialize_table.html).
    fn create_table(
        table_name: &TableName,
        table_constraints: &TableConstraints,
        column_definitions: &[ColumnDefinition],
    ) -> ApllodbResult<()> {
        let table = Table::create(table_name, table_constraints, column_definitions)?;

        Self::materialize_table(table)?;

        Ok(())
    }

    /// ALTER TABLE command.
    ///
    /// This function executes the following steps:
    ///
    /// 1. Dematerialize `v_current`.
    /// 1. Create `v_(current+1)`.
    /// 1. Auto-upgrade.
    /// 1. Inactivate `v_i` `(i <= current)` if all of `v_i`'s records are DELETEd.
    ///
    /// # Failures
    ///
    fn alter_table(table_name: &TableName, action: &AlterTableAction) -> ApllodbResult<()> {
        // TODO transaction (lock)

        let mut table = Self::dematerialize_table(&TableName::from(table_name.clone()))?;
        table.alter(action)?;
        Self::materialize_table(table)?;

        Ok(())
    }

    /// DROP TABLE command.
    ///
    /// # Panics
    ///
    /// # Failures
    ///
    /// # Safety
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    fn drop_table() -> ApllodbResult<()>;

    fn materialize_table(table: Table) -> ApllodbResult<()>;

    fn dematerialize_table(name: &TableName) -> ApllodbResult<Table>;
}
