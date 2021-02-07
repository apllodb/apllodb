use apllodb_immutable_schema_engine_domain::{
    row::immutable_row::ImmutableRow,
    row_iter::version_row_iter::{row_column_ref_schema::RowColumnRefSchema, VersionRowIterator},
};
use apllodb_shared_components::{ApllodbResult, ColumnDataType, ColumnName, TableName};
use std::{collections::VecDeque, fmt::Debug};

#[derive(Clone, PartialEq, Debug)]
pub struct SqliteRowIterator {
    schema: RowColumnRefSchema,
    rows: VecDeque<ImmutableRow>,
}

impl Iterator for SqliteRowIterator {
    type Item = ImmutableRow;

    fn next(&mut self) -> Option<Self::Item> {
        self.rows.pop_front()
    }
}

impl VersionRowIterator for SqliteRowIterator {}

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

        let schema = RowColumnRefSchema::new(
            table_name.clone(),
            column_data_types
                .iter()
                .map(|cdt| cdt.column_name())
                .chain(void_projection.iter())
                .cloned()
                .collect(),
        );

        Ok(Self { schema, rows })
    }

    pub(crate) fn schema(&self) -> &RowColumnRefSchema {
        &self.schema
    }

    pub(crate) fn empty() -> Self {
        Self {
            schema: RowColumnRefSchema::new(TableName::new("from_empty_rows").unwrap(), vec![]),
            rows: VecDeque::new(),
        }
    }
}

impl From<VecDeque<ImmutableRow>> for SqliteRowIterator {
    fn from(rows: VecDeque<ImmutableRow>) -> Self {
        if rows.is_empty() {
            Self::empty()
        } else {
            let r = rows.front().unwrap();
            let schema = r.schema().clone();
            Self { schema, rows }
        }
    }
}
