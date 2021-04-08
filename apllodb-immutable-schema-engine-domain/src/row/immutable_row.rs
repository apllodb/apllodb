pub mod builder;

use apllodb_shared_components::{ApllodbResult, ColumnName, SqlConvertible, SqlValue, SqlValues};

use crate::row_iter::version_row_iter::row_column_ref_schema::RowColumnRefSchema;

/// Immutable row which is never updated or deleted by any transaction.
/// Only used for SELECT statement (or internally for UPDATE == SELECT + INSERT).
#[derive(Clone, PartialEq, Debug)]
pub struct ImmutableRow {
    schema: RowColumnRefSchema, // TODO wrap into Arc, own raw data in ImmutableRowIterator
    values: SqlValues,
    // TODO have TransactionId to enable time-machine (TODO naming...) feature.
}

impl ImmutableRow {
    /// Retrieve (and remove) an [SqlValue](apllodb_shared_components::SqlValue) from this row.
    ///
    /// # Failures
    ///
    /// - [UndefinedColumn](apllodb-shared-components::ApllodbErrorKind::UndefinedColumn) when:
    ///   - Specified column does not exist in this row.
    pub fn get_sql_value(&mut self, column_name: &ColumnName) -> ApllodbResult<SqlValue> {
        let pos = self.schema.resolve_pos_with_rm(column_name)?;
        Ok(self.values.remove(pos))
    }

    /// Retrieve (and remove) an SqlValue from this row and return it as Rust type.
    ///
    /// Returns `None` if matching [SqlValue](apllodb_shared_components::SqlValue) is NULL.
    ///
    /// # Failures
    ///
    /// - [UndefinedColumn](apllodb_shared_components::ApllodbErrorKind::UndefinedColumn) when:
    ///   - `table_column_reference` is not in this row.
    pub fn get<T: SqlConvertible>(&mut self, column_name: &ColumnName) -> ApllodbResult<Option<T>> {
        let sql_value = self.get_sql_value(column_name)?;

        match sql_value {
            SqlValue::Null => Ok(None),
            SqlValue::NotNull(nn) => {
                let v = nn.unpack().or_else(|e| {
                    // write back removed value into row
                    self.append(column_name.clone(), SqlValue::NotNull(nn))?;
                    Err(e)
                })?;
                Ok(Some(v))
            }
        }
    }

    /// Append a column value to this row.
    ///
    /// # Failures
    ///
    /// - [DuplicateColumn](apllodb_shared_components::ApllodbErrorKind::DuplicateColumn) when:
    ///   - Same [ColumnName](apllodb_shared_components::ColumnName) is already in this row.
    pub fn append(&mut self, column_name: ColumnName, sql_value: SqlValue) -> ApllodbResult<()> {
        self.schema.append(column_name)?;
        self.values.append(sql_value);
        Ok(())
    }

    pub fn into_zipped(self) -> Vec<(ColumnName, SqlValue)> {
        self.schema
            .into_column_names()
            .into_iter()
            .zip(self.values)
            .collect()
    }

    pub fn schema(&self) -> &RowColumnRefSchema {
        &self.schema
    }
}
