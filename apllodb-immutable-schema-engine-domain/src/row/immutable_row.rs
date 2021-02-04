pub mod builder;

use apllodb_shared_components::{
    ApllodbError, ApllodbErrorKind, ApllodbResult, FieldIndex, FullFieldReference, SqlConvertible,
    SqlValue,
};
use std::collections::{hash_map::Entry, HashMap};

/// Immutable row which is never updated or deleted by any transaction.
/// Only used for SELECT statement (or internally for UPDATE == SELECT + INSERT).
#[derive(Clone, PartialEq, Debug)]
pub struct ImmutableRow {
    col_vals: HashMap<FullFieldReference, SqlValue>,
    // TODO have TransactionId to enable time-machine (TODO naming...) feature.
}

impl ImmutableRow {
    /// Retrieve (and remove) an [NNSqlValue](apllodb_shared_components::NNSqlValue) from this row.
    ///
    /// # Failures
    ///
    /// - [UndefinedColumn](apllodb-shared-components::ApllodbErrorKind::UndefinedColumn) when:
    ///   - Specified column does not exist in this row.
    pub fn get_sql_value(&mut self, field_index: &FieldIndex) -> ApllodbResult<SqlValue> {
        let ffr = field_index.peek(self.col_vals.keys())?.1.clone();
        self.col_vals.remove(&ffr).ok_or_else(|| {
            ApllodbError::new(
                ApllodbErrorKind::UndefinedColumn,
                format!("undefined column: `{:?}`", field_index),
                None,
            )
        })
    }

    /// Retrieve (and remove) an SqlValue from this row and return it as Rust type.
    ///
    /// Returns `None` if matching [SqlValue](apllodb_shared_components::SqlValue) is NULL.
    ///
    /// # Failures
    ///
    /// - [UndefinedColumn](apllodb_shared_components::ApllodbErrorKind::UndefinedColumn) when:
    ///   - `table_column_reference` is not in this row.
    pub fn get<T: SqlConvertible>(&mut self, field_index: &FieldIndex) -> ApllodbResult<Option<T>> {
        let ffr = field_index.peek(self.col_vals.keys())?.1.clone();
        let sql_value = self.get_sql_value(field_index)?;

        match sql_value {
            SqlValue::Null => Ok(None),
            SqlValue::NotNull(nn) => {
                let v = nn.unpack().or_else(|e| {
                    // write back removed value into row
                    self.append(ffr.clone(), SqlValue::NotNull(nn))?;
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
    ///   - Same [ColumnReference](apllodb_shared_components::ColumnReference) is already in this row.
    pub fn append(
        &mut self,
        full_field_reference: FullFieldReference,
        sql_value: SqlValue,
    ) -> ApllodbResult<()> {
        match self.col_vals.entry(full_field_reference.clone()) {
            Entry::Occupied(_) => Err(ApllodbError::new(
                ApllodbErrorKind::DuplicateColumn,
                format!("column `{}` is already in this row", full_field_reference),
                None,
            )),
            Entry::Vacant(e) => {
                e.insert(sql_value);
                Ok(())
            }
        }
    }
}

impl ImmutableRow {
    pub fn into_col_vals(self) -> HashMap<FullFieldReference, SqlValue> {
        self.col_vals
    }
}
