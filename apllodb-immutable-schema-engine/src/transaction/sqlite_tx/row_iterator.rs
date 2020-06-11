use apllodb_shared_components::error::{ApllodbError, ApllodbErrorKind, ApllodbResult};
use apllodb_storage_manager_interface::Row;

type ToApllodbRow = Box<dyn FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<Row>>;

pub(crate) struct RowIterator<'stmt>(rusqlite::MappedRows<'stmt, ToApllodbRow>);

impl Iterator for RowIterator<'_> {
    type Item = ApllodbResult<Row>;

    fn next(&mut self) -> Option<Self::Item> {
        let rec_res: rusqlite::Result<Row> = self.0.next()?;
        Some(rec_res.map_err(|rusqlite_err| {
            ApllodbError::new(
                ApllodbErrorKind::IoError,
                "SQLite raised error while iterating next row",
                Some(Box::new(rusqlite_err)),
            )
        }))
    }
}

impl<'stmt> From<rusqlite::MappedRows<'stmt, ToApllodbRow>> for RowIterator<'stmt> {
    fn from(sqlite_rows: rusqlite::MappedRows<'stmt, ToApllodbRow>) -> Self {
        Self(sqlite_rows)
    }
}
