pub mod builder;

use apllodb_shared_components::{
    data_structure::{ColumnReference, SqlValue},
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use apllodb_storage_engine_interface::Row;
use std::collections::HashMap;

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
}
