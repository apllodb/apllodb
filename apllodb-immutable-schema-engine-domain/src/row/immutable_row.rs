mod builder;

pub use builder::ImmutableRowBuilder;

use crate::{ApparentPrimaryKey, FullPrimaryKey};
use apllodb_shared_components::{
    data_structure::{ColumnName, SqlValue},
    error::{ApllodbError, ApllodbResult, ApllodbErrorKind},
};
use apllodb_storage_engine_interface::Row;
use std::collections::HashMap;

/// Immutable row who is never updated or deleted by any transaction.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ImmutableRow {
    pk: FullPrimaryKey,

    // Columns not including PK.
    columns: HashMap<ColumnName, SqlValue>,
    // TODO have TransactionId to enable time-machine (TODO naming...) feature.
}

impl Row for ImmutableRow {
    type PK = ApparentPrimaryKey;

    fn pk(&self) -> &Self::PK {
        self.pk.apparent_pk()
    }

    fn get_core(&self, column_name: &ColumnName) -> ApllodbResult<&SqlValue> {
        let sql_value = self.columns.get(column_name).ok_or_else(|| {
            ApllodbError::new(
                ApllodbErrorKind::UndefinedColumn,
                format!("undefined column name: `{:?}`", column_name),
                None,
            )
        })?;
        Ok(sql_value)
    }
}
