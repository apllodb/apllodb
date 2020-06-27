mod create_table_sql_for_navi;

use crate::sqlite::sqlite_error::map_sqlite_err;
use apllodb_immutable_schema_engine_domain::{TableWideConstraints, VTable, VTableId};
use apllodb_shared_components::error::{ApllodbError, ApllodbErrorKind, ApllodbResult};
use create_table_sql_for_navi::CreateTableSqlForNavi;
use log::error;

#[derive(Debug)]
pub(in crate::sqlite) struct VTableDao<'tx, 'db: 'tx> {
    sqlite_tx: &'tx rusqlite::Transaction<'db>,
}

const TNAME_VTABLE_METADATA: &str = "_vtable_metadata";
const CNAME_TABLE_NAME: &str = "table_name";
const CNAME_TABLE_WIDE_CONSTRAINTS: &str = "table_wide_constraints";

impl<'tx, 'db: 'tx> VTableDao<'tx, 'db> {
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
            TNAME_VTABLE_METADATA, CNAME_TABLE_NAME, CNAME_TABLE_WIDE_CONSTRAINTS
        );

        sqlite_conn
            .execute(sql.as_str(), rusqlite::params![])
            .map(|_| ())
            .map_err(|e| {
                map_sqlite_err(
                    e,
                    format!(
                        "backend sqlite3 raised an error on creating metadata table `{}`",
                        TNAME_VTABLE_METADATA
                    ),
                )
            })?;
        Ok(())
    }

    pub(in crate::sqlite::transaction::sqlite_tx) fn new(
        sqlite_tx: &'tx rusqlite::Transaction<'db>,
    ) -> Self {
        Self { sqlite_tx }
    }

    /// Do the following:
    ///
    /// - INSERT INTO TNAME_VTABLE_METADATA
    /// - CREATE TABLE <tableName>_navi
    ///
    /// # Failures
    ///
    /// - Errors from insert_into_vtable_metadata()
    pub(in crate::sqlite::transaction::sqlite_tx) fn create(
        &self,
        vtable: &VTable,
    ) -> ApllodbResult<()> {
        self.insert_into_vtable_metadata(vtable)?;
        self.create_navi_table(vtable)?;

        Ok(())
    }

    /// # Failures
    ///
    /// - [UndefinedTable](error/enum.ApllodbErrorKind.html#variant.UndefinedTable) when:
    ///   - `table` is not visible from this transaction.
    /// - [DeserializationError](error/enum.ApllodbErrorKind.html#variant.DeserializationError) when:
    ///   - Somehow failed to deserialize part of [VTable](foobar.html).
    pub(in crate::sqlite::transaction::sqlite_tx) fn read(
        &self,
        vtable_id: &VTableId,
    ) -> ApllodbResult<VTable> {
        let sql = format!(
            "SELECT {}, {} FROM {} WHERE {} = :table_name;",
            CNAME_TABLE_NAME, CNAME_TABLE_WIDE_CONSTRAINTS, TNAME_VTABLE_METADATA, CNAME_TABLE_NAME
        );

        let table_name = format!("{}", vtable_id.table_name());

        let mut stmt = self.sqlite_tx.prepare(&sql).map_err(|e| {
            error!("unexpected SQLite error: {:?}", e);
            map_sqlite_err(
                e,
                format!(
                    "SQLite raised an error while preparing for selecting table `{}`'s metadata",
                    table_name
                ),
            )
        })?;
        let mut rows = stmt
            .query_named(&[(":table_name", &table_name)])
            .map_err(|e| {
                map_sqlite_err(
                    e,
                    format!(
                        "SQLite raised an error while selecting table `{}`'s metadata",
                        table_name
                    ),
                )
            })?;
        let row = rows
            .next()
            .map_err(|e| {
                map_sqlite_err(
                    e,
                    format!(
                        "SQLite raised an error while fetching row of table `{}`'s metadata",
                        table_name
                    ),
                )
            })?
            .ok_or_else(|| {
                ApllodbError::new(
                    ApllodbErrorKind::UndefinedTable,
                    format!(
                        "table `{}`'s metadata is not visible from this transaction",
                        table_name
                    ),
                    None,
                )
            })?;

        let table_wide_constraints_str: String =
            row.get(CNAME_TABLE_WIDE_CONSTRAINTS).map_err(|e| {
                map_sqlite_err(
                    e,
                    format!(
                        "table `{}`'s metadata row not have column `{}`",
                        table_name, CNAME_TABLE_WIDE_CONSTRAINTS
                    ),
                )
            })?;
        let table_wide_constraints: TableWideConstraints =
            serde_yaml::from_str(&table_wide_constraints_str).map_err(|e| {
                ApllodbError::new(
                    ApllodbErrorKind::DeserializationError,
                    format!(
                        "failed to deserialize table `{}`'s metadata: `{}`",
                        table_name, table_wide_constraints_str
                    ),
                    Some(Box::new(e)),
                )
            })?;

        let vtable = VTable::new(vtable_id.clone(), table_wide_constraints);
        Ok(vtable)
    }

    /// # Failures
    ///
    /// - [DeadlockDetected](error/enum.ApllodbErrorKind.html#variant.DeadlockDetected) when:
    ///   - transaction lock to metadata table takes too long time.
    /// - [DuplicateTable](error/enum.ApllodbErrorKind.html#variant.DuplicateTable) when:
    ///   - `table` is already created.
    /// - [SerializationError](error/enum.ApllodbErrorKind.html#variant.SerializationError) when:
    ///   - Somehow failed to serialize part of [VTable](foobar.html).
    fn insert_into_vtable_metadata(&self, vtable: &VTable) -> ApllodbResult<()> {
        let sql = format!(
            "INSERT INTO {} ({}, {}) VALUES (:table_name, :table_wide_constraints);",
            TNAME_VTABLE_METADATA, CNAME_TABLE_NAME, CNAME_TABLE_WIDE_CONSTRAINTS
        );

        let table_name = format!("{}", vtable.table_name());

        let table_wide_constraints = vtable.table_wide_constraints();
        let table_wide_constraints_str =
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
                (":table_wide_constraints", &table_wide_constraints_str),
            ],
        ) {
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
                Err(map_sqlite_err(
                    e,
                    format!(
                        "SQLite raised an error creating table `{}`'s metadata",
                        table_name
                    ),
                ))
            }
            Ok(_) => Ok(()),
        }
    }

    fn create_navi_table(&self, vtable: &VTable) -> ApllodbResult<()> {
        let sql = CreateTableSqlForNavi::from(vtable);

        self.sqlite_tx
            .execute(sql.as_str(), rusqlite::params![])
            .map(|_| ())
            .map_err(|e| {
                map_sqlite_err(
                    e,
                    format!(
                        "backend sqlite3 raised an error on creating navi table for `{}`",
                        vtable.table_name()
                    ),
                )
            })?;
        Ok(())
    }
}
