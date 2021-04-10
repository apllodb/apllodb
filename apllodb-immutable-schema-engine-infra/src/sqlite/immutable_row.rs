use apllodb_immutable_schema_engine_domain::row::immutable_row::{
    builder::ImmutableRowBuilder, ImmutableRow,
};
use apllodb_shared_components::{
    ApllodbResult, I64LooseType, NumericComparableType, SqlConvertible, SqlType, SqlValue,
    StringComparableLoseType,
};
use apllodb_storage_engine_interface::{ColumnDataType, ColumnName, TableName};
use sqlx::Row;

use crate::error::InfraError;

pub(crate) trait FromSqliteRow {
    fn from_sqlite_row(
        sqlite_row: &sqlx::sqlite::SqliteRow,
        table_name: &TableName,
        column_data_types: &[&ColumnDataType],
        void_projections: &[ColumnName],
    ) -> ApllodbResult<ImmutableRow> {
        let mut builder = ImmutableRowBuilder::new(table_name.clone());

        for cdt in column_data_types {
            let non_pk_sql_value = Self::_sql_value(sqlite_row, cdt)?;
            builder = builder.append(cdt.column_name().clone(), non_pk_sql_value)?;
        }

        // add requested (specified in projection) columns as NULL.
        // (E.g. v1 has `c1` and v2 does not. This row is for v2 and `c1` is requested.)
        for non_pk_void_projection in void_projections {
            builder = builder.add_void_projection(non_pk_void_projection.clone())?;
        }

        let row = builder.build()?;
        Ok(row)
    }

    fn _sql_value(
        sqlite_row: &sqlx::sqlite::SqliteRow,
        column_data_type: &ColumnDataType,
    ) -> ApllodbResult<SqlValue> {
        let sql_value = match column_data_type.sql_type() {
            SqlType::NumericComparable(n) => match n {
                NumericComparableType::I64Loose(i) => match i {
                    I64LooseType::SmallInt => {
                        Self::_sqlite_row_value::<i16>(sqlite_row, column_data_type)?
                    }
                    I64LooseType::Integer => {
                        Self::_sqlite_row_value::<i32>(sqlite_row, column_data_type)?
                    }
                    I64LooseType::BigInt => {
                        Self::_sqlite_row_value::<i64>(sqlite_row, column_data_type)?
                    }
                },
            },
            SqlType::StringComparableLoose(s) => match s {
                StringComparableLoseType::Text => {
                    Self::_sqlite_row_value::<String>(sqlite_row, column_data_type)?
                }
            },
            SqlType::BooleanComparable => {
                Self::_sqlite_row_value::<bool>(sqlite_row, column_data_type)?
            }
        };

        Ok(sql_value)
    }

    fn _sqlite_row_value<'r, T>(
        sqlite_row: &'r sqlx::sqlite::SqliteRow,
        column_data_type: &ColumnDataType,
    ) -> ApllodbResult<SqlValue>
    where
        T: sqlx::Decode<'r, sqlx::sqlite::Sqlite>
            + sqlx::Type<sqlx::sqlite::Sqlite>
            + SqlConvertible,
    {
        let rust_value: Option<T> = sqlite_row
            .try_get(column_data_type.column_name().as_str())
            .map_err(InfraError::from)?;

        let sql_value = if let Some(rust_value) = rust_value {
            let nn_sql_value = rust_value.into_sql_value();
            debug_assert_eq!(column_data_type.sql_type(), &nn_sql_value.sql_type());
            SqlValue::NotNull(nn_sql_value)
        } else {
            SqlValue::Null
        };

        Ok(sql_value)
    }
}

impl FromSqliteRow for ImmutableRow {}
