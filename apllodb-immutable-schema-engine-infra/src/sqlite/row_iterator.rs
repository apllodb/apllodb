use apllodb_immutable_schema_engine_domain::{
    row::immutable_row::ImmutableRow, row_iter::version_row_iter::VersionRowIterator,
};
use apllodb_shared_components::{ApllodbResult, ColumnDataType, ColumnReference};

use std::{collections::VecDeque, fmt::Debug};

#[derive(Clone, PartialEq, Debug)]
pub struct SqliteRowIterator(
    // Better to hold rusqlite::Rows or rusqlite::MappedRows (which implements Iterator) inside
    // to reduce memory consumption but I found it's impossible.
    //
    // rusqlite::Row<'stmt> requires rusqlite::Statement has the same or longer lifetime to it.
    // It seems impossible to have such lifetime and return rusqlite::Row<'stmt> to a caller who makes it.
    //
    // rusqlite::MappedRows<'stmt, F> has the same difficulty with rusqlite::Row.
    // Besides, converting rusqlite::MappedRows into crate::Row requires `&[crate::ColumnDataType]`
    // so this conversion has to be a closure capturing `&[crate::ColumnDataType]` and an instance
    // of type `F: FnMut(&rusqlite::Row) -> rusqlite::Result<T>` can be determined only where
    // the closure is written with `&[crate::ColumnDataType]`.
    // So type parameter F cannot be passed to a caller who cannot resolve F with the closure instance.
    VecDeque<ImmutableRow>,
);

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
        void_projection: &[ColumnReference],
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
