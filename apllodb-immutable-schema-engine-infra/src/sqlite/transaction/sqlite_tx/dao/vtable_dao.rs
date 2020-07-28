use crate::sqlite::{sqlite_error::map_sqlite_err, SqliteTx};
use apllodb_immutable_schema_engine_domain::{
    row::column::non_pk_column::{NonPKColumnDataType, NonPKColumnName},
    TableWideConstraints, VTable, VTableId,
};
use apllodb_shared_components::{
    data_structure::{ColumnName, DataType, DataTypeKind},
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};

#[derive(Debug)]
pub(in crate::sqlite) struct VTableDao<'tx, 'db: 'tx> {
    sqlite_tx: &'tx SqliteTx<'db>,
}

const TNAME: &str = "_vtable_metadata";
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
  {} TEXT NOT NULL
)
        ",
            TNAME, CNAME_TABLE_NAME, CNAME_TABLE_WIDE_CONSTRAINTS
        );

        sqlite_conn
            .execute(sql.as_str(), rusqlite::params![])
            .map(|_| ())
            .map_err(|e| {
                map_sqlite_err(
                    e,
                    format!(
                        "backend sqlite3 raised an error on creating metadata table `{}`",
                        TNAME
                    ),
                )
            })?;
        Ok(())
    }

    pub(in crate::sqlite::transaction::sqlite_tx) fn new(sqlite_tx: &'tx SqliteTx<'db>) -> Self {
        Self { sqlite_tx }
    }

    /// # Failures
    ///
    /// - Errors from insert_into_vtable_metadata()
    pub(in crate::sqlite::transaction::sqlite_tx) fn insert(
        &self,
        vtable: &VTable,
    ) -> ApllodbResult<()> {
        self.insert_into_vtable_metadata(vtable)?;
        Ok(())
    }

    /// # Failures
    ///
    /// - [UndefinedTable](error/enum.ApllodbErrorKind.html#variant.UndefinedTable) when:
    ///   - `table` is not visible from this transaction.
    /// - [DeserializationError](error/enum.ApllodbErrorKind.html#variant.DeserializationError) when:
    ///   - Somehow failed to deserialize part of [VTable](foobar.html).
    pub(in crate::sqlite::transaction::sqlite_tx) fn select(
        &self,
        vtable_id: &VTableId,
    ) -> ApllodbResult<VTable> {
        use apllodb_storage_engine_interface::Row;

        let sql = format!(
            "SELECT {}, {} FROM {} WHERE {} = :table_name;",
            CNAME_TABLE_NAME, CNAME_TABLE_WIDE_CONSTRAINTS, TNAME, CNAME_TABLE_NAME
        );

        let mut stmt = self.sqlite_tx.prepare(&sql)?;
        let mut row_iter = stmt.query_named(
            &[(":table_name", vtable_id.table_name())],
            &[],
            &vec![&self.cdt_table_wide_constraints()],
        )?;
        let row = row_iter.next().ok_or_else(|| {
            ApllodbError::new(
                ApllodbErrorKind::UndefinedTable,
                format!(
                    "table `{}`'s metadata is not visible from this transaction",
                    vtable_id.table_name()
                ),
                None,
            )
        })??;

        let table_wide_constraints_str: String =
            row.get(&ColumnName::new(CNAME_TABLE_WIDE_CONSTRAINTS)?)?;

        let table_wide_constraints: TableWideConstraints =
            serde_yaml::from_str(&table_wide_constraints_str).map_err(|e| {
                ApllodbError::new(
                    ApllodbErrorKind::DeserializationError,
                    format!(
                        "failed to deserialize table `{}`'s metadata: `{}`",
                        vtable_id.table_name(),
                        table_wide_constraints_str
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
            TNAME, CNAME_TABLE_NAME, CNAME_TABLE_WIDE_CONSTRAINTS
        );

        let table_wide_constraints = vtable.table_wide_constraints();
        let table_wide_constraints_str =
            serde_yaml::to_string(table_wide_constraints).map_err(|e| {
                ApllodbError::new(
                    ApllodbErrorKind::SerializationError,
                    format!(
                        "failed to serialize `{}`'s table wide constraints: `{:?}`",
                        vtable.table_name(),
                        table_wide_constraints
                    ),
                    Some(Box::new(e)),
                )
            })?;

        self.sqlite_tx
            .execute_named(
                &sql,
                &[
                    (":table_name", vtable.table_name()),
                    (":table_wide_constraints", &table_wide_constraints_str),
                ],
            )
            .map_err(|e| match e.kind() {
                ApllodbErrorKind::UniqueViolation => ApllodbError::new(
                    ApllodbErrorKind::DuplicateTable,
                    format!("table `{}` is already created", vtable.table_name()),
                    Some(Box::new(e)),
                ),
                _ => e,
            })?;

        Ok(())
    }

    fn cdt_table_wide_constraints(&self) -> NonPKColumnDataType {
        NonPKColumnDataType::new(
            NonPKColumnName::new(CNAME_TABLE_WIDE_CONSTRAINTS).unwrap(),
            DataType::new(DataTypeKind::Text, false),
        )
    }
}
