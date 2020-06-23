use super::sqlite_error::map_sqlite_err;
use apllodb_immutable_schema_engine_domain::{ImmutableRow, ImmutableRowBuilder};
use apllodb_shared_components::{
    data_structure::{ColumnDataType, DataTypeKind, SqlValue},
    error::ApllodbResult,
    traits::SqlConvertible,
};

pub(crate) trait FromSqliteRow {
    fn from_sqlite_row(
        sqlite_row: &rusqlite::Row<'_>,
        column_data_types: &[&ColumnDataType],
    ) -> ApllodbResult<ImmutableRow> {
        let mut builder = ImmutableRowBuilder::default();

        for column_data_type in column_data_types {
            let column_name = column_data_type.column_name();
            let data_type = column_data_type.data_type();

            let sql_value = match data_type.kind() {
                DataTypeKind::SmallInt => {
                    Self::_sqlite_row_value::<i16>(&sqlite_row, &column_data_type)?
                }
                DataTypeKind::Integer => {
                    Self::_sqlite_row_value::<i32>(&sqlite_row, &column_data_type)?
                }
                DataTypeKind::BigInt => {
                    Self::_sqlite_row_value::<i64>(&sqlite_row, &column_data_type)?
                }
            };

            builder = builder.add_column(column_name, sql_value)?;
        }

        let row = builder.build()?;
        Ok(row)
    }

    fn _sqlite_row_value<T>(
        sqlite_row: &rusqlite::Row<'_>,
        column_data_type: &ColumnDataType,
    ) -> ApllodbResult<SqlValue>
    where
        T: rusqlite::types::FromSql + SqlConvertible,
    {
        let column_name = column_data_type.column_name();
        let data_type = column_data_type.data_type();

        let err_conv = |e: rusqlite::Error| {
            map_sqlite_err(
                e,
                format!("failed to get column `{}`'s value from SQLite", column_name),
            )
        };

        let sql_value: SqlValue = if data_type.nullable() {
            let rust_value: Option<T> = sqlite_row.get(column_name.as_str()).map_err(err_conv)?;
            SqlValue::pack(data_type, &rust_value)?
        } else {
            let rust_value: T = sqlite_row.get(column_name.as_str()).map_err(err_conv)?;
            SqlValue::pack(data_type, &rust_value)?
        };

        Ok(sql_value)
    }
}

impl FromSqliteRow for ImmutableRow {}
