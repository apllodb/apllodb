use super::AccessMethods;
use crate::transaction::{SqliteRowIterator, SqliteTx};
use crate::ImmutableSchemaTx;
use apllodb_shared_components::data_structure::{ColumnName, Expression, TableName};
use apllodb_shared_components::error::ApllodbResult;
use apllodb_storage_manager_interface::AccessMethodsDml;

impl<'stmt, 'db: 'stmt> AccessMethodsDml<SqliteTx<'db>, SqliteRowIterator<'stmt>>
    for AccessMethods
{
    // TODO async とかつけような

    /// SELECT command.
    fn select(
        tx: &mut SqliteTx<'db>,
        table_name: &TableName,
        column_names: &[ColumnName],
    ) -> ApllodbResult<SqliteRowIterator<'stmt>> {
        let table = tx.read_table(table_name)?;
        table.select(column_names)
    }

    fn insert(
        _tx: &mut SqliteTx<'db>,
        _table_name: &TableName,
        _column_values: std::collections::HashMap<ColumnName, Expression>,
    ) -> ApllodbResult<()> {
        todo!()
    }

    fn update(_tx: &mut SqliteTx<'db>, _table_name: &TableName) -> ApllodbResult<()> {
        todo!()
    }

    fn delete(_tx: &mut SqliteTx<'db>, _table_name: &TableName) -> ApllodbResult<()> {
        todo!()
    }
}
