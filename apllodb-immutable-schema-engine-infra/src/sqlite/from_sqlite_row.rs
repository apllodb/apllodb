use super::sqlite_error::map_sqlite_err;
use apllodb_immutable_schema_engine_domain::row::immutable_row::{
    builder::ImmutableRowBuilder, ImmutableRow,
};
use apllodb_shared_components::{
    data_structure::ColumnReference,
    data_structure::{ColumnDataType, DataTypeKind, SqlValue},
    error::ApllodbResult,
    traits::SqlConvertible,
};

pub(crate) trait FromSqliteRow {
    fn from_sqlite_row(
        sqlite_row: &rusqlite::Row<'_>,
        column_data_types: &[&ColumnDataType],
        void_projections: &[ColumnReference],
    ) -> ApllodbResult<ImmutableRow> {
        let mut builder = ImmutableRowBuilder::default();

        for cdt in column_data_types {
            let non_pk_column_name = cdt.column_ref();
            let non_pk_sql_value = Self::_sql_value(sqlite_row, cdt)?;
            builder = builder.add_col_val(&non_pk_column_name, non_pk_sql_value)?;
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
        sqlite_row: &rusqlite::Row<'_>,
        column_data_type: &ColumnDataType,
    ) -> ApllodbResult<SqlValue> {
        let data_type = column_data_type.data_type();

        let sql_value = match data_type.kind() {
            DataTypeKind::SmallInt => Self::_sqlite_row_value::<i16>(sqlite_row, column_data_type)?,
            DataTypeKind::Integer => Self::_sqlite_row_value::<i32>(sqlite_row, column_data_type)?,
            DataTypeKind::BigInt => Self::_sqlite_row_value::<i64>(sqlite_row, column_data_type)?,
            DataTypeKind::Text => Self::_sqlite_row_value::<String>(sqlite_row, column_data_type)?,
        };

        Ok(sql_value)
    }

    fn _sqlite_row_value<T>(
        sqlite_row: &rusqlite::Row<'_>,
        column_data_type: &ColumnDataType,
    ) -> ApllodbResult<SqlValue>
    where
        T: rusqlite::types::FromSql + SqlConvertible,
    {
        let colref = column_data_type.column_ref();
        let data_type = column_data_type.data_type();

        let err_conv = |e: rusqlite::Error| {
            map_sqlite_err(
                e,
                format!("failed to get column `{}`'s value from SQLite", colref),
            )
        };

        let sql_value: SqlValue = if data_type.nullable() {
            let rust_value: Option<T> = sqlite_row
                .get(colref.as_column_name().as_str())
                .map_err(err_conv)?;
            SqlValue::pack(data_type, &rust_value)?
        } else {
            let rust_value: T = sqlite_row
                .get(colref.as_column_name().as_str())
                .map_err(err_conv)?;
            SqlValue::pack(data_type, &rust_value)?
        };

        Ok(sql_value)
    }
}

impl FromSqliteRow for ImmutableRow {}
