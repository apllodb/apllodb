use crate::{row::pk::Revision, ApparentPrimaryKey, FullPrimaryKey, ImmutableRow};
use apllodb_shared_components::{
    data_structure::{ColumnName, SqlValue},
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use std::collections::{hash_map::Entry, HashMap};

/// Builder for ImmutableRow.
#[derive(Debug, Default)]
pub struct ImmutableRowBuilder {
    columns: HashMap<ColumnName, SqlValue>,
    pk_builder: PKBuilder,
}

#[derive(Debug)]
struct PKBuilder {
    column_names: Vec<ColumnName>,
    revision: Revision,
}
impl PKBuilder {
    fn to_full_pk(self, sql_values: Vec<SqlValue>) -> FullPrimaryKey {
        let apparent_pk = ApparentPrimaryKey::new(self.column_names, sql_values);
        FullPrimaryKey::new(apparent_pk, self.revision)
    }
}
impl Default for PKBuilder {
    fn default() -> Self {
        Self {
            column_names: vec![],
            revision: Revision::initial(),
        }
    }
}

impl ImmutableRowBuilder {
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

    /// Mark PK column and determine revision.
    pub fn mark_pk(mut self, column_names: Vec<ColumnName>, revision: Revision) -> Self {
        self.pk_builder = PKBuilder {
            column_names,
            revision,
        };
        self
    }

    /// Finalize.
    ///
    /// TODO validate duplicate column name.
    pub fn build(self) -> ApllodbResult<ImmutableRow> {
        let pk_column_names = &self.pk_builder.column_names;
        let mut columns = self.columns;

        let sql_values = pk_column_names
            .iter()
            .map(|pk_c| {
                let entry = columns.entry(pk_c.clone());
                if let Entry::Occupied(oe) = entry {
                    Ok(oe.remove())
                } else {
                    Err(ApllodbError::new(
                        ApllodbErrorKind::UndefinedColumn,
                        format!("specified undefined column for PK: {:?}", pk_c),
                        None,
                    ))
                }
            })
            .collect::<ApllodbResult<Vec<SqlValue>>>()?;

        Ok(ImmutableRow {
            pk: self.pk_builder.to_full_pk(sql_values),
            columns,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::ImmutableRowBuilder;
    use crate::row::pk::Revision;
    use apllodb_shared_components::{
        data_structure::{ColumnName, DataType, DataTypeKind, SqlValue},
        error::ApllodbResult,
    };
    use apllodb_storage_engine_interface::Row;

    #[test]
    fn test_success() -> ApllodbResult<()> {
        let row1 = ImmutableRowBuilder::default()
            .add_column(
                &ColumnName::new("c1")?,
                SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &0i32)?,
            )?
            .mark_pk(vec![ColumnName::new("c1")?], Revision::initial())
            .add_column(
                &ColumnName::new("c2")?,
                SqlValue::pack(&DataType::new(DataTypeKind::Integer, true), &None::<i32>)?,
            )?
            .build()?;

        let row2 = ImmutableRowBuilder::default()
            .mark_pk(vec![ColumnName::new("c1")?], Revision::initial())
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

    #[test]
    fn test_diff_revision_has_same_apparent_pk() -> ApllodbResult<()> {
        let row1 = ImmutableRowBuilder::default()
            .add_column(
                &ColumnName::new("c1")?,
                SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &0i32)?,
            )?
            .mark_pk(vec![ColumnName::new("c1")?], Revision::initial())
            .build()?;
        let row2 = ImmutableRowBuilder::default()
            .add_column(
                &ColumnName::new("c1")?,
                SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &0i32)?,
            )?
            .mark_pk(vec![ColumnName::new("c1")?], Revision::initial().next())
            .build()?;

        assert_eq!(row1.pk(), row2.pk());

        Ok(())
    }

    #[test]
    fn test_compound_pk() -> ApllodbResult<()> {
        let row1 = ImmutableRowBuilder::default()
            .add_column(
                &ColumnName::new("c1")?,
                SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &0i32)?,
            )?
            .add_column(
                &ColumnName::new("c2")?,
                SqlValue::pack(&DataType::new(DataTypeKind::Integer, true), &None::<i32>)?,
            )?
            .mark_pk(
                vec![ColumnName::new("c1")?, ColumnName::new("c2")?],
                Revision::initial(),
            )
            .build()?;

        let row2 = ImmutableRowBuilder::default()
            .add_column(
                &ColumnName::new("c1")?,
                SqlValue::pack(&DataType::new(DataTypeKind::Integer, true), &None::<i32>)?,
            )?
            .add_column(
                &ColumnName::new("c2")?,
                SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &0i32)?,
            )?
            .mark_pk(
                vec![ColumnName::new("c2")?, ColumnName::new("c1")?],
                Revision::initial(),
            )
            .build()?;

        assert_ne!(row1, row2);

        Ok(())
    }
}
