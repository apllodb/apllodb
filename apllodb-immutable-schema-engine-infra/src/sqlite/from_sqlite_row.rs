use apllodb_immutable_schema_engine_domain::row::immutable_row::{
    builder::ImmutableRowBuilder, ImmutableRow,
};
use apllodb_shared_components::{
    ApllodbResult, ColumnDataType, ColumnReference, ColumnValue, I64LooseType,
    NumericComparableType, SqlConvertible, SqlType, SqlValue, StringComparableLoseType,
};
use sqlx::Row;

use crate::error::InfraError;

pub(crate) trait FromSqliteRow {
    fn from_sqlite_row(
        sqlite_row: &sqlx::sqlite::SqliteRow,
        column_data_types: &[&ColumnDataType],
        void_projections: &[ColumnReference],
    ) -> ApllodbResult<ImmutableRow> {
        let mut builder = ImmutableRowBuilder::default();

        for cdt in column_data_types {
            let non_pk_colref = cdt.column_ref();
            let non_pk_sql_value = Self::_sql_value(sqlite_row, cdt)?;
            builder =
                builder.add_colval(ColumnValue::new(non_pk_colref.clone(), non_pk_sql_value))?;
        }

        // add requested (specified in projection) columns as NULL.
        // (E.g. v1 has `c1` and v2 does not. This row is for v2 and `c1` is requested.)
        for non_pk_void_projection in void_projections {
            builder = builder.add_void_projection(non_pk_void_projection)?;
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
        let colref = column_data_type.column_ref();
        let sql_type = column_data_type.sql_type();

        let rust_value: Option<T> = sqlite_row
            .try_get(colref.as_column_name().as_str())
            .map_err(InfraError::from)?;

        let sql_value = if let Some(rust_value) = rust_value {
            SqlValue::pack(sql_type.clone(), &rust_value)?
        } else {
            SqlValue::Null
        };

        Ok(sql_value)
    }
}

impl FromSqliteRow for ImmutableRow {}
