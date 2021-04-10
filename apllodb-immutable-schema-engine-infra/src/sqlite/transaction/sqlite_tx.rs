pub(crate) mod version;
pub(crate) mod version_revision_resolver;
pub(crate) mod vtable;

use std::{cell::RefCell, rc::Rc};

use self::{
    version::repository_impl::VersionRepositoryImpl, vtable::repository_impl::VTableRepositoryImpl,
};

use crate::{
    error::InfraError,
    sqlite::{
        database::SqliteDatabase, row_iterator::SqliteRowIterator, sqlite_rowid::SqliteRowid,
    },
};
use apllodb_shared_components::{ApllodbResult, DatabaseName};
use apllodb_storage_engine_interface::{ColumnDataType, ColumnName, TableName};
use log::debug;

/// Many transactions share 1 SQLite connection in `Database`.
#[derive(Debug)]
pub(crate) struct SqliteTx {
    database_name: DatabaseName,

    // will be Option::take() -n on commit/abort.
    sqlx_tx: Option<sqlx::Transaction<'static, sqlx::sqlite::Sqlite>>,
}

impl SqliteTx {
    pub(crate) fn vtable_repo(slf: Rc<RefCell<Self>>) -> VTableRepositoryImpl {
        VTableRepositoryImpl::new(slf)
    }

    pub(crate) fn version_repo(slf: Rc<RefCell<Self>>) -> VersionRepositoryImpl {
        VersionRepositoryImpl::new(slf)
    }
}

impl SqliteTx {
    /// # Failures
    ///
    /// - [IoError](apllodb_shared_components::ApllodbErrorKind::IoError) when:
    ///   - sqlx raises an error.
    pub(crate) async fn begin(db: &SqliteDatabase) -> ApllodbResult<Rc<RefCell<SqliteTx>>> {
        let database_name = { db.name().clone() };

        let tx = db.sqlite_pool().begin().await.map_err(InfraError::from)?;

        Ok(Rc::new(RefCell::new(Self {
            database_name,
            sqlx_tx: Some(tx),
        })))
    }

    /// # Failures
    ///
    /// If any of the following error is returned, transaction has already been aborted.
    ///
    /// - [IoError](apllodb_shared_components::ApllodbErrorKind::IoError) when:
    ///   - sqlx raises an error.
    pub(crate) async fn commit(&mut self) -> ApllodbResult<()> {
        self.sqlx_tx
            .take()
            .expect("SqliteTx::commit() / SqliteTx::abort() must be called only once")
            .commit()
            .await
            .map_err(InfraError::from)?;
        Ok(())
    }

    /// # Failures
    ///
    /// - [IoError](apllodb_shared_components::ApllodbErrorKind::IoError) when:
    ///   - sqlx raises an error.
    pub(crate) async fn abort(&mut self) -> ApllodbResult<()> {
        self.sqlx_tx
            .take()
            .expect("SqliteTx::commit() / SqliteTx::abort() must be called only once")
            .rollback()
            .await
            .map_err(InfraError::from)?;
        Ok(())
    }

    pub(crate) fn database_name(&self) -> &DatabaseName {
        &self.database_name
    }
}

impl SqliteTx {
    // FIXME should take placeholder argument to prevent SQL-i
    pub(in crate::sqlite::transaction::sqlite_tx) async fn query(
        &mut self,
        sql: &str,
        table_name: &TableName,
        column_data_types: &[&ColumnDataType],
        void_projection: &[ColumnName],
    ) -> ApllodbResult<SqliteRowIterator> {
        debug!("SqliteTx::query():\n    {}", sql);

        let rows = sqlx::query(sql)
            .fetch_all(self.sqlx_tx.as_mut().unwrap())
            .await
            .map_err(InfraError::from)?;
        SqliteRowIterator::new(&rows, table_name, column_data_types, void_projection)
    }

    pub(in crate::sqlite::transaction::sqlite_tx) async fn execute(
        &mut self,
        sql: &str,
    ) -> ApllodbResult<SqliteRowid> {
        debug!("SqliteTx::execute():\n    {}", sql);

        let done = sqlx::query(sql)
            .execute(self.sqlx_tx.as_mut().unwrap())
            .await
            .map_err(InfraError::from)?;

        Ok(SqliteRowid(done.last_insert_rowid()))
    }
}
