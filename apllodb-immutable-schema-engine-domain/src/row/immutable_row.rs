pub mod builder;

use super::{
    column::non_pk_column::column_name::NonPKColumnName,
    pk::{apparent_pk::ApparentPrimaryKey, full_pk::FullPrimaryKey},
};
use apllodb_shared_components::{
    data_structure::{ColumnName, SqlValue},
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use apllodb_storage_engine_interface::Row;
use std::collections::HashMap;

/// Immutable row who is never updated or deleted by any transaction.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ImmutableRow {
    pk: FullPrimaryKey,
    non_pk_columns: HashMap<NonPKColumnName, SqlValue>,
    // TODO have TransactionId to enable time-machine (TODO naming...) feature.
}

impl Row for ImmutableRow {
    type PK = ApparentPrimaryKey;

    fn pk(&self) -> &Self::PK {
        self.pk.apparent_pk()
    }

    fn get_core(&self, non_pk_column_name: &ColumnName) -> ApllodbResult<&SqlValue> {
        let non_pk_column_name = NonPKColumnName::from(non_pk_column_name.clone());
        let sql_value = self
            .non_pk_columns
            .get(&non_pk_column_name)
            .ok_or_else(|| {
                ApllodbError::new(
                    ApllodbErrorKind::UndefinedColumn,
                    format!("undefined column name: `{}`", non_pk_column_name),
                    None,
                )
            })?;
        Ok(sql_value)
    }
}
