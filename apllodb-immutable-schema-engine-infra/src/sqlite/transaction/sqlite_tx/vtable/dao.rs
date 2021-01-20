use std::{cell::RefCell, rc::Rc};

use crate::{
    error::InfraError,
    sqlite::{to_sql_string::ToSqlString, transaction::sqlite_tx::SqliteTx},
};
use apllodb_immutable_schema_engine_domain::vtable::{
    constraints::TableWideConstraints, id::VTableId, VTable,
};
use apllodb_shared_components::{
    ApllodbError, ApllodbErrorKind, ApllodbResult, ColumnDataType, ColumnName, ColumnReference,
    SqlType, TableName,
};

#[derive(Debug)]
pub(in crate::sqlite) struct VTableDao<'sqcn> {
    sqlite_tx: Rc<RefCell<SqliteTx<'sqcn>>>,
}

const TNAME: &str = "_vtable_metadata";
const CNAME_TABLE_NAME: &str = "table_name";
const CNAME_TABLE_WIDE_CONSTRAINTS: &str = "table_wide_constraints";

impl<'sqcn> VTableDao<'sqcn> {
    pub(in crate::sqlite) async fn create_table_if_not_exist(
        sqlite_conn: &mut sqlx::SqliteConnection,
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

        sqlx::query(&sql)
            .execute(sqlite_conn)
            .await
            .map_err(InfraError::from)?;

        Ok(())
    }

    pub(in crate::sqlite::transaction::sqlite_tx) fn new(
        sqlite_tx: Rc<RefCell<SqliteTx<'sqcn>>>,
    ) -> Self {
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
    /// - [UndefinedTable](apllodb_shared_components::ApllodbErrorKind::UndefinedTable) when:
    ///   - `table` is not visible from this transaction.
    /// - [DeserializationError](apllodb_shared_components::ApllodbErrorKind::DeserializationError) when:
    ///   - Somehow failed to deserialize part of [VTable](foobar.html).
    pub(in crate::sqlite::transaction::sqlite_tx) async fn select(
        &self,
        vtable_id: &VTableId,
    ) -> ApllodbResult<VTable> {
        let sql = format!(
            "SELECT {}, {} FROM {} WHERE {} = {};",
            CNAME_TABLE_NAME,
            CNAME_TABLE_WIDE_CONSTRAINTS,
            TNAME,
            CNAME_TABLE_NAME,
            vtable_id.table_name().to_sql_string(),
        );

        let mut row_iter = self
            .sqlite_tx
            .borrow_mut()
            .query(
                &sql,
                &[&self.cdt_table_wide_constraints(vtable_id.table_name().clone())],
                &[],
            )
            .await?;
        let mut row = row_iter.next().ok_or_else(|| {
            ApllodbError::new(
                ApllodbErrorKind::UndefinedTable,
                format!(
                    "table `{:?}`'s metadata is not visible from this transaction",
                    vtable_id.table_name()
                ),
                None,
            )
        })?;

        let table_wide_constraints_str: String = row
            .get(&ColumnReference::new(
                vtable_id.table_name().clone(),
                ColumnName::new(CNAME_TABLE_WIDE_CONSTRAINTS)?,
            ))?
            .expect("must be NOT NULL");

        let table_wide_constraints: TableWideConstraints =
            serde_yaml::from_str(&table_wide_constraints_str).map_err(|e| {
                ApllodbError::new(
                    ApllodbErrorKind::DeserializationError,
                    format!(
                        "failed to deserialize table `{:?}`'s metadata: `{:?}`",
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
    /// - [DeadlockDetected](apllodb_shared_components::ApllodbErrorKind::DeadlockDetected) when:
    ///   - transaction lock to metadata table takes too long time.
    /// - [DuplicateTable](apllodb_shared_components::ApllodbErrorKind::DuplicateTable) when:
    ///   - `table` is already created.
    /// - [SerializationError](apllodb_shared_components::ApllodbErrorKind::SerializationError) when:
    ///   - Somehow failed to serialize part of [VTable](foobar.html).
    async fn insert_into_vtable_metadata(&self, vtable: &VTable) -> ApllodbResult<()> {
        let table_wide_constraints = vtable.table_wide_constraints();
        let table_wide_constraints_str =
            serde_yaml::to_string(table_wide_constraints).map_err(|e| {
                ApllodbError::new(
                    ApllodbErrorKind::SerializationError,
                    format!(
                        "failed to serialize `{:?}`'s table wide constraints: `{:?}`",
                        vtable.table_name(),
                        table_wide_constraints
                    ),
                    Some(Box::new(e)),
                )
            })?;

        let sql = format!(
            "
            INSERT INTO {} ({}, {}) VALUES ({table_name}, {table_wide_constraints});
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
                ApllodbErrorKind::UniqueViolation => ApllodbError::new(
                    ApllodbErrorKind::DuplicateTable,
                    format!("table `{:?}` is already created", vtable.table_name()),
                    Some(Box::new(e)),
                ),
                _ => e,
            })?;

        Ok(())
    }

    fn cdt_table_wide_constraints(&self, table_name: TableName) -> ColumnDataType {
        ColumnDataType::new(
            ColumnReference::new(
                table_name,
                ColumnName::new(CNAME_TABLE_WIDE_CONSTRAINTS).unwrap(),
            ),
            SqlType::text(),
            false,
        )
    }
}
