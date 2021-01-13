use apllodb_shared_components::ApllodbResult;
use apllodb_shared_components::{AlterTableAction, ColumnDefinition, TableConstraints, TableName};
use std::fmt::Debug;

/// DDL access methods interface.
///
/// DDL methods implementations have freedom of choice about whether to realize transactional DDL.
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
