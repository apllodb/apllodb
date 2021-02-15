use crate::row_iter::version_row_iter::row_column_ref_schema::RowColumnRefSchema;

use super::ImmutableRow;

use apllodb_shared_components::{
    ApllodbError, ApllodbErrorKind, ApllodbResult, ColumnName, SqlValue, SqlValues, TableName,
};

/// Builder for ImmutableRow.
#[derive(Debug)]
pub struct ImmutableRowBuilder {
    table_name: TableName,
    col_vals: Vec<(ColumnName, SqlValue)>,
}

impl ImmutableRowBuilder {
    /// Constructor
    pub fn new(table_name: TableName) -> Self {
        Self {
            table_name,
            col_vals: Vec::new(),
        }
    }

    /// Add column value to row.
    ///
    /// # Failures
    ///
    /// - [DuplicateColumn](apllodb_shared_components::ApllodbErrorKind::DuplicateColumn) when:
    ///   - Same `ColumnName` added twice.
    pub fn append(mut self, column_name: ColumnName, sql_value: SqlValue) -> ApllodbResult<Self> {
        if self.col_vals.iter().any(|(cn, _)| cn == &column_name) {
            Err(ApllodbError::new(
                ApllodbErrorKind::DuplicateColumn,
                format!(
                    "column `{}` is already added to this row",
                    column_name.as_str()
                ),
                None,
            ))
        } else {
            self.col_vals.push((column_name, sql_value));
            Ok(self)
        }
    }

    pub fn add_void_projection(self, column_name: ColumnName) -> ApllodbResult<Self> {
        self.append(column_name, SqlValue::Null)
    }

    /// Finalize.
    pub fn build(self) -> ApllodbResult<ImmutableRow> {
        let column_names: Vec<ColumnName> =
            self.col_vals.iter().map(|(cn, _)| cn.clone()).collect();
        let sql_values: SqlValues = {
            let s = self
                .col_vals
                .into_iter()
                .map(|(_, sql_value)| sql_value)
                .collect();
            SqlValues::new(s)
        };

        let schema = RowColumnRefSchema::new(self.table_name, column_names);

        Ok(ImmutableRow {
            schema,
            values: sql_values,
        })
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::ImmutableRowBuilder;
    use apllodb_shared_components::{ApllodbResult, ColumnName, NNSqlValue, SqlValue, TableName};

    #[test]
    #[allow(clippy::redundant_clone)]
    fn test_success() -> ApllodbResult<()> {
        let cn = ColumnName::factory("c1");

        let mut row1 = ImmutableRowBuilder::new(TableName::factory("t"))
            .append(cn.clone(), SqlValue::NotNull(NNSqlValue::Integer(0)))?
            .build()?;
        let mut row2 = ImmutableRowBuilder::new(TableName::factory("t"))
            .append(cn.clone(), SqlValue::NotNull(NNSqlValue::Integer(0)))?
            .build()?;

        assert_eq!(row1.get::<i32>(&cn.clone())?, row2.get::<i32>(&cn.clone())?);

        Ok(())
    }
}
