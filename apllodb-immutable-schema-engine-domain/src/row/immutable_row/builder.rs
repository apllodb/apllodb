use super::ImmutableRow;

use apllodb_shared_components::{
    ApllodbError, ApllodbErrorKind, ApllodbResult, ColumnReference, SqlValue,
};
use std::collections::HashMap;

/// Builder for ImmutableRow.
#[derive(Debug, Default)]
pub struct ImmutableRowBuilder {
    col_vals: HashMap<ColumnReference, SqlValue>,
}

impl ImmutableRowBuilder {
    /// Add column value to row.
    ///
    /// # Failures
    ///
    /// - [DuplicateColumn](apllodb_shared_components::ApllodbErrorKind::DuplicateColumn) when:
    ///   - Same `ColumnName` added twice.
    pub fn add_col_val(mut self, colref: &ColumnReference, value: SqlValue) -> ApllodbResult<Self> {
        if self.col_vals.insert(colref.clone(), value).is_some() {
            Err(ApllodbError::new(
                ApllodbErrorKind::DuplicateColumn,
                format!("column `{:?}` is already added to this record", colref),
                None,
            ))
        } else {
            Ok(self)
        }
    }

    pub fn add_void_projection(self, colref: &ColumnReference) -> ApllodbResult<Self> {
        let null = SqlValue::null();
        self.add_col_val(colref, null)
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
    use pretty_assertions::assert_eq;

    use super::ImmutableRowBuilder;
    use crate::test_support::setup;
    use apllodb_shared_components::{
        ApllodbResult, ColumnName, ColumnReference, DataType, DataTypeKind, SqlValue, TableName,
    };
    use apllodb_storage_engine_interface::Row;

    #[test]
    fn test_success() -> ApllodbResult<()> {
        setup();

        let colref = ColumnReference::new(TableName::new("t")?, ColumnName::new("c1")?);

        let mut row1 = ImmutableRowBuilder::default()
            .add_col_val(
                &colref,
                SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &0i32)?,
            )?
            .build()?;
        let mut row2 = ImmutableRowBuilder::default()
            .add_col_val(
                &colref,
                SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &0i32)?,
            )?
            .build()?;

        assert_eq!(row1.get::<i32>(&colref)?, row2.get::<i32>(&colref)?);

        Ok(())
    }

    #[test]
    fn test_add_order_does_not_matter() -> ApllodbResult<()> {
        setup();

        let colref1 = ColumnReference::new(TableName::new("t")?, ColumnName::new("c1")?);
        let colref2 = ColumnReference::new(TableName::new("t")?, ColumnName::new("c2")?);

        let row1 = ImmutableRowBuilder::default()
            .add_col_val(
                &colref1,
                SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &0i32)?,
            )?
            .add_col_val(
                &colref2,
                SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &1i32)?,
            )?
            .build()?;

        let row2 = ImmutableRowBuilder::default()
            .add_col_val(
                &colref2,
                SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &1i32)?,
            )?
            .add_col_val(
                &colref1,
                SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &0i32)?,
            )?
            .build()?;

        assert_eq!(row1, row2);

        Ok(())
    }
}
