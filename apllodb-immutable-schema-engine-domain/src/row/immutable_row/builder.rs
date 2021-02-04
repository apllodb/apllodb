use super::ImmutableRow;

use apllodb_shared_components::{
    ApllodbError, ApllodbErrorKind, ApllodbResult, FullFieldReference, SqlValue,
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
    pub fn append(
        mut self,
        full_field_reference: FullFieldReference,
        sql_value: SqlValue,
    ) -> ApllodbResult<Self> {
        if self
            .col_vals
            .insert(full_field_reference.clone(), sql_value)
            .is_some()
        {
            Err(ApllodbError::new(
                ApllodbErrorKind::DuplicateColumn,
                format!(
                    "column `{}` is already added to this row",
                    full_field_reference
                ),
                None,
            ))
        } else {
            Ok(self)
        }
    }

    pub fn add_void_projection(
        self,
        full_field_reference: FullFieldReference,
    ) -> ApllodbResult<Self> {
        self.append(full_field_reference, SqlValue::Null)
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
    use apllodb_shared_components::{ApllodbResult, FullFieldReference, NNSqlValue, SqlValue};

    #[test]
    fn test_success() -> ApllodbResult<()> {
        let ffr = FullFieldReference::factory("t", "c1");

        let mut row1 = ImmutableRowBuilder::default()
            .append(ffr.clone(), SqlValue::NotNull(NNSqlValue::Integer(0)))?
            .build()?;
        let mut row2 = ImmutableRowBuilder::default()
            .append(ffr.clone(), SqlValue::NotNull(NNSqlValue::Integer(0)))?
            .build()?;

        assert_eq!(
            row1.get::<i32>(&ffr.clone().into())?,
            row2.get::<i32>(&ffr.clone().into())?
        );

        Ok(())
    }

    #[test]
    fn test_add_order_does_not_matter() -> ApllodbResult<()> {
        let ffr1 = FullFieldReference::factory("t", "c1");
        let ffr2 = FullFieldReference::factory("t", "c2");

        let row1 = ImmutableRowBuilder::default()
            .append(ffr1.clone(), SqlValue::NotNull(NNSqlValue::Integer(0)))?
            .append(ffr2.clone(), SqlValue::NotNull(NNSqlValue::Integer(1)))?
            .build()?;

        let row2 = ImmutableRowBuilder::default()
            .append(ffr2, SqlValue::NotNull(NNSqlValue::Integer(1)))?
            .append(ffr1, SqlValue::NotNull(NNSqlValue::Integer(0)))?
            .build()?;

        assert_eq!(row1, row2);

        Ok(())
    }
}
