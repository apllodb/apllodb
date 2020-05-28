use crate::TxCtxLike;
use apllodb_shared_components::data_structure::{
    AlterTableAction, ColumnDefinition, TableConstraints, TableName,
};
use apllodb_shared_components::error::ApllodbResult;

/// Access methods for DDL.
pub trait AccessMethodsDdl<'st, Tx: TxCtxLike<'st>> {
    // TODO async とかつけような

    /// CREATE TABLE command.
    fn create_table(
        tx: &mut Tx,
        table_name: &TableName,
        table_constraints: &TableConstraints,
        column_definitions: &[ColumnDefinition],
    ) -> ApllodbResult<()>;

    /// ALTER TABLE command.
    fn alter_table(
        tx: &mut Tx,
        table_name: &TableName,
        action: &AlterTableAction,
    ) -> ApllodbResult<()>;

    /// DROP TABLE command.
    fn drop_table(tx: &mut Tx) -> ApllodbResult<()>;
}
