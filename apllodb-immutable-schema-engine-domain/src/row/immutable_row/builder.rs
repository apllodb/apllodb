use crate::{row::pk::Revision, FullPrimaryKey, ImmutableRow};
use apllodb_shared_components::{
    data_structure::{ColumnName, SqlValue},
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use std::collections::HashMap;

/// Builder for ImmutableRow.
#[derive(Debug, Default)]
pub struct ImmutableRowBuilder {
    pk: Option<FullPrimaryKey>,
    columns: HashMap<ColumnName, SqlValue>,
}

impl ImmutableRowBuilder {
    /// Add PK to row.
    ///
    /// # Failures
    ///
    /// - [DuplicateColumn](error/enum.ApllodbErrorKind.html#variant.DuplicateColumn) when:
    ///   - Same `ColumnName` added twice.
    pub fn add_pk(
        // TODO このシグネチャだと複合PKに対応できてない
        mut self,
        column_name: &ColumnName,
        value: SqlValue,
        revision: Revision, // TODO 使う
    ) -> ApllodbResult<Self> {
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
    ///
    /// TODO validate duplicate column name.
    pub fn build(self) -> ApllodbResult<ImmutableRow> {
        let pk = self.pk.ok_or_else(|| {
            ApllodbError::new(
                ApllodbErrorKind::UndefinedPrimaryKey,
                "ImmutableRowBuilder::build() called before ImmutableRowBuilder::add_pk()",
                None,
            )
        })?;
        if self.columns.contains_key(pk.column_name()) {
            Err(ApllodbError::new(
                ApllodbErrorKind::DuplicateColumn,
                format!(
                    "column `{}` is already added to this record",
                    pk.column_name()
                ),
                None,
            ))
        } else {
            Ok(ImmutableRow {
                pk,
                columns: self.columns,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ImmutableRowBuilder;
    use apllodb_shared_components::{
        data_structure::{ColumnName, DataType, DataTypeKind, SqlValue},
        error::ApllodbResult,
    };

    #[test]
    fn test_empty_row() -> ApllodbResult<()> {
        let row1 = ImmutableRowBuilder::default().build()?;
        let row2 = ImmutableRowBuilder::default().build()?;
        assert_eq!(row1, row2);

        Ok(())
    }

    #[test]
    fn test_success() -> ApllodbResult<()> {
        let row1 = ImmutableRowBuilder::default()
            .add_column(
                &ColumnName::new("c1")?,
                SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &0i32)?,
            )?
            .add_column(
                &ColumnName::new("c2")?,
                SqlValue::pack(&DataType::new(DataTypeKind::Integer, true), &None::<i32>)?,
            )?
            .build()?;

        let row2 = ImmutableRowBuilder::default()
            .add_column(
                &ColumnName::new("c2")?,
                SqlValue::pack(&DataType::new(DataTypeKind::Integer, true), &None::<i32>)?,
            )?
            .add_column(
                &ColumnName::new("c1")?,
                SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &0i32)?,
            )?
            .build()?;

        assert_eq!(row1, row2);

        Ok(())
    }
}
