pub(crate) mod projection;

use apllodb_shared_components::{AlterTableAction, ColumnDefinition, TableConstraints, TableName};
use apllodb_shared_components::{ApllodbResult, ColumnName, Expression, RecordIterator};
use std::collections::HashMap;

use crate::ProjectionQuery;

#[cfg(any(test, feature = "test_support"))]
use mockall::automock;

/// Access methods with open transaction.
#[cfg_attr(any(test, feature = "test_support"), automock)]
pub trait MethodsWithTx {
    /// CREATE TABLE command.
    fn create_table(
        &self,
        table_name: &TableName,
        table_constraints: &TableConstraints,
        column_definitions: Vec<ColumnDefinition>,
    ) -> ApllodbResult<()>;

    /// ALTER TABLE command.
    fn alter_table(&self, table_name: &TableName, action: &AlterTableAction) -> ApllodbResult<()>;

    /// DROP TABLE command.
    fn drop_table(&self, table_name: &TableName) -> ApllodbResult<()>;

    /// SELECT command.
    ///
    /// Storage engine's SELECT fields are merely ColumnName.
    /// Expression fields are not allowed. Calculating expressions is job for query processor.
    fn select(
        &self,
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

    /// Commit a transaction and calls [SessionWithDb::unset_tid()](apllodb-shared-components::SessionWithDb::unset_tid()).
    fn commit(self) -> ApllodbResult<()>;

    /// Abort (rollback) a transaction and calls [SessionWithDb::unset_tid()](apllodb-shared-components::SessionWithDb::unset_tid())..
    fn abort(self) -> ApllodbResult<()>;
}
