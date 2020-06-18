use apllodb_immutable_schema_engine_domain::VTable;
use apllodb_shared_components::error::{ApllodbError, ApllodbErrorKind, ApllodbResult};
use log::error;

#[derive(Debug)]
pub(in crate::sqlite) struct VTableDao<'tx> {
    sqlite_tx: &'tx rusqlite::Transaction<'tx>,
}

const TABLE_NAME: &str = "_vtable_metadata";
const CNAME_TABLE_NAME: &str = "table_name";
const CNAME_TABLE_WIDE_CONSTRAINTS: &str = "table_wide_constraints";

impl<'tx> VTableDao<'tx> {
    pub(in crate::sqlite) fn create_table_if_not_exist(
        sqlite_conn: &rusqlite::Connection,
    ) -> ApllodbResult<()> {
        let sql = format!(
            "
CREATE TABLE IF NOT EXISTS {} (
  {} TEXT PRIMARY KEY,
  {} TEXT
)
        ",
            TABLE_NAME, CNAME_TABLE_NAME, CNAME_TABLE_WIDE_CONSTRAINTS
        );

        sqlite_conn
            .execute(sql.as_str(), rusqlite::params![])
            .map(|_| ())
            .map_err(|e| {
                ApllodbError::new(
                    ApllodbErrorKind::IoError,
                    format!(
                        "backend sqlite3 raised an error on creating metadata table `{}`",
                        TABLE_NAME
                    ),
                    Some(Box::new(e)),
                )
            })?;
        Ok(())
    }

    pub(in crate::sqlite::transaction::sqlite_tx) fn new(
        sqlite_tx: &'tx rusqlite::Transaction<'tx>,
    ) -> Self {
        Self { sqlite_tx }
    }

    /// # Failures
    ///
    /// - [DeadlockDetected](error/enum.ApllodbErrorKind.html#variant.DeadlockDetected) when:
    ///   - transaction lock to metadata table takes too long time.
    /// - [DuplicateTable](error/enum.ApllodbErrorKind.html#variant.DuplicateTable) when:
    ///   - `table` is already created.
    /// - [SerializationError](error/enum.ApllodbErrorKind.html#variant.SerializationError) when:
    ///   - Somehow failed to serialize part of [Table](foobar.html).
    pub(in crate::sqlite::transaction::sqlite_tx) fn create(
        &self,
        vtable: &VTable,
    ) -> ApllodbResult<()> {
        let sql = format!(
            "INSERT INTO {} ({}, {}) VALUES (:table_name, :table_wide_constraints);",
            TABLE_NAME, CNAME_TABLE_NAME, CNAME_TABLE_WIDE_CONSTRAINTS
        );

        let table_name = format!("{}", vtable.table_name());

        let table_wide_constraints = vtable.table_wide_constraints();
        let table_wide_constraints =
            serde_yaml::to_string(table_wide_constraints).map_err(|e| {
                ApllodbError::new(
                    ApllodbErrorKind::SerializationError,
                    format!(
                        "failed to serialize `{}`'s table wide constraints: `{:?}`",
                        table_name, table_wide_constraints
                    ),
                    Some(Box::new(e)),
                )
            })?;

        match self.sqlite_tx.execute_named(
            &sql,
            &[
                (":table_name", &table_name),
                (":table_wide_constraints", &table_wide_constraints),
            ],
        ) {
            // TODO SQLite側のエラー処理は、 https://www.sqlite.org/rescode.html#busy をenum variant で引っ掛けられるようになりたい。rusqliteへの改修を画策中
            Err(
                e
                @
                rusqlite::Error::SqliteFailure(
                    libsqlite3_sys::Error {
                        code: libsqlite3_sys::ErrorCode::DatabaseBusy,
                        ..
                    },
                    _,
                ),
            ) => Err(ApllodbError::new(
                // TODO TableDao::create() 意外にも現れは汎用的なエラー処理ななんとかする
                ApllodbErrorKind::DeadlockDetected,
                format!(
                    "table `{}` is exclusively locked by another transaction for too long time",
                    table_name
                ),
                Some(Box::new(e)),
            )),
            Err(
                e
                @
                rusqlite::Error::SqliteFailure(
                    libsqlite3_sys::Error {
                        extended_code: rusqlite::ffi::SQLITE_CONSTRAINT_PRIMARYKEY,
                        ..
                    },
                    _,
                ),
            ) => Err(ApllodbError::new(
                ApllodbErrorKind::DuplicateTable,
                format!("table `{}` is already CREATEd", table_name),
                Some(Box::new(e)),
            )),
            Err(e) => {
                error!("unexpected SQLite error: {:?}", e);
                Err(ApllodbError::new(
                    ApllodbErrorKind::IoError,
                    format!(
                        "SQLite raised an error creating table `{}`'s metadata",
                        table_name
                    ),
                    Some(Box::new(e)),
                ))
            }
            Ok(_) => Ok(()),
        }
    }
}
