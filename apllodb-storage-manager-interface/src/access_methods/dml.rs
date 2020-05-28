use crate::TxCtxLike;
use apllodb_shared_components::data_structure::TableName;
use apllodb_shared_components::error::ApllodbResult;

/// Access methods for DML.
pub trait AccessMethodsDml<'st, Tx: TxCtxLike<'st>> {
    /// SELECT command.
    ///
    /// TODO interface
    fn select(tx: &mut Tx, table_name: &TableName) -> ApllodbResult<()>;

    /// INSERT command.
    ///
    /// TODO interface
    fn insert(tx: &mut Tx, table_name: &TableName) -> ApllodbResult<()>;

    /// UPDATE command.
    ///
    /// TODO interface
    fn update(tx: &mut Tx, table_name: &TableName) -> ApllodbResult<()>;

    /// DELETE command.
    ///
    /// TODO interface
    fn delete(tx: &mut Tx, table_name: &TableName) -> ApllodbResult<()>;
}
