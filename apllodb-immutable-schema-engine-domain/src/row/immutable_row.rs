pub mod builder;

use apllodb_shared_components::{
    ApllodbError, ApllodbErrorKind, ApllodbResult, ColumnValue, FieldIndex, FullFieldReference,
    Record, SqlConvertible, SqlValue,
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
    pub fn get_sql_value(&mut self, ffr: &FullFieldReference) -> ApllodbResult<SqlValue> {
        self.col_vals.remove(&ffr).ok_or_else(|| {
            ApllodbError::new(
                ApllodbErrorKind::UndefinedColumn,
                format!("undefined column: `{:?}`", ffr),
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
    ///   - `column_name` is not in this row.
    pub fn get<T: SqlConvertible>(
        &mut self,
        colref: &FullFieldReference,
    ) -> ApllodbResult<Option<T>> {
        let sql_value = self.get_sql_value(colref)?;
        match sql_value {
            SqlValue::Null => Ok(None),
            SqlValue::NotNull(nn) => {
                let v = nn.unpack().or_else(|e| {
                    // write back removed value into row
                    let colval = ColumnValue::new(colref.clone(), SqlValue::NotNull(nn));
                    self.append(vec![colval])?;
                    Err(e)
                })?;
                Ok(Some(v))
            }
        }
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
                |colval| match self.col_vals.entry(colval.as_column_name().clone()) {
                    Entry::Occupied(_) => Err(ApllodbError::new(
                        ApllodbErrorKind::DuplicateColumn,
                        format!(
                            "column `{:?}` is already in this row",
                            colval.as_column_name()
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
    pub fn into_col_vals(self) -> HashMap<FullFieldReference, SqlValue> {
        self.col_vals
    }
}

impl Into<Record> for ImmutableRow {
    fn into(self) -> Record {
        let mut col_vals = self.col_vals;
        let fields: HashMap<FieldIndex, SqlValue> = col_vals
            .drain()
            .map(|(colref, sql_value)| (FieldIndex::InFullFieldReference(colref), sql_value))
            .collect();
        Record::new(fields)
    }
}
