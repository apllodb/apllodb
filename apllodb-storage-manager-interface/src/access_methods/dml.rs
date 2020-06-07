use crate::TxCtxLike;
use apllodb_shared_components::data_structure::{ColumnName, Expression, Record, TableName};
use apllodb_shared_components::error::ApllodbResult;
use std::collections::HashMap;

/// Access methods for DML.
pub trait AccessMethodsDml<Tx, RecIter>
where
    Tx: TxCtxLike,
    RecIter: Iterator<Item = ApllodbResult<Record>>,
{
    /// SELECT command.
    fn select(
        tx: &mut Tx,
        table_name: &TableName,

        // TODO: use SelectField like structure in apllodb-AST to allow alias.
        fields: &[Expression],
    ) -> ApllodbResult<RecIter>;

    /// INSERT command.
    fn insert(
        tx: &mut Tx,
        table_name: &TableName,
        column_values: HashMap<ColumnName, Expression>,
    ) -> ApllodbResult<()>;

    /// UPDATE command.
    ///
    /// TODO interface
    fn update(tx: &mut Tx, table_name: &TableName) -> ApllodbResult<()>;

    /// DELETE command.
    ///
    /// TODO interface
    fn delete(tx: &mut Tx, table_name: &TableName) -> ApllodbResult<()>;
}
