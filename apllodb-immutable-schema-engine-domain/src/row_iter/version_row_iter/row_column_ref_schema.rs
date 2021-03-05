use apllodb_shared_components::{
    ApllodbError, ApllodbErrorKind, ApllodbResult, ColumnName, TableName,
};
use serde::{Deserialize, Serialize};

/// Internally has similar structure as `Vec<ColumnColumn>` and works with [SqlValues](apllodb-shared-components::SqlValues) with the same length
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct RowColumnRefSchema {
    table_name: TableName,
    column_names: Vec<ColumnName>,
}

impl RowColumnRefSchema {
    /// Constructor
    pub fn new(table_name: TableName, column_names: Vec<ColumnName>) -> Self {
        Self {
            table_name,
            column_names,
        }
    }

    pub fn empty() -> Self {
        Self::new(TableName::new("from_empty_rows").unwrap(), vec![])
    }

    pub fn as_column_names(&self) -> &[ColumnName] {
        &self.column_names
    }

    pub fn into_column_names(self) -> Vec<ColumnName> {
        self.column_names
    }

    /// # Failures
    ///
    /// - [DuplicateColumn](apllodb_shared_components::ApllodbErrorKind::DuplicateColumn) when:
    ///   - Same [ColumnReference](apllodb_shared_components::ColumnReference) is already in this row.
    pub(crate) fn append(&mut self, column_name: ColumnName) -> ApllodbResult<()> {
        if self.column_names.iter().any(|cn| cn == &column_name) {
            Err(ApllodbError::new(
                ApllodbErrorKind::DuplicateColumn,
                format!("column `{}` is already in this row", column_name.as_str()),
                None,
            ))
        } else {
            self.column_names.push(column_name);
            Ok(())
        }
    }

    /// # Failures
    ///
    /// - [UndefinedColumn](apllodb-shared-components::ApllodbErrorKind::UndefinedColumn) when:
    ///   - `column_name` does not exist in this row.
    pub(crate) fn resolve_index_with_rm(
        &mut self,
        column_name: &ColumnName,
    ) -> ApllodbResult<usize> {
        let idx = self
            .column_names
            .iter()
            .position(|cn| cn == column_name)
            .ok_or_else(|| {
                ApllodbError::new(
                    ApllodbErrorKind::UndefinedColumn,
                    format!(
                        "column named `{}` does not exist in this row",
                        column_name.as_str()
                    ),
                    None,
                )
            })?;

        self.column_names.remove(idx);
        Ok(idx)
    }
}
