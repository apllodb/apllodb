use super::SqliteTx;
use crate::sqlite::{sqlite_error::map_sqlite_err, to_sql_string::ToSqlString, SqliteRowIterator};
use apllodb_shared_components::{data_structure::ColumnDataType, error::ApllodbResult};

#[derive(Debug)]
pub struct SqliteStatement<'tx, 'db: 'tx> {
    sqlite_tx: &'tx SqliteTx<'db>,
    sqlite_stmt: rusqlite::Statement<'db>,
}

impl<'tx, 'db: 'tx> SqliteStatement<'tx, 'db> {
    pub(super) fn new(
        sqlite_tx: &'tx SqliteTx<'db>,
        sqlite_stmt: rusqlite::Statement<'db>,
    ) -> Self {
        Self {
            sqlite_tx,
            sqlite_stmt,
        }
    }

    pub(in crate::sqlite::transaction::sqlite_tx) fn query_named(
        &mut self,
        params: &[(&str, &dyn ToSqlString)],
        column_data_types: &[&ColumnDataType],
    ) -> ApllodbResult<SqliteRowIterator> {
        let params = params
            .into_iter()
            .map(|(pname, v)| (*pname, v.to_sql_string()))
            .collect::<Vec<(&str, String)>>();

        let mut rusqlite_rows = self
            .sqlite_stmt
            .query_named(
                params
                    .iter()
                    .map(|(pname, s)| (*pname, s as &dyn rusqlite::ToSql))
                    .collect::<Vec<_>>()
                    .as_slice(),
            )
            .map_err(|e| map_sqlite_err(e, "SQLite raised an error on query_named()"))?;


        let iter = SqliteRowIterator::new(&mut rusqlite_rows, column_data_types)?;
        Ok(iter)
    }
}
