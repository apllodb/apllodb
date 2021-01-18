use crate::{
    error::InfraError,
    sqlite::{row_iterator::SqliteRowIterator, to_sql_string::ToSqlString},
};
use apllodb_shared_components::{ApllodbResult, ColumnDataType, ColumnReference};
use sqlx::Statement;

use super::SqliteTx;

#[derive(Debug, new)]
pub(in crate::sqlite::transaction) struct SqliteStatement<'stmt, 'sqcn: 'stmt> {
    sqlite_tx: &'stmt mut SqliteTx<'sqcn>,
    sqlite_stmt: sqlx::sqlite::SqliteStatement<'sqcn>,
}

impl<'stmt, 'sqcn: 'stmt> SqliteStatement<'stmt, 'sqcn> {
    pub(in crate::sqlite::transaction::sqlite_tx) fn query_with(
        &mut self,
        params: sqlx::sqlite::SqliteArguments,
        column_data_types: &[&ColumnDataType],
        void_projection: &[ColumnReference],
    ) -> ApllodbResult<SqliteRowIterator> {
        let params = params
            .iter()
            .map(|(pname, v)| (*pname, v.to_sql_string()))
            .collect::<Vec<(&str, String)>>();

        let mut sqlite_rows = self
            .sqlite_stmt
            .query_with(params)
            .map_err(InfraError::from)?
            .execute(self);

        let iter = SqliteRowIterator::new(&mut sqlite_rows, column_data_types, void_projection)?;
        Ok(iter)
    }
}
