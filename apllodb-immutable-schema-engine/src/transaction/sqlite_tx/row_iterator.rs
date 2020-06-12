use crate::row::ImmutableSchemaRowIter;
use apllodb_shared_components::error::{ApllodbError, ApllodbErrorKind, ApllodbResult};
use apllodb_storage_manager_interface::Row;

type ToApllodbRow = Box<dyn FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<Row>>;

pub(crate) struct SqliteRowIterator<'stmt>(rusqlite::MappedRows<'stmt, ToApllodbRow>);

impl Iterator for SqliteRowIterator<'_> {
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

impl ImmutableSchemaRowIter for SqliteRowIterator<'_> {
    fn chain(_iters: Vec<Self>) -> Self
    where
        Self: Sized,
    {
        todo!()
        // これ、PoCだとただのVecDeqだからつなげたけど、無理かなぁ...
        // PoCみたいに for1VersionのVecDeqにするのは良い方法
    }
}

impl<'stmt> From<rusqlite::MappedRows<'stmt, ToApllodbRow>> for SqliteRowIterator<'stmt> {
    fn from(sqlite_rows: rusqlite::MappedRows<'stmt, ToApllodbRow>) -> Self {
        Self(sqlite_rows)
    }
}
