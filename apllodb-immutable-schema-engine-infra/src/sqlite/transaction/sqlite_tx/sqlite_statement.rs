use super::SqliteTx;
use crate::sqlite::{
    row_iterator::SqliteRowIterator, sqlite_error::map_sqlite_err, to_sql_string::ToSqlString,
};
use apllodb_shared_components::{
    data_structure::ColumnDataType, data_structure::ColumnReference, error::ApllodbResult,
};

#[derive(Debug)]
pub(in crate::sqlite::transaction) struct SqliteStatement<'stmt, 'db: 'stmt> {
    sqlite_tx: &'stmt SqliteTx<'db>,
    sqlite_stmt: rusqlite::Statement<'db>,
}

impl<'stmt, 'db: 'stmt> SqliteStatement<'stmt, 'db> {
    pub(super) fn new(
        sqlite_tx: &'stmt SqliteTx<'db>,
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
        void_projection: &[ColumnReference],
    ) -> ApllodbResult<SqliteRowIterator> {
        let params = params
            .iter()
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

        let iter = SqliteRowIterator::new(&mut rusqlite_rows, column_data_types, void_projection)?;
        Ok(iter)
    }
}
