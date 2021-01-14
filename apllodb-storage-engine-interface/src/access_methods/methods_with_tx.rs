pub(crate) mod projection;

use apllodb_shared_components::{
    AlterTableAction, ColumnDefinition, SessionWithTx, TableConstraints, TableName,
};
use apllodb_shared_components::{ApllodbResult, ColumnName, Expression, RecordIterator};
use std::collections::HashMap;

use crate::ProjectionQuery;

#[cfg(feature = "test-support")]
use mockall::automock;

/// Access methods with open transaction.
#[cfg_attr(feature = "test-support", automock)]
pub trait MethodsWithTx {
    /// CREATE TABLE command.
    fn create_table(
        &self,
        session: &SessionWithTx,
        table_name: &TableName,
        table_constraints: &TableConstraints,
        column_definitions: Vec<ColumnDefinition>,
    ) -> ApllodbResult<()>;

    /// ALTER TABLE command.
    fn alter_table(
        &self,
        session: &SessionWithTx,
        table_name: &TableName,
        action: &AlterTableAction,
    ) -> ApllodbResult<()>;

    /// DROP TABLE command.
    fn drop_table(&self, session: &SessionWithTx, table_name: &TableName) -> ApllodbResult<()>;

    /// SELECT command.
    ///
    /// Storage engine's SELECT fields are merely ColumnName.
    /// Expression fields are not allowed. Calculating expressions is job for query processor.
    fn select(
        &self,
        session: &SessionWithTx,
        table_name: &TableName,
        projection: ProjectionQuery,
    ) -> ApllodbResult<RecordIterator>;

    /// INSERT command.
    fn insert(
        &self,
        session: &SessionWithTx,
        table_name: &TableName,
        records: RecordIterator,
    ) -> ApllodbResult<()>;

    /// UPDATE command.
    fn update(
        &self,
        session: &SessionWithTx,
        table_name: &TableName,
        column_values: HashMap<ColumnName, Expression>,
    ) -> ApllodbResult<()>;

    /// DELETE command.
    fn delete(&self, session: &SessionWithTx, table_name: &TableName) -> ApllodbResult<()>;

    /// Commit a transaction and calls [SessionWithDb::unset_tid()](apllodb-shared-components::SessionWithDb::unset_tid()).
    fn commit(self, session: SessionWithTx) -> ApllodbResult<()>;

    /// Abort (rollback) a transaction and calls [SessionWithDb::unset_tid()](apllodb-shared-components::SessionWithDb::unset_tid())..
    fn abort(self, session: SessionWithTx) -> ApllodbResult<()>;
}
