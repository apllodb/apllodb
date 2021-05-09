use std::{cell::RefCell, rc::Rc};

use crate::{
    error::InfraError,
    sqlite::{to_sql_string::ToSqlString, transaction::sqlite_tx::SqliteTx},
};
use apllodb_immutable_schema_engine_domain::vtable::{
    constraints::TableWideConstraints, id::VTableId, VTable,
};
use apllodb_shared_components::{
    ApllodbError, ApllodbResult, Schema, SchemaIndex, SqlState, SqlType,
};
use apllodb_storage_engine_interface::{ColumnDataType, ColumnName, TableName};

#[derive(Debug)]
pub(in crate::sqlite) struct VTableMetadataDao {
    sqlite_tx: Rc<RefCell<SqliteTx>>,
}

const TNAME: &str = "_vtable_metadata";
const CNAME_TABLE_NAME: &str = "table_name";
const CNAME_TABLE_WIDE_CONSTRAINTS: &str = "table_wide_constraints";

impl VTableMetadataDao {
    pub(in crate::sqlite) async fn create_table(
        sqlite_conn: &mut sqlx::SqliteConnection,
    ) -> ApllodbResult<()> {
        let sql = format!(
            "
CREATE TABLE {} (
  {} TEXT PRIMARY KEY,
  {} TEXT NOT NULL
)
        ",
            TNAME, CNAME_TABLE_NAME, CNAME_TABLE_WIDE_CONSTRAINTS
        );

        sqlx::query(&sql)
            .execute(sqlite_conn)
            .await
            .map_err(InfraError::from)?;

        Ok(())
    }

    pub(in crate::sqlite::transaction::sqlite_tx) fn new(sqlite_tx: Rc<RefCell<SqliteTx>>) -> Self {
        Self { sqlite_tx }
    }

    /// # Failures
    ///
    /// - Errors from insert_into_vtable_metadata()
    pub(in crate::sqlite::transaction::sqlite_tx) async fn insert(
        &self,
        vtable: &VTable,
    ) -> ApllodbResult<()> {
        self.insert_into_vtable_metadata(vtable).await?;
        Ok(())
    }

    /// # Failures
    ///
    /// - [NameErrorNotFound](apllodb_shared_components::SqlState::NameErrorNotFound) when:
    ///   - `table` is not visible from this transaction.
    /// - [SystemError](apllodb_shared_components::SqlState::SystemError) when:
    ///   - Somehow failed to deserialize part of [VTable](foobar.html).
    pub(in crate::sqlite::transaction::sqlite_tx) async fn select(
        &self,
        vtable_id: &VTableId,
    ) -> ApllodbResult<VTable> {
        let sql = format!(
            "SELECT {}, {} FROM {} WHERE {} = \"{}\";",
            CNAME_TABLE_NAME,
            CNAME_TABLE_WIDE_CONSTRAINTS,
            TNAME,
            CNAME_TABLE_NAME,
            vtable_id.table_name().to_sql_string(),
        );

        let tname = TableName::new(TNAME)?;

        let mut rows = self
            .sqlite_tx
            .borrow_mut()
            .query(&sql, &tname, &[&self.cdt_table_wide_constraints()], &[])
            .await?;

        let (pos_table_wide_constraints, _) = rows
            .as_schema()
            .index(&SchemaIndex::from(CNAME_TABLE_WIDE_CONSTRAINTS))?;

        let table_wide_constraints_str: String = rows
            .next()
            .ok_or_else(|| {
                ApllodbError::name_error_not_found(format!(
                    "table `{:?}`'s metadata is not visible from this transaction",
                    vtable_id.table_name()
                ))
            })
            .and_then(|row| row.get(pos_table_wide_constraints))?
            .expect("must be NOT NULL");

        let table_wide_constraints: TableWideConstraints =
            serde_yaml::from_str(&table_wide_constraints_str).map_err(|e| {
                ApllodbError::system_error(
                    format!(
                        "failed to deserialize table `{:?}`'s metadata: `{:?}`",
                        vtable_id.table_name(),
                        table_wide_constraints_str
                    ),
                    Box::new(e),
                )
            })?;

        let vtable = VTable::new(vtable_id.clone(), table_wide_constraints);
        Ok(vtable)
    }

    /// # Failures
    ///
    /// - [TransactionRollbackDeadlock](apllodb_shared_components::SqlState::TransactionRollbackDeadlock) when:
    ///   - transaction lock to metadata table takes too long time.
    /// - [NameErrorDuplicate](apllodb_shared_components::SqlState::NameErrorDuplicate) when:
    ///   - `table` is already created.
    /// - [SystemError](apllodb_shared_components::SqlState::SystemError) when:
    ///   - Somehow failed to serialize part of [VTable](foobar.html).
    async fn insert_into_vtable_metadata(&self, vtable: &VTable) -> ApllodbResult<()> {
        let table_wide_constraints = vtable.table_wide_constraints();
        let table_wide_constraints_str =
            serde_yaml::to_string(table_wide_constraints).map_err(|e| {
                ApllodbError::system_error(
                    format!(
                        "failed to serialize `{:?}`'s table wide constraints: `{:?}`",
                        vtable.table_name(),
                        table_wide_constraints
                    ),
                    Box::new(e),
                )
            })?;

        let sql = format!(
            "
            INSERT INTO {} ({}, {}) VALUES (\"{table_name}\", \"{table_wide_constraints}\");
            ",
            TNAME,
            CNAME_TABLE_NAME,
            CNAME_TABLE_WIDE_CONSTRAINTS,
            table_name = vtable.table_name().to_sql_string(),
            table_wide_constraints = table_wide_constraints_str
        );

        self.sqlite_tx
            .borrow_mut()
            .execute(&sql)
            .await
            .map_err(|e| match e.kind() {
                SqlState::IntegrityConstraintUniqueViolation => ApllodbError::name_error_duplicate(
                    format!("table `{:?}` is already created", vtable.table_name()),
                ),
                _ => e,
            })?;

        Ok(())
    }

    fn cdt_table_wide_constraints(&self) -> ColumnDataType {
        ColumnDataType::new(
            ColumnName::new(CNAME_TABLE_WIDE_CONSTRAINTS).unwrap(),
            SqlType::text(),
            false,
        )
    }
}
