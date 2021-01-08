use apllodb_shared_components::ApllodbResult;
use apllodb_shared_components::{AlterTableAction, ColumnDefinition, TableConstraints, TableName};
use std::fmt::Debug;

/// DDL access methods interface.
///
/// Not only DML but also DDL are executed under the transaction context (like PostgreSQL).
pub trait DDLMethods: Debug {
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
}
