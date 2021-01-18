pub(crate) mod version;
pub(crate) mod version_revision_resolver;
pub(crate) mod vtable;

use sqlx::Connection;

use self::{
    version::repository_impl::VersionRepositoryImpl, vtable::repository_impl::VTableRepositoryImpl,
};

use crate::{
    error::InfraError,
    sqlite::{
        database::SqliteDatabase, row_iterator::SqliteRowIterator, sqlite_rowid::SqliteRowid,
        to_sql_string::ToSqlString,
    },
};
use apllodb_shared_components::{ApllodbResult, ColumnDataType, ColumnReference, DatabaseName};
use log::debug;

/// Many transactions share 1 SQLite connection in `Database`.
#[derive(Debug)]
pub struct SqliteTx<'sqcn> {
    database_name: DatabaseName,
    sqlx_tx: sqlx::Transaction<'sqcn, sqlx::sqlite::Sqlite>,
}

impl<'sqcn> SqliteTx<'sqcn> {
    pub(crate) fn vtable_repo(&self) -> VTableRepositoryImpl<'_, 'sqcn> {
        VTableRepositoryImpl::new(self)
    }

    pub(crate) fn version_repo(&self) -> VersionRepositoryImpl<'_, 'sqcn> {
        VersionRepositoryImpl::new(self)
    }
}

impl<'sqcn> SqliteTx<'sqcn> {
    /// # Failures
    ///
    /// - [IoError](apllodb_shared_components::ApllodbErrorKind::IoError) when:
    ///   - rusqlite raises an error.
    async fn begin(db: &'sqcn mut SqliteDatabase) -> ApllodbResult<SqliteTx<'sqcn>> {
        let database_name = { db.name().clone() };

        let tx = db.sqlite_conn().begin().await.map_err(InfraError::from)?;

        Ok(Self {
            database_name,
            sqlx_tx: tx,
        })
    }

    /// # Failures
    ///
    /// If any of the following error is returned, transaction has already been aborted.
    ///
    /// - [IoError](apllodb_shared_components::ApllodbErrorKind::IoError) when:
    ///   - rusqlite raises an error.
    async fn commit(self) -> ApllodbResult<()> {
        self.sqlx_tx.commit().await.map_err(InfraError::from)?;
        Ok(())
    }

    /// # Failures
    ///
    /// - [IoError](apllodb_shared_components::ApllodbErrorKind::IoError) when:
    ///   - rusqlite raises an error.
    async fn abort(self) -> ApllodbResult<()> {
        self.sqlx_tx.rollback().await.map_err(InfraError::from)?;
        Ok(())
    }

    fn database_name(&self) -> &DatabaseName {
        &self.database_name
    }
}

impl<'sqcn> SqliteTx<'sqcn> {
    // FIXME should take placeholder argument to prevent SQL-i
    pub(in crate::sqlite::transaction::sqlite_tx) async fn query<'q>(
        &mut self,
        sql: &'q str,
        column_data_types: &[&ColumnDataType],
        void_projection: &[ColumnReference],
    ) -> ApllodbResult<SqliteRowIterator> {
        debug!("SqliteTx::query():\n    {}", sql);

        let mut rows = sqlx::query(sql)
            .fetch_all(&mut self.sqlx_tx)
            .await
            .map_err(InfraError::from)?;
        SqliteRowIterator::new(&mut rows, column_data_types, void_projection)
    }

    pub(in crate::sqlite::transaction::sqlite_tx) async fn execute<'q>(
        &mut self,
        sql: &'q str,
    ) -> ApllodbResult<SqliteRowid> {
        debug!("SqliteTx::execute_named():\n    {}", sql);

        let done = sqlx::query(sql)
            .execute(&mut self.sqlx_tx)
            .await
            .map_err(InfraError::from)?;

        Ok(SqliteRowid(done.last_insert_rowid()))
    }
}
