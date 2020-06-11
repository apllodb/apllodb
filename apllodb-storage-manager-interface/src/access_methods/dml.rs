use crate::{Row, TxCtxLike};
use apllodb_shared_components::data_structure::{ColumnName, Expression, TableName};
use apllodb_shared_components::error::ApllodbResult;
use std::collections::HashMap;

/// Access methods for DML.
pub trait AccessMethodsDml<Tx, RowIter>
where
    Tx: TxCtxLike,
    RowIter: Iterator<Item = ApllodbResult<Row>>,
{
    /// SELECT command.
    ///
    /// Storage engine's SELECT fields are merely ColumnName.
    /// Expression's are not allowed. Calculating expressions is job for query processor.
    fn select(
        tx: &mut Tx,
        table_name: &TableName,
        column_names: &[ColumnName],
    ) -> ApllodbResult<RowIter>;

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
