use crate::Row;
use apllodb_shared_components::{
    data_structure::{ColumnName, SqlValue},
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use std::collections::HashMap;

/// Builder for Row.
#[derive(Debug, Default)]
pub struct RowBuilder {
    columns: HashMap<ColumnName, SqlValue>,
}

impl RowBuilder {
    /// Add column to row.
    ///
    /// # Failures
    ///
    /// - [DuplicateColumn](error/enum.ApllodbErrorKind.html#variant.DuplicateColumn) when:
    ///   - Same `ColumnName` added twice.
    pub fn add_column(mut self, column_name: &ColumnName, value: SqlValue) -> ApllodbResult<Self> {
        if let Some(_) = self.columns.insert(column_name.clone(), value) {
            Err(ApllodbError::new(
                ApllodbErrorKind::DuplicateColumn,
                format!("column `{}` is already added to this record", column_name),
                None,
            ))
        } else {
            Ok(self)
        }
    }

    /// Finalize.
    pub fn build(self) -> Row {
        Row {
            columns: self.columns,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RowBuilder;
    use apllodb_shared_components::{
        data_structure::{ColumnName, DataType, DataTypeKind, SqlValue},
        error::ApllodbResult,
    };

    #[test]
    fn test_empty_row() {
        let row1 = RowBuilder::default().build();
        let row2 = RowBuilder::default().build();
        assert_eq!(row1, row2);
    }

    #[test]
    fn test_success() -> ApllodbResult<()> {
        let row1 = RowBuilder::default()
            .add_column(
                &ColumnName::new("c1")?,
                SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &0i32)?,
            )?
            .add_column(
                &ColumnName::new("c2")?,
                SqlValue::pack(&DataType::new(DataTypeKind::Integer, true), &None::<i32>)?,
            )?
            .build();

        let row2 = RowBuilder::default()
            .add_column(
                &ColumnName::new("c2")?,
                SqlValue::pack(&DataType::new(DataTypeKind::Integer, true), &None::<i32>)?,
            )?
            .add_column(
                &ColumnName::new("c1")?,
                SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &0i32)?,
            )?
            .build();

        assert_eq!(row1, row2);

        Ok(())
    }
}
