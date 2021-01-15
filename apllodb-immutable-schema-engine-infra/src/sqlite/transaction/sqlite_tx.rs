mod sqlite_statement;
pub(crate) mod version;
pub(crate) mod version_revision_resolver;
pub(crate) mod vtable;

pub(in crate::sqlite::transaction::sqlite_tx) use sqlite_statement::SqliteStatement;

use self::{
    version::repository_impl::VersionRepositoryImpl, vtable::repository_impl::VTableRepositoryImpl,
};

use crate::sqlite::{
    database::SqliteDatabase, sqlite_error::map_sqlite_err, sqlite_rowid::SqliteRowid,
    to_sql_string::ToSqlString,
};
use apllodb_shared_components::{ApllodbError, ApllodbErrorKind, ApllodbResult, DatabaseName};
use log::debug;

/// Many transactions share 1 SQLite connection in `Database`.
#[derive(Debug)]
pub struct SqliteTx<'sqcn> {
    database_name: DatabaseName,
    rusqlite_tx: rusqlite::Transaction<'sqcn>,
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
    fn begin(db: &'sqcn mut SqliteDatabase) -> ApllodbResult<Self> {
        let database_name = { db.name().clone() };

        let tx = db.sqlite_conn().transaction().map_err(|e| {
            map_sqlite_err(
                e,
                "backend sqlite3 raised an error on beginning transaction",
            )
        })?;

        Ok(Self {
            database_name,
            rusqlite_tx: tx,
        })
    }

    /// # Failures
    ///
    /// If any of the following error is returned, transaction has already been aborted.
    ///
    /// - [IoError](apllodb_shared_components::ApllodbErrorKind::IoError) when:
    ///   - rusqlite raises an error.
    fn commit(self) -> ApllodbResult<()> {
        self.rusqlite_tx.commit().map_err(|e| {
            map_sqlite_err(
                e,
                "backend sqlite3 raised an error on committing transaction",
            )
        })?;
        Ok(())
    }

    /// # Failures
    ///
    /// - [IoError](apllodb_shared_components::ApllodbErrorKind::IoError) when:
    ///   - rusqlite raises an error.
    fn abort(self) -> ApllodbResult<()> {
        self.rusqlite_tx.rollback().map_err(|e| {
            map_sqlite_err(e, "backend sqlite3 raised an error on aborting transaction")
        })?;
        Ok(())
    }

    fn database_name(&self) -> &DatabaseName {
        &self.database_name
    }

    pub(in crate::sqlite::transaction::sqlite_tx) fn prepare<S: AsRef<str>>(
        &self,
        sql: S,
    ) -> ApllodbResult<SqliteStatement<'_, '_>> {
        let sql = sql.as_ref();
        debug!("SqliteTx::prepare():\n    {}", sql);

        let raw_stmt = self
            .rusqlite_tx
            .prepare(sql)
            .map_err(|e| map_sqlite_err(e, "SQLite raised an error on prepare"))?;
        Ok(SqliteStatement::new(&self, raw_stmt))
    }

    pub(in crate::sqlite::transaction::sqlite_tx) fn execute_named<S: AsRef<str>>(
        &self,
        sql: S,
        params: &[(&str, &dyn ToSqlString)],
    ) -> ApllodbResult<()> {
        // TODO return ChangedRows(usize)

        let sql = sql.as_ref();
        debug!("SqliteTx::execute_named():\n    {}", sql);

        let params = params
            .iter()
            .map(|(pname, v)| (*pname, v.to_sql_string()))
            .collect::<Vec<(&str, String)>>();

        let msg = |prefix: &str| {
            format!(
                "{} while execute_named() with the following command:\n    {}",
                prefix, sql
            )
        };

        self.rusqlite_tx
            .execute_named(
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
        SqliteRowid(self.rusqlite_tx.last_insert_rowid())
    }
}
