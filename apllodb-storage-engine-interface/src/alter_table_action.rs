use serde::{Deserialize, Serialize};

use crate::column::{column_definition::ColumnDefinition, column_name::ColumnName};

/// Actions to be done by ALTER TABLE statement.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum AlterTableAction {
    /// ALTER TABLE {table_name} ADD COLUMN {column_definition}
    AddColumn {
        /// Column to add
        column_definition: ColumnDefinition,
    },

    /// ALTER TABLE {table_name} DROP COLUMN {column_name}
    DropColumn {
        /// Column to drop. Currently PK column cannot be dropped.
        column_name: ColumnName,
    },
}
