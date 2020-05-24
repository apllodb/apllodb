use crate::{ActiveVersion, Table};
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
    /// - Errors from [materialize_version](method.materialize_version.html).
    fn create_table(
        table_name: &TableName,
        table_constraints: &TableConstraints,
        column_definitions: &[ColumnDefinition],
    ) -> ApllodbResult<()> {
        let table = Table::new(table_name, table_constraints, column_definitions)?;
        let version = ActiveVersion::create_initial(column_definitions, table_constraints)?;

        Self::materialize_table(table)?;
        Self::materialize_version(version)?;

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
    fn alter_table(_table_name: &TableName, _action: &AlterTableAction) -> ApllodbResult<()> {
        todo!();

        // TODO transaction (lock)

        // let table = Self::dematerialize_table(&TableName::from(table_name.clone()))?;
        // let current_version_num = table.current_version_number();
        // let current_version = Self::dematerialize_active_version(current_version_num)?;

        // let alter_table_action = AlterTableAction::from(action);
        // let next_version_action = NextVersionAction::from(action);

        // table.alter(alter_table_action)?;
        // let next_version = current_version.create_next(next_version_action)?;

        // // TODO auto-upgrade.
        // // TODO Inactivate old empty versions.

        // Self::materialize_table(table)?;
        // Self::materialize_version(next_version)?;
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

    fn materialize_version(version: ActiveVersion) -> ApllodbResult<()>;

    fn dematerialize_table(name: &TableName) -> ApllodbResult<Table>;
}
