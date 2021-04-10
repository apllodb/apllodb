use apllodb_immutable_schema_engine_domain::row::immutable_row::ImmutableRow;
use apllodb_shared_components::ApllodbResult;
use apllodb_storage_engine_interface::{
    ColumnDataType, ColumnName, RowSchema, TableColumnName, TableName,
};
use std::{collections::VecDeque, fmt::Debug};

#[derive(Clone, PartialEq, Debug)]
pub struct SqliteRowIterator {
    schema: RowSchema,
    rows: VecDeque<ImmutableRow>,
}

impl Iterator for SqliteRowIterator {
    type Item = ImmutableRow;

    fn next(&mut self) -> Option<Self::Item> {
        self.rows.pop_front()
    }
}

impl SqliteRowIterator {
    /// # Arguments
    ///
    /// - `non_pk_column_data_types` - Only contains columns `sqlite_rows` have.
    /// - `non_pk_void_projection` - Columns `sqlite_rows` do not have but another version has.
    pub(in crate::sqlite) fn new(
        sqlite_rows: &[sqlx::sqlite::SqliteRow],
        table_name: &TableName,
        column_data_types: &[&ColumnDataType],
        void_projection: &[ColumnName],
    ) -> ApllodbResult<Self> {
        use crate::sqlite::from_sqlite_row::FromSqliteRow;

        let mut rows: VecDeque<ImmutableRow> = VecDeque::new();

        for sqlite_row in sqlite_rows {
            let row = ImmutableRow::from_sqlite_row(
                sqlite_row,
                table_name,
                column_data_types,
                void_projection,
            )?;
            rows.push_back(row);
        }

        let schema = RowSchema::from(
            column_data_types
                .iter()
                .map(|cdt| cdt.column_name())
                .chain(void_projection.iter())
                .map(|cn| TableColumnName::new(table_name.clone(), cn.clone()))
                .collect::<Vec<TableColumnName>>(),
        );

        Ok(Self { schema, rows })
    }

    pub(crate) fn as_schema(&self) -> &RowSchema {
        &self.schema
    }
}

impl From<(RowSchema, VecDeque<ImmutableRow>)> for SqliteRowIterator {
    fn from(f: (RowSchema, VecDeque<ImmutableRow>)) -> Self {
        let (schema, rows) = f;
        Self { schema, rows }
    }
}
