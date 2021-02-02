use super::ImmutableRow;

use apllodb_shared_components::{ApllodbError, ApllodbErrorKind, ApllodbResult, SqlValue};
use apllodb_storage_engine_interface::TableColumnReference;
use std::collections::HashMap;

/// Builder for ImmutableRow.
#[derive(Debug, Default)]
pub struct ImmutableRowBuilder {
    col_vals: HashMap<TableColumnReference, SqlValue>,
}

impl ImmutableRowBuilder {
    /// Add column value to row.
    ///
    /// # Failures
    ///
    /// - [DuplicateColumn](apllodb_shared_components::ApllodbErrorKind::DuplicateColumn) when:
    ///   - Same `ColumnName` added twice.
    pub fn append(
        mut self,
        table_column_reference: TableColumnReference,
        sql_value: SqlValue,
    ) -> ApllodbResult<Self> {
        if self
            .col_vals
            .insert(table_column_reference.clone(), sql_value)
            .is_some()
        {
            Err(ApllodbError::new(
                ApllodbErrorKind::DuplicateColumn,
                format!(
                    "column `{}` is already added to this row",
                    table_column_reference
                ),
                None,
            ))
        } else {
            Ok(self)
        }
    }

    pub fn add_void_projection(
        self,
        table_column_reference: TableColumnReference,
    ) -> ApllodbResult<Self> {
        self.append(table_column_reference, SqlValue::Null)
    }

    /// Finalize.
    ///
    /// TODO validate duplicate column name.
    pub fn build(self) -> ApllodbResult<ImmutableRow> {
        Ok(ImmutableRow {
            col_vals: self.col_vals,
        })
    }
}

#[cfg(test)]
mod tests {
    use apllodb_storage_engine_interface::TableColumnReference;
    use pretty_assertions::assert_eq;

    use super::ImmutableRowBuilder;
    use apllodb_shared_components::{ApllodbResult, NNSqlValue, SqlValue};

    #[test]
    fn test_success() -> ApllodbResult<()> {
        let tcr = TableColumnReference::factory("t", "c1");

        let mut row1 = ImmutableRowBuilder::default()
            .append(tcr.clone(), SqlValue::NotNull(NNSqlValue::Integer(0)))?
            .build()?;
        let mut row2 = ImmutableRowBuilder::default()
            .append(tcr.clone(), SqlValue::NotNull(NNSqlValue::Integer(0)))?
            .build()?;

        assert_eq!(row1.get::<i32>(&tcr)?, row2.get::<i32>(&tcr)?);

        Ok(())
    }

    #[test]
    fn test_add_order_does_not_matter() -> ApllodbResult<()> {
        let tcr1 = TableColumnReference::factory("t", "c1");
        let tcr2 = TableColumnReference::factory("t", "c2");

        let row1 = ImmutableRowBuilder::default()
            .append(tcr1.clone(), SqlValue::NotNull(NNSqlValue::Integer(0)))?
            .append(tcr2.clone(), SqlValue::NotNull(NNSqlValue::Integer(1)))?
            .build()?;

        let row2 = ImmutableRowBuilder::default()
            .append(tcr2, SqlValue::NotNull(NNSqlValue::Integer(1)))?
            .append(tcr1, SqlValue::NotNull(NNSqlValue::Integer(0)))?
            .build()?;

        assert_eq!(row1, row2);

        Ok(())
    }
}
