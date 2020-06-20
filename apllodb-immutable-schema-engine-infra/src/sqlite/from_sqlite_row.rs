use apllodb_shared_components::{
    data_structure::{ColumnDataType, DataTypeKind, SqlValue},
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
    traits::SqlConvertible,
};
use apllodb_storage_engine_interface::{Row, RowBuilder};

pub(crate) trait FromSqliteRow {
    fn from_sqlite_row(
        sqlite_row: rusqlite::Row,
        column_data_types: &[ColumnDataType],
    ) -> ApllodbResult<Row> {
        let mut builder = RowBuilder::default();

        for column_data_type in column_data_types {
            let column_name = column_data_type.column_name();
            let data_type = column_data_type.data_type();

            let sql_value = match data_type.kind() {
                DataTypeKind::SmallInt | DataTypeKind::Integer | DataTypeKind::BigInt => {
                    Self::_sqlite_row_value::<i64>(&sqlite_row, column_data_type)?
                }
            };

            builder = builder.add_column(column_name, sql_value)?;
        }

        Ok(builder.build())
    }

    fn _sqlite_row_value<T>(
        sqlite_row: &rusqlite::Row,
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

impl FromSqliteRow for Row {}
