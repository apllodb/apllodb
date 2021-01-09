use apllodb_shared_components::ApllodbResult;
use apllodb_shared_components::{AlterTableAction, ColumnDefinition, TableConstraints, TableName};
use std::fmt::Debug;

use crate::StorageEngine;

/// DDL access methods interface.
///
/// Not only DML but also DDL are executed under the transaction context (like PostgreSQL).
pub trait DDLMethods<Engine: StorageEngine>: Debug {
    /// CREATE TABLE command.
    fn create_table(
        &self,
        tx: &mut Engine::Tx,
        table_name: &TableName,
        table_constraints: &TableConstraints,
        column_definitions: Vec<ColumnDefinition>,
    ) -> ApllodbResult<()>;

    /// ALTER TABLE command.
    fn alter_table(
        &self,
        tx: &mut Engine::Tx,
        table_name: &TableName,
        action: &AlterTableAction,
    ) -> ApllodbResult<()>;

    /// DROP TABLE command.
    fn drop_table(&self, tx: &mut Engine::Tx, table_name: &TableName) -> ApllodbResult<()>;
}
