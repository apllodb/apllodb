use super::SqliteTx;
use crate::sqlite::{
    row_iterator::SqliteRowIterator, sqlite_error::map_sqlite_err, to_sql_string::ToSqlString,
};
use apllodb_immutable_schema_engine_domain::row::column::{
    non_pk_column::column_data_type::NonPKColumnDataType,
    non_pk_column::column_name::NonPKColumnName, pk_column::column_data_type::PKColumnDataType,
};
use apllodb_shared_components::error::ApllodbResult;

#[derive(Debug)]
pub(in crate::sqlite::transaction) struct SqliteStatement<'tx, 'db: 'tx> {
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
        pk_column_data_types: &[&PKColumnDataType],
        non_pk_column_data_types: &[&NonPKColumnDataType],
        non_pk_void_projection: &[NonPKColumnName],
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

        let iter = SqliteRowIterator::new(
            &mut rusqlite_rows,
            pk_column_data_types,
            non_pk_column_data_types,
            non_pk_void_projection,
        )?;
        Ok(iter)
    }
}
