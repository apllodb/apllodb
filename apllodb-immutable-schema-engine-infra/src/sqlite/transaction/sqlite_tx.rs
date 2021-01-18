mod sqlite_statement;
pub(crate) mod version;
pub(crate) mod version_revision_resolver;
pub(crate) mod vtable;

pub(in crate::sqlite::transaction::sqlite_tx) use sqlite_statement::SqliteStatement;
use sqlx::{Connection, Executor};

use self::{
    version::repository_impl::VersionRepositoryImpl, vtable::repository_impl::VTableRepositoryImpl,
};

use crate::{
    error::InfraError,
    sqlite::{database::SqliteDatabase, sqlite_rowid::SqliteRowid, to_sql_string::ToSqlString},
};
use apllodb_shared_components::{ApllodbError, ApllodbErrorKind, ApllodbResult, DatabaseName};
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

    pub(in crate::sqlite::transaction::sqlite_tx) async fn prepare(
        &mut self,
        sql: &str,
    ) -> ApllodbResult<SqliteStatement<'_, '_>> {
        let sql = sql.as_ref();
        debug!("SqliteTx::prepare():\n    {}", sql);

        let raw_stmt = self.sqlx_tx.prepare(sql).await.map_err(InfraError::from)?;
        Ok(SqliteStatement::new(&mut self, raw_stmt))
    }

    pub(in crate::sqlite::transaction::sqlite_tx) fn execute_named(
        &self,
        sql: &str,
        params: sqlx::sqlite::SqliteArguments,
    ) -> ApllodbResult<()> {
        // TODO return ChangedRows(usize)

        let sql = sql.as_ref();
        debug!("SqliteTx::execute_named():\n    {}", sql);

        let msg = |prefix: &str| {
            format!(
                "{} while execute_named() with the following command:\n    {}",
                prefix, sql
            )
        };

        self.sqlx_tx
            .execute(
                sql,
                params
                    .iter()
                    .map(|(pname, s)| (*pname, s as &dyn rusqlite::ToSql))
                    .collect::<Vec<_>>()
                    .as_slice(),
            )
            .map_err(|e| match e {
                rusqlite::Error::SqliteFailure(
                    libsqlite3_sys::Error {
                        code: libsqlite3_sys::ErrorCode::DatabaseBusy,
                        ..
                    },
                    _,
                ) => ApllodbError::new(
                    ApllodbErrorKind::DeadlockDetected,
                    msg("deadlock detected"),
                    Some(Box::new(e)),
                ),

                rusqlite::Error::SqliteFailure(
                    libsqlite3_sys::Error {
                        extended_code: rusqlite::ffi::SQLITE_CONSTRAINT_PRIMARYKEY,
                        ..
                    },
                    _,
                ) => ApllodbError::new(
                    ApllodbErrorKind::UniqueViolation,
                    msg("duplicate value on primary key"),
                    Some(Box::new(e)),
                ),

                _ => map_sqlite_err(e, msg("unexpected error")),
            })?;

        Ok(())
    }

    pub(in crate::sqlite::transaction::sqlite_tx) fn last_insert_rowid(&self) -> SqliteRowid {
        SqliteRowid(self.sqlx_tx.last_insert_rowid())
    }
}
