use super::ColumnName;
use serde::{Deserialize, Serialize};

/// Actions to be done by ALTER TABLE statement.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum AlterTableAction {
    /// ALTER TABLE {table_name} DROP COLUMN {column_name}
    DropColumn {
        /// Column to drop. Currently PK column cannot be droppe.
        column_name: ColumnName,
    },
}
