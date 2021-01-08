pub(crate) mod projection;

use apllodb_shared_components::ApllodbResult;
use apllodb_shared_components::{ColumnName, Expression, RecordIterator, TableName};
use std::{collections::HashMap, fmt::Debug};

use crate::ProjectionQuery;

/// DML access methods interface.
pub trait DMLMethods: Debug {
    /// SELECT command.
    ///
    /// Storage engine's SELECT fields are merely ColumnName.
    /// Expression fields are not allowed. Calculating expressions is job for query processor.
    fn select(
        tx: &self,
        table_name: &TableName,
        projection: ProjectionQuery,
    ) -> ApllodbResult<RecordIterator>;

    /// INSERT command.
    fn insert(&self, table_name: &TableName, records: RecordIterator) -> ApllodbResult<()>;

    /// UPDATE command.
    fn update(
        &self,
        table_name: &TableName,
        column_values: HashMap<ColumnName, Expression>,
    ) -> ApllodbResult<()>;

    /// DELETE command.
    fn delete(&self, table_name: &TableName) -> ApllodbResult<()>;
}
