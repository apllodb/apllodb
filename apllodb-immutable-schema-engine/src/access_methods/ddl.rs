use super::AccessMethods;
use crate::{transaction::SimpleTx, Table};
use apllodb_shared_components::data_structure::{
    AlterTableAction, ColumnDefinition, TableConstraints, TableName,
};
use apllodb_shared_components::error::ApllodbResult;
use apllodb_storage_manager_interface::AccessMethodsDdl;

impl<'st> AccessMethodsDdl<'st, SimpleTx<'st>> for AccessMethods {
    // TODO async とかつけような

    /// CREATE TABLE command.
    ///
    /// Transactional.
    ///
    /// # Failures
    ///
    /// - Errors from [Table::new](foobar.html).
    /// - Errors from [ActiveVersion::create_initial](foobar.html).
    /// - Errors from [Tx::write_table](foobar.html).
    fn create_table(
        tx: &mut SimpleTx<'st>,
        table_name: &TableName,
        table_constraints: &TableConstraints,
        column_definitions: &[ColumnDefinition],
    ) -> ApllodbResult<()> {
        let table = Table::create(table_name, table_constraints, column_definitions)?;
        tx.write_table(table)?;
        Ok(())
    }

    /// ALTER TABLE command.
    ///
    /// Transactional.
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
    fn alter_table(
        tx: &mut SimpleTx<'st>,
        table_name: &TableName,
        action: &AlterTableAction,
    ) -> ApllodbResult<()> {
        let mut table = tx.read_table(table_name)?;
        table.alter(action)?;
        tx.write_table(table)?;

        Ok(())
    }

    /// DROP TABLE command.
    ///
    /// Transactional.
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
    fn drop_table(_tx: &mut SimpleTx<'st>) -> ApllodbResult<()> {
        todo!()
    }
}
