use super::AccessMethods;
use crate::transaction::{SqliteRecordIterator, SqliteTx};
use apllodb_shared_components::data_structure::{ColumnName, Expression, TableName};
use apllodb_shared_components::error::ApllodbResult;
use apllodb_storage_manager_interface::AccessMethodsDml;

impl<'db> AccessMethodsDml<SqliteTx<'db>, SqliteRecordIterator> for AccessMethods {
    // TODO async とかつけような

    /// SELECT command.
    fn select(
        _tx: &mut SqliteTx<'db>,
        _table_name: &TableName,

        // TODO: use SelectField like structure in apllodb-AST to allow alias.
        _fields: &[Expression],
    ) -> ApllodbResult<SqliteRecordIterator> {
        todo!()
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
