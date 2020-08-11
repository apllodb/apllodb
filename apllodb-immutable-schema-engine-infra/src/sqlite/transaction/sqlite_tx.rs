mod dao;
mod repository;
mod sqlite_statement;

pub(in crate::sqlite) use dao::VTableDao;
pub(in crate::sqlite::transaction::sqlite_tx) use sqlite_statement::SqliteStatement;

use super::tx_id::TxId;
use crate::sqlite::{
    database::SqliteDatabase, sqlite_error::map_sqlite_err, sqlite_rowid::SqliteRowid,
    to_sql_string::ToSqlString,
};
use apllodb_immutable_schema_engine_domain::transaction::ImmutableSchemaTx;
use apllodb_shared_components::{
    data_structure::DatabaseName,
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use log::debug;
use repository::{VTableRepositoryImpl, VersionRepositoryImpl};
use std::cmp::Ordering;

/// Many transactions share 1 SQLite connection in `Database`.
#[derive(Debug)]
pub struct SqliteTx<'db> {
    id: TxId,
    database_name: DatabaseName,
    rusqlite_tx: rusqlite::Transaction<'db>,
}

impl PartialEq for SqliteTx<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for SqliteTx<'_> {}

impl PartialOrd for SqliteTx<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for SqliteTx<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl<'tx, 'db: 'tx> ImmutableSchemaTx<'tx, 'db> for SqliteTx<'db> {
    type Db = SqliteDatabase;
    type TID = TxId;

    type VTRepo = VTableRepositoryImpl<'tx, 'db>;
    type VRepo = VersionRepositoryImpl<'tx, 'db>;

    fn id(&self) -> &Self::TID {
        &self.id
    }

    /// # Failures
    ///
    /// - [IoError](error/enum.ApllodbErrorKind.html#variant.IoError) when:
    ///   - rusqlite raises an error.
    fn begin(db: &'db mut Self::Db) -> ApllodbResult<Self> {
        use apllodb_shared_components::traits::Database;

        let database_name = { db.name().clone() };

        let tx = db.sqlite_conn().transaction().map_err(|e| {
            map_sqlite_err(
                e,
                format!("backend sqlite3 raised an error on beginning transaction"),
            )
        })?;

        Ok(Self {
            id: TxId::new(),
            database_name: database_name,
            rusqlite_tx: tx,
        })
    }

    /// # Failures
    ///
    /// If any of the following error is returned, transaction has already been aborted.
    ///
    /// - [IoError](error/enum.ApllodbErrorKind.html#variant.IoError) when:
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
    /// - [IoError](error/enum.ApllodbErrorKind.html#variant.IoError) when:
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

    fn vtable_repo(&'tx self) -> Self::VTRepo {
        use apllodb_immutable_schema_engine_domain::traits::VTableRepository;
        VTableRepositoryImpl::new(&self)
    }

    fn version_repo(&'tx self) -> Self::VRepo {
        use apllodb_immutable_schema_engine_domain::traits::VersionRepository;
        VersionRepositoryImpl::new(&self)
    }
}

impl<'db> SqliteTx<'db> {
    pub(in crate::sqlite::transaction::sqlite_tx) fn prepare<S: AsRef<str>>(
        &self,
        sql: S,
    ) -> ApllodbResult<SqliteStatement<'_, '_>> {
        let sql = sql.as_ref();
        debug!("SqliteTx::prepare(): {}", sql);

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
        debug!("SqliteTx::execute_named(): {}", sql);

        let params = params
            .into_iter()
            .map(|(pname, v)| (*pname, v.to_sql_string()))
            .collect::<Vec<(&str, String)>>();

        let msg = |prefix: &str| {
            format!(
                "{} while execute_named() with the following command: {}",
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
