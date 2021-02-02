use super::ImmutableRow;

use apllodb_shared_components::{
    ApllodbError, ApllodbErrorKind, ApllodbResult, FullFieldReference, ColumnValue, SqlValue,
};
use std::collections::HashMap;

/// Builder for ImmutableRow.
#[derive(Debug, Default)]
pub struct ImmutableRowBuilder {
    col_vals: HashMap<FullFieldReference, SqlValue>,
}

impl ImmutableRowBuilder {
    /// Add column value to row.
    ///
    /// # Failures
    ///
    /// - [DuplicateColumn](apllodb_shared_components::ApllodbErrorKind::DuplicateColumn) when:
    ///   - Same `ColumnName` added twice.
    pub fn add_colval(mut self, colval: ColumnValue) -> ApllodbResult<Self> {
        let colref = colval.as_column_name().clone();

        if self
            .col_vals
            .insert(colref.clone(), colval.into_sql_value())
            .is_some()
        {
            Err(ApllodbError::new(
                ApllodbErrorKind::DuplicateColumn,
                format!("column `{:?}` is already added to this record", colref),
                None,
            ))
        } else {
            Ok(self)
        }
    }

    pub fn add_void_projection(self, colref: &FullFieldReference) -> ApllodbResult<Self> {
        self.add_colval(ColumnValue::new(colref.clone(), SqlValue::Null))
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
    use apllodb_shared_components::{
        ApllodbResult, ColumnName, FullFieldReference, ColumnValue, NNSqlValue, SqlValue, TableName,
    };

    #[test]
    fn test_success() -> ApllodbResult<()> {
        let colref = FullFieldReference::new(TableName::new("t")?, ColumnName::new("c1")?);

        let mut row1 = ImmutableRowBuilder::default()
            .add_colval(ColumnValue::new(
                colref.clone(),
                SqlValue::NotNull(NNSqlValue::Integer(0)),
            ))?
            .build()?;
        let mut row2 = ImmutableRowBuilder::default()
            .add_colval(ColumnValue::new(
                colref.clone(),
                SqlValue::NotNull(NNSqlValue::Integer(0)),
            ))?
            .build()?;

        assert_eq!(row1.get::<i32>(&colref)?, row2.get::<i32>(&colref)?);

        Ok(())
    }

    #[test]
    fn test_add_order_does_not_matter() -> ApllodbResult<()> {
        let colref1 = FullFieldReference::new(TableName::new("t")?, ColumnName::new("c1")?);
        let colref2 = FullFieldReference::new(TableName::new("t")?, ColumnName::new("c2")?);

        let row1 = ImmutableRowBuilder::default()
            .add_colval(ColumnValue::new(
                colref1.clone(),
                SqlValue::NotNull(NNSqlValue::Integer(0)),
            ))?
            .add_colval(ColumnValue::new(
                colref2.clone(),
                SqlValue::NotNull(NNSqlValue::Integer(1)),
            ))?
            .build()?;

        let row2 = ImmutableRowBuilder::default()
            .add_colval(ColumnValue::new(
                colref2,
                SqlValue::NotNull(NNSqlValue::Integer(1)),
            ))?
            .add_colval(ColumnValue::new(
                colref1,
                SqlValue::NotNull(NNSqlValue::Integer(0)),
            ))?
            .build()?;

        assert_eq!(row1, row2);

        Ok(())
    }
}
