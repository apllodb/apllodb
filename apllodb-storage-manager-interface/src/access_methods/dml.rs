use crate::TxCtxLike;
use apllodb_shared_components::data_structure::{ColumnName, Expression, TableName};
use apllodb_shared_components::error::ApllodbResult;
use std::collections::HashMap;

/// Access methods for DML.
pub trait AccessMethodsDml<Tx: TxCtxLike> {
    /// SELECT command.
    ///
    /// TODO interface
    fn select(tx: &mut Tx, table_name: &TableName) -> ApllodbResult<()>;

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
