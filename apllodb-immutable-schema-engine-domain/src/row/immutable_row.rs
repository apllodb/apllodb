pub mod builder;

use apllodb_shared_components::{
    ApllodbError, ApllodbErrorKind, ApllodbResult, ColumnReference, ColumnValue, FieldIndex,
    Record, SqlConvertible, SqlValue,
};
use std::collections::{hash_map::Entry, HashMap};

/// Immutable row which is never updated or deleted by any transaction.
/// Only used for SELECT statement (or internally for UPDATE == SELECT + INSERT).
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ImmutableRow {
    col_vals: HashMap<ColumnReference, SqlValue>,
    // TODO have TransactionId to enable time-machine (TODO naming...) feature.
}

impl ImmutableRow {
    /// Retrieve (and remove) an [SqlValue](apllodb_shared_components::SqlValue) from this row.
    pub fn get_sql_value(&mut self, colref: &ColumnReference) -> ApllodbResult<SqlValue> {
        self.col_vals.remove(&colref).ok_or_else(|| {
            ApllodbError::new(
                ApllodbErrorKind::UndefinedColumn,
                format!("undefined column: `{:?}`", colref),
                None,
            )
        })
    }

    /// Retrieve (and remove) an SqlValue from this row and return it as Rust type.
    ///
    /// # Failures
    ///
    /// - [UndefinedColumn](apllodb_shared_components::ApllodbErrorKind::UndefinedColumn) when:
    ///   - `column_name` is not in this row.
    pub fn get<T: SqlConvertible>(&mut self, colref: &ColumnReference) -> ApllodbResult<T> {
        let sql_value = self.get_sql_value(colref)?;
        sql_value.unpack().or_else(|e| {
            // write back removed value into row
            let colval = ColumnValue::new(colref.clone(), sql_value);
            self.append(vec![colval])?;
            Err(e)
        })
    }

    /// Append column values to this row.
    ///
    /// # Failures
    ///
    /// - [DuplicateColumn](apllodb_shared_components::ApllodbErrorKind::DuplicateColumn) when:
    ///   - Same [ColumnReference](apllodb_shared_components::ColumnReference) is already in this row.
    pub fn append(&mut self, colvals: Vec<ColumnValue>) -> ApllodbResult<()> {
        colvals
            .into_iter()
            .map(
                |colval| match self.col_vals.entry(colval.as_column_ref().clone()) {
                    Entry::Occupied(_) => Err(ApllodbError::new(
                        ApllodbErrorKind::DuplicateColumn,
                        format!(
                            "column `{:?}` is already in this row",
                            colval.as_column_ref()
                        ),
                        None,
                    )),
                    Entry::Vacant(e) => {
                        e.insert(colval.into_sql_value());
                        Ok(())
                    }
                },
            )
            .collect::<ApllodbResult<Vec<()>>>()?;

        Ok(())
    }
}

impl ImmutableRow {
    pub fn into_col_vals(self) -> HashMap<ColumnReference, SqlValue> {
        self.col_vals
    }
}

impl Into<Record> for ImmutableRow {
    fn into(self) -> Record {
        let mut col_vals = self.col_vals;
        let fields: HashMap<FieldIndex, SqlValue> = col_vals
            .drain()
            .map(|(colref, sql_value)| (FieldIndex::InColumnReference(colref), sql_value))
            .collect();
        Record::new(fields)
    }
}
