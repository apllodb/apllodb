use crate::{transaction::sqlite_tx::sqlite_table_name::SqliteTableNameForTable, Table};
use apllodb_shared_components::error::{ApllodbError, ApllodbErrorKind, ApllodbResult};

#[derive(Debug)]
pub(in crate::transaction::sqlite_tx) struct TableDao<'tx> {
    sqlite_tx: &'tx rusqlite::Transaction<'tx>,
}

impl<'tx> TableDao<'tx> {
    pub(in crate::transaction::sqlite_tx) fn new(
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
    pub(in crate::transaction::sqlite_tx) fn create(&self, table: &Table) -> ApllodbResult<()> {
        let metadata_table = SqliteTableNameForTable::name();
        let sql = format!(
            "INSERT INTO {} (table_name, table_wide_constraints) VALUES (:table_name, :table_wide_constraints);",
            &metadata_table,
        );

        let table_name = format!("{}", table.name());

        let table_wide_constraints = table.table_wide_constraints();
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
            ) => Err(ApllodbError::new(  // TODO TableDao::create() 意外にも現れは汎用的なエラー処理ななんとかする
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
                        extended_code: 1555, // SQLITE_CONSTRAINT_PRIMARYKEY
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
                println!("{:?}", e);
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
