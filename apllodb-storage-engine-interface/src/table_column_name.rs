use apllodb_shared_components::{SchemaIndex, SchemaName};
use serde::{Deserialize, Serialize};

use crate::{column::column_name::ColumnName, table::table_name::TableName};

/// Full name in storage-engine: `TableName . ColumnName`.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
pub struct TableColumnName {
    table: TableName,
    column: ColumnName,
}

impl SchemaName for TableColumnName {
    fn _attr_matches(&self, attr: &str) -> bool {
        self.column.as_str() == attr
    }

    fn _prefix_matches(&self, prefix: &str) -> bool {
        self.table.as_str() == prefix
    }
}

impl TableColumnName {
    /// ref to table name
    pub fn as_table_name(&self) -> &TableName {
        &self.table
    }

    /// ref to column name
    pub fn as_column_name(&self) -> &ColumnName {
        &self.column
    }
}

impl From<TableColumnName> for SchemaIndex {
    fn from(tc: TableColumnName) -> Self {
        let s = format!(
            "{}.{}",
            tc.as_table_name().as_str(),
            tc.as_column_name().as_str()
        );
        SchemaIndex::from(s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::TableColumnName;
    use apllodb_shared_components::{SchemaIndex, SchemaName};

    #[test]
    fn test_matches() {
        struct TestDatum {
            schema_index: &'static str,
            table_column_name: TableColumnName,
            matches: bool,
        }

        let test_data: Vec<TestDatum> = vec![
            TestDatum {
                schema_index: "c",
                table_column_name: TableColumnName::factory("t", "c"),
                matches: true,
            },
            TestDatum {
                schema_index: "xxx",
                table_column_name: TableColumnName::factory("t", "c"),
                matches: false,
            },
            TestDatum {
                schema_index: "t.c",
                table_column_name: TableColumnName::factory("t", "c"),
                matches: true,
            },
            TestDatum {
                schema_index: "xxx.c",
                table_column_name: TableColumnName::factory("t", "c"),
                matches: false,
            },
            TestDatum {
                schema_index: "t",
                table_column_name: TableColumnName::factory("t", "c"),
                matches: false,
            },
            TestDatum {
                schema_index: "c.t",
                table_column_name: TableColumnName::factory("t", "c"),
                matches: false,
            },
        ];

        for test_datum in test_data {
            let index = SchemaIndex::from(test_datum.schema_index);
            assert_eq!(
                test_datum.table_column_name.matches(&index),
                test_datum.matches
            );
        }
    }
}
