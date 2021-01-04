pub(crate) mod transaction_id;

use apllodb_shared_components::ApllodbResult;
use apllodb_shared_components::{
    AlterTableAction, ColumnDefinition, ColumnName, DatabaseName, Expression, RecordIterator,
    TableConstraints, TableName,
};
use std::{collections::HashMap, fmt::Debug};

use crate::{ProjectionQuery, StorageEngine};

/// Transaction builder.
/// Implement this to contain reference type in it.
/// (Without builder, [Transaction::begin()](crate::Transaction::begin) may take reference type and it should take lifetime parameter although Transaction is a trait.)
pub trait TransactionBuilder: Debug {}

/// Transaction interface.
///
/// It has methods to control transaction's lifetime (BEGIN, COMMIT/ABORT)
/// and storage engine's access methods (like system calls in OS).
///
/// Not only DML but also DDL are executed under the transaction context (like PostgreSQL).
///
/// Implementation of this trait can either execute physical transaction operations (e.g. locking objects, writing logs to disk, etc...)
/// directly or delegate physical operations to another object.
pub trait Transaction<Engine: StorageEngine>: Debug {
    /// Transaction ID
    fn id(&self) -> &Engine::TID;

    /// Begins a transaction.
    fn begin(builder: Engine::TxBuilder) -> ApllodbResult<Self>
    where
        Self: Sized;

    /// Commit a transaction.
    ///
    /// # Failures
    ///
    /// Vary between transaction implementations but all implementations must ABORT transaction on failure.
    fn commit(self) -> ApllodbResult<()>;

    /// Abort (rollback) a transaction.
    fn abort(self) -> ApllodbResult<()>;

    /// Ref to database name.
    fn database_name(&self) -> &DatabaseName;

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
}
