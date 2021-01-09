pub(crate) mod projection;

use apllodb_shared_components::ApllodbResult;
use apllodb_shared_components::{ColumnName, Expression, RecordIterator, TableName};
use std::{collections::HashMap, fmt::Debug};

use crate::{ProjectionQuery, StorageEngine};

/// DML access methods interface.
pub trait DMLMethods<Engine: StorageEngine>: Default + Debug {
    /// SELECT command.
    ///
    /// Storage engine's SELECT fields are merely ColumnName.
    /// Expression fields are not allowed. Calculating expressions is job for query processor.
    fn select(
        &self,
        tx: &mut Engine::Tx,
        table_name: &TableName,
        projection: ProjectionQuery,
    ) -> ApllodbResult<RecordIterator>;

    /// INSERT command.
    fn insert(
        &self,
        tx: &mut Engine::Tx,
        table_name: &TableName,
        records: RecordIterator,
    ) -> ApllodbResult<()>;

    /// UPDATE command.
    fn update(
        &self,
        tx: &mut Engine::Tx,
        table_name: &TableName,
        column_values: HashMap<ColumnName, Expression>,
    ) -> ApllodbResult<()>;

    /// DELETE command.
    fn delete(&self, tx: &mut Engine::Tx, table_name: &TableName) -> ApllodbResult<()>;
}
