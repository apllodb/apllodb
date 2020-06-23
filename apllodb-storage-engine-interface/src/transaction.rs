mod transaction_id;

pub use transaction_id::TransactionId;

use crate::Row;
use apllodb_shared_components::data_structure::{
    AlterTableAction, ColumnDefinition, ColumnName, DatabaseName, Expression, TableConstraints,
    TableName,
};
use apllodb_shared_components::{error::ApllodbResult, traits::Database};
use std::collections::HashMap;

/// Transaction interface.
///
/// It has methods to control transaction's lifetime (BEGIN, COMMIT/ABORT)
/// and storage engine's access methods (like system calls in OS).
///
/// Not only DML but also DDL are executed under the transaction context (like PostgreSQL).
///
/// Implementation of this trait can either execute physical transaction operations (e.g. locking objects, writing logs to disk, etc...)
/// directly or delegate physical operations to another object.
/// See [apllodb-immutable-schema-engine-interface-adapter::TransactionController](foo.html) (impl of `Transaction`) and [apllodb-immutable-schema-engine-domain::ImmutableSchemaTx](foo.html) (interface of physical transaction) for latter example.
pub trait Transaction<'tx, 'db: 'tx> {
    /// Transaction ID.
    type TID: TransactionId;

    /// Database in which this transaction works.
    type Db: Database + 'db;

    /// Iterator of [Row](foobar.html)s returned from [select()](foobar.html) method.
    type RowIter: Iterator<Item = ApllodbResult<Row>>;

    /// Transaction ID.
    fn id(&self) -> &Self::TID;

    /// Begins a transaction.
    /// A database cannot starts multiple transactions at a time (&mut reference enforces it).
    fn begin(db: &'db mut Self::Db) -> ApllodbResult<Self>
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
        &'tx self,
        table_name: &TableName,
        table_constraints: &TableConstraints,
        column_definitions: &[ColumnDefinition],
    ) -> ApllodbResult<()>;

    /// ALTER TABLE command.
    fn alter_table(
        &'tx self,
        table_name: &TableName,
        action: &AlterTableAction,
    ) -> ApllodbResult<()>;

    /// DROP TABLE command.
    fn drop_table(&'tx self, table_name: &TableName) -> ApllodbResult<()>;

    /// SELECT command.
    ///
    /// Storage engine's SELECT fields are merely ColumnName.
    /// Expression's are not allowed. Calculating expressions is job for query processor.
    fn select(
        &'tx self,
        table_name: &TableName,
        column_names: &[ColumnName],
    ) -> ApllodbResult<Self::RowIter>;

    /// INSERT command.
    fn insert(
        &'tx self,
        table_name: &TableName,
        column_values: HashMap<ColumnName, Expression>,
    ) -> ApllodbResult<()>;

    /// UPDATE command.
    ///
    /// TODO interface
    fn update(&'tx self, table_name: &TableName) -> ApllodbResult<()>;

    /// DELETE command.
    ///
    /// TODO interface
    fn delete(&'tx self, table_name: &TableName) -> ApllodbResult<()>;
}
