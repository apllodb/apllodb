use super::ImmutableRow;
use crate::row::pk::{
    apparent_pk::ApparentPrimaryKey,
    full_pk::{revision::Revision, FullPrimaryKey},
};
use apllodb_shared_components::{
    data_structure::{ColumnName, SqlValue},
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use std::collections::HashMap;

/// Builder for ImmutableRow.
#[derive(Debug, Default)]
pub struct ImmutableRowBuilder {
    non_pk_columns: HashMap<ColumnName, SqlValue>,
    pk_builder: PKBuilder,
}

#[derive(Debug)]
struct PKBuilder {
    pk_columns: HashMap<ColumnName, SqlValue>,
    revision: Revision,
}
impl PKBuilder {
    fn add_pk_column(
        mut self,
        pk_column_name: &ColumnName,
        value: SqlValue,
    ) -> ApllodbResult<Self> {
        if let Some(_) = self.pk_columns.insert(pk_column_name.clone(), value) {
            Err(ApllodbError::new(
                ApllodbErrorKind::DuplicateColumn,
                format!(
                    "column `{}` is already added to this record",
                    pk_column_name
                ),
                None,
            ))
        } else {
            Ok(self)
        }
    }

    fn to_full_pk(self) -> FullPrimaryKey {
        let mut pk_column_names: Vec<ColumnName> = vec![];
        let mut pk_sql_values: Vec<SqlValue> = vec![];

        for (pk_column_name, pk_sql_value) in self.pk_columns {
            pk_column_names.push(pk_column_name);
            pk_sql_values.push(pk_sql_value);
        }

        let apparent_pk = ApparentPrimaryKey::new(pk_column_names, pk_sql_values);
        FullPrimaryKey::new(apparent_pk, self.revision)
    }
}
impl Default for PKBuilder {
    fn default() -> Self {
        Self {
            pk_columns: HashMap::new(),
            revision: Revision::initial(),
        }
    }
}

impl ImmutableRowBuilder {
    /// Add PK column to row.
    ///
    /// # Failures
    ///
    /// - [DuplicateColumn](error/enum.ApllodbErrorKind.html#variant.DuplicateColumn) when:
    ///   - Same `ColumnName` added twice.
    pub fn add_pk_column(
        mut self,
        pk_column_name: &ColumnName,
        value: SqlValue,
    ) -> ApllodbResult<Self> {
        self.pk_builder = self.pk_builder.add_pk_column(pk_column_name, value)?;
        Ok(self)
    }

    /// Add non-PK column to row.
    ///
    /// # Failures
    ///
    /// - [DuplicateColumn](error/enum.ApllodbErrorKind.html#variant.DuplicateColumn) when:
    ///   - Same `ColumnName` added twice.
    pub fn add_non_pk_column(
        mut self,
        non_pk_column_name: &ColumnName,
        value: SqlValue,
    ) -> ApllodbResult<Self> {
        if let Some(_) = self
            .non_pk_columns
            .insert(non_pk_column_name.clone(), value)
        {
            Err(ApllodbError::new(
                ApllodbErrorKind::DuplicateColumn,
                format!(
                    "column `{}` is already added to this record",
                    non_pk_column_name
                ),
                None,
            ))
        } else {
            Ok(self)
        }
    }

    pub fn add_non_pk_void_projection(
        self,
        non_pk_column_name: &ColumnName,
    ) -> ApllodbResult<Self> {
        let null = SqlValue::null();
        self.add_non_pk_column(non_pk_column_name, null)
    }

    /// Finalize.
    ///
    /// TODO validate duplicate column name.
    pub fn build(self) -> ApllodbResult<ImmutableRow> {
        Ok(ImmutableRow {
            pk: self.pk_builder.to_full_pk(),
            non_pk_columns: self.non_pk_columns,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::ImmutableRowBuilder;
    use crate::test_support::setup;
    use apllodb_shared_components::{
        data_structure::ColumnName,
        data_structure::{DataType, DataTypeKind, SqlValue},
        error::ApllodbResult,
    };
    use apllodb_storage_engine_interface::Row;

    #[test]
    fn test_success() -> ApllodbResult<()> {
        setup();

        let row1 = ImmutableRowBuilder::default()
            .add_pk_column(
                &ColumnName::new("c1")?,
                SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &0i32)?,
            )?
            .add_non_pk_column(
                &ColumnName::new("c2")?,
                SqlValue::pack(&DataType::new(DataTypeKind::Integer, true), &None::<i32>)?,
            )?
            .build()?;

        let row2 = ImmutableRowBuilder::default()
            .add_non_pk_column(
                &ColumnName::new("c2")?,
                SqlValue::pack(&DataType::new(DataTypeKind::Integer, true), &None::<i32>)?,
            )?
            .add_pk_column(
                &ColumnName::new("c1")?,
                SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &0i32)?,
            )?
            .build()?;

        assert_eq!(row1, row2);

        Ok(())
    }

    #[test]
    fn test_diff_revision_has_same_apparent_pk() -> ApllodbResult<()> {
        setup();

        let row1 = ImmutableRowBuilder::default()
            .add_pk_column(
                &ColumnName::new("c1")?,
                SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &0i32)?,
            )?
            .build()?;
        let row2 = ImmutableRowBuilder::default()
            .add_pk_column(
                &ColumnName::new("c1")?,
                SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &0i32)?,
            )?
            .build()?;

        assert_eq!(row1.pk(), row2.pk());

        Ok(())
    }

    #[test]
    fn test_compound_pk() -> ApllodbResult<()> {
        setup();

        let row1 = ImmutableRowBuilder::default()
            .add_pk_column(
                &ColumnName::new("c1")?,
                SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &0i32)?,
            )?
            .add_pk_column(
                &ColumnName::new("c2")?,
                SqlValue::pack(&DataType::new(DataTypeKind::Integer, true), &None::<i32>)?,
            )?
            .build()?;

        let row2 = ImmutableRowBuilder::default()
            .add_pk_column(
                &ColumnName::new("c2")?,
                SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &0i32)?,
            )?
            .add_pk_column(
                &ColumnName::new("c1")?,
                SqlValue::pack(&DataType::new(DataTypeKind::Integer, true), &None::<i32>)?,
            )?
            .build()?;

        assert_ne!(row1, row2);

        Ok(())
    }
}
