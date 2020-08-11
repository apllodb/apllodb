pub mod builder;

use super::{
    column::{non_pk_column::column_name::NonPKColumnName, pk_column::column_name::PKColumnName},
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

    fn get_core(&self, column_name: &ColumnName) -> ApllodbResult<&SqlValue> {
        self.get_from_pk(&PKColumnName::from(column_name.clone()))
            .or_else(|| self.get_from_non_pk(&NonPKColumnName::from(column_name.clone())))
            .ok_or_else(|| {
                ApllodbError::new(
                    ApllodbErrorKind::UndefinedColumn,
                    format!("undefined column name: `{}`", column_name),
                    None,
                )
            })
    }
}

impl ImmutableRow {
    fn get_from_pk(&self, pk_column_name: &PKColumnName) -> Option<&SqlValue> {
        let apk = self.pk.apparent_pk();
        apk.zipped().into_iter().find_map(|(cn, sql_value)| {
            if cn == pk_column_name {
                Some(sql_value)
            } else {
                None
            }
        })
    }

    fn get_from_non_pk(&self, non_pk_column_name: &NonPKColumnName) -> Option<&SqlValue> {
        self.non_pk_columns.get(&non_pk_column_name)
    }
}
