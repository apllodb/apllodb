pub mod builder;

use apllodb_shared_components::{
    ApllodbError, ApllodbErrorKind, ApllodbResult, FullFieldReference, Record, SqlConvertible,
    SqlValue,
};
use apllodb_storage_engine_interface::TableColumnReference;
use std::collections::{hash_map::Entry, HashMap};

/// Immutable row which is never updated or deleted by any transaction.
/// Only used for SELECT statement (or internally for UPDATE == SELECT + INSERT).
#[derive(Clone, PartialEq, Debug)]
pub struct ImmutableRow {
    col_vals: HashMap<TableColumnReference, SqlValue>,  // これ普通にFFRもてばいいかも
    // TODO have TransactionId to enable time-machine (TODO naming...) feature.
}

impl ImmutableRow {
    /// Retrieve (and remove) an [NNSqlValue](apllodb_shared_components::NNSqlValue) from this row.
    ///
    /// # Failures
    ///
    /// - [UndefinedColumn](apllodb-shared-components::ApllodbErrorKind::UndefinedColumn) when:
    ///   - Specified column does not exist in this row.
    pub fn get_sql_value(
        &mut self,
        table_column_reference: &TableColumnReference,  // FieldIndexがいいかな
    ) -> ApllodbResult<SqlValue> {
        self.col_vals
            .remove(&table_column_reference)
            .ok_or_else(|| {
                ApllodbError::new(
                    ApllodbErrorKind::UndefinedColumn,
                    format!("undefined column: `{:?}`", table_column_reference),
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
    pub fn get<T: SqlConvertible>(
        &mut self,
        table_column_reference: &TableColumnReference,
    ) -> ApllodbResult<Option<T>> {
        let sql_value = self.get_sql_value(table_column_reference)?;
        match sql_value {
            SqlValue::Null => Ok(None),
            SqlValue::NotNull(nn) => {
                let v = nn.unpack().or_else(|e| {
                    // write back removed value into row
                    self.append(table_column_reference.clone(), SqlValue::NotNull(nn))?;
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
        table_column_reference: TableColumnReference,
        sql_value: SqlValue,
    ) -> ApllodbResult<()> {
        match self.col_vals.entry(table_column_reference.clone()) {
            Entry::Occupied(_) => Err(ApllodbError::new(
                ApllodbErrorKind::DuplicateColumn,
                format!("column `{}` is already in this row", table_column_reference),
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
    pub fn into_col_vals(self) -> HashMap<TableColumnReference, SqlValue> {
        self.col_vals
    }
}

impl Into<Record> for ImmutableRow {
    fn into(self) -> Record {
        let mut col_vals = self.col_vals;
        let fields: HashMap<FullFieldReference, SqlValue> = col_vals
            .drain()
            .map(|(tcr, sql_value)| (FullFieldReference::from(tcr), sql_value))
            .collect();
        Record::new(fields)
    }
}
