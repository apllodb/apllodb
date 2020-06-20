use super::sqlite_error::map_sqlite_err;
use apllodb_immutable_schema_engine_domain::VersionRowIter;
use apllodb_shared_components::error::ApllodbResult;
use apllodb_storage_engine_interface::Row;

type ToApllodbRow = Box<dyn FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<Row>>;

pub struct SqliteRowIterator<'stmt>(rusqlite::MappedRows<'stmt, ToApllodbRow>);

impl Iterator for SqliteRowIterator<'_> {
    type Item = ApllodbResult<Row>;

    fn next(&mut self) -> Option<Self::Item> {
        let rec_res: rusqlite::Result<Row> = self.0.next()?;
        Some(rec_res.map_err(|e| map_sqlite_err(e, "SQLite raised error while iterating next row")))
    }
}

impl VersionRowIter for SqliteRowIterator<'_> {}

impl<'stmt> From<rusqlite::MappedRows<'stmt, ToApllodbRow>> for SqliteRowIterator<'stmt> {
    fn from(sqlite_rows: rusqlite::MappedRows<'stmt, ToApllodbRow>) -> Self {
        Self(sqlite_rows)
    }
}
