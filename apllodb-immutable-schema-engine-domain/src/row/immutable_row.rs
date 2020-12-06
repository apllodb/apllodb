pub mod builder;

use apllodb_shared_components::{
    data_structure::{ColumnReference, ColumnValue, SqlValue},
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use apllodb_storage_engine_interface::Row;
use std::collections::{hash_map::Entry, HashMap};

/// Immutable row which is never updated or deleted by any transaction.
/// Only used for SELECT statement (or internally for UPDATE == SELECT + INSERT).
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ImmutableRow {
    col_vals: HashMap<ColumnReference, SqlValue>,
    // TODO have TransactionId to enable time-machine (TODO naming...) feature.
}

impl Row for ImmutableRow {
    fn get_core(&self, colref: &ColumnReference) -> ApllodbResult<&SqlValue> {
        self.col_vals.get(&colref).ok_or_else(|| {
            ApllodbError::new(
                ApllodbErrorKind::UndefinedColumn,
                format!("undefined column: `{}`", colref),
                None,
            )
        })
    }

    fn append(&mut self, colvals: Vec<ColumnValue>) -> ApllodbResult<()> {
        colvals
            .into_iter()
            .map(
                |colval| match self.col_vals.entry(colval.as_column_ref().clone()) {
                    Entry::Occupied(_) => Err(ApllodbError::new(
                        ApllodbErrorKind::DuplicateColumn,
                        format!("column `{}` is already in this row", colval.as_column_ref()),
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
