use apllodb_immutable_schema_engine_domain::{
    row::immutable_row::ImmutableRow, row_iter::version_row_iter::VersionRowIterator,
};
use apllodb_shared_components::{ApllodbResult, ColumnDataType};
use apllodb_storage_engine_interface::TableColumnReference;

use std::{collections::VecDeque, fmt::Debug};

#[derive(Clone, PartialEq, Debug)]
pub struct SqliteRowIterator(VecDeque<ImmutableRow>);

impl Iterator for SqliteRowIterator {
    type Item = ImmutableRow;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
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
        column_data_types: &[&ColumnDataType],
        void_projection: &[TableColumnReference],
    ) -> ApllodbResult<Self> {
        use crate::sqlite::from_sqlite_row::FromSqliteRow;

        let mut rows: VecDeque<ImmutableRow> = VecDeque::new();

        for sqlite_row in sqlite_rows {
            let row =
                ImmutableRow::from_sqlite_row(sqlite_row, column_data_types, void_projection)?;
            rows.push_back(row);
        }
        Ok(Self(rows))
    }
}

impl From<VecDeque<ImmutableRow>> for SqliteRowIterator {
    fn from(rows: VecDeque<ImmutableRow>) -> Self {
        Self(rows)
    }
}
