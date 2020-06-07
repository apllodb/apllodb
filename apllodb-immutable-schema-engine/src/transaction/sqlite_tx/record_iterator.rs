use apllodb_shared_components::{
    data_structure::Record,
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};

pub(crate) struct SqliteRecordIterator<'stmt>(
    rusqlite::MappedRows<'stmt, Box<dyn FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<Record>>>,
);

impl Iterator for SqliteRecordIterator<'_> {
    type Item = ApllodbResult<Record>;

    fn next(&mut self) -> Option<Self::Item> {
        let rec_res: rusqlite::Result<Record> = self.0.next()?;
        Some(rec_res.map_err(|rusqlite_err| {
            ApllodbError::new(
                ApllodbErrorKind::IoError,
                "SQLite raised error while iterating next row",
                Some(Box::new(rusqlite_err)),
            )
        }))
    }
}
