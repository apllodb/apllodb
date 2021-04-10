use apllodb_shared_components::{
    ApllodbResult, I64LooseType, NumericComparableType, SqlConvertible, SqlType, SqlValue,
    StringComparableLoseType,
};
use apllodb_storage_engine_interface::{
    ColumnDataType, ColumnName, Row, RowSchema, Rows, TableColumnName, TableName,
};

use crate::error::InfraError;

pub(crate) trait FromSqliteRows {
    /// # Arguments
    ///
    /// - `non_pk_column_data_types` - Only contains columns `sqlite_rows` have.
    /// - `non_pk_void_projection` - Columns `sqlite_rows` do not have but another version has.
    fn from_sqlite_rows(
        sqlite_rows: &[sqlx::sqlite::SqliteRow],
        table_name: &TableName,
        column_data_types: &[&ColumnDataType],
        void_projection: &[ColumnName],
    ) -> ApllodbResult<Rows> {
        let schema = RowSchema::from(
            column_data_types
                .iter()
                .map(|cdt| cdt.column_name())
                .chain(void_projection.iter())
                .map(|cn| TableColumnName::new(table_name.clone(), cn.clone()))
                .collect::<Vec<TableColumnName>>(),
        );

        let rows: Vec<Row> = sqlite_rows
            .iter()
            .map(|sqlite_row| {
                let mut sql_values: Vec<SqlValue> = column_data_types
                    .iter()
                    .map(|cdt| Self::_sql_value(sqlite_row, cdt))
                    .collect::<ApllodbResult<_>>()?;

                // add requested (specified in projection) columns as NULL.
                // (E.g. v1 has `c1` and v2 does not. This row is for v2 and `c1` is requested.)
                void_projection
                    .iter()
                    .for_each(|_| sql_values.push(SqlValue::Null));

                Ok(Row::new(sql_values))
            })
            .collect::<ApllodbResult<_>>()?;

        Ok(Rows::new(schema, rows))
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
        use sqlx::Row;

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

impl FromSqliteRows for Rows {}
