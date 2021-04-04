mod model;

use crate::{error::InfraError, sqlite::transaction::sqlite_tx::SqliteTx};
use model::VersionMetadataModel;
use std::{cell::RefCell, convert::TryFrom, rc::Rc};

use apllodb_immutable_schema_engine_domain::{
    version::{active_version::ActiveVersion, id::VersionId},
    vtable::id::VTableId,
};
use apllodb_shared_components::{
    ApllodbError, ApllodbErrorKind, ApllodbResult, ColumnDataType, ColumnName, SqlType, TableName,
};

#[derive(Debug)]
pub(in crate::sqlite) struct VersionMetadataDao {
    sqlite_tx: Rc<RefCell<SqliteTx>>,
}

const TNAME: &str = "_version_metadata";
const CNAME_TABLE_NAME: &str = "table_name";
const CNAME_VERSION_NUMBER: &str = "version_number";
const CNAME_COLUMN_DATA_TYPES: &str = "column_data_types";
const CNAME_VERSION_CONSTRAINTS: &str = "version_constraints";
const CNAME_IS_ACTIVE: &str = "is_active";

impl VersionMetadataDao {
    pub(in crate::sqlite) async fn create_table(
        sqlite_conn: &mut sqlx::SqliteConnection,
    ) -> ApllodbResult<()> {
        let sql = format!(
            "
CREATE TABLE {tname} (
  {cname_table_name} TEXT NOT NULL,
  {cname_version_number} INTEGER NOT NULL,
  {cname_column_data_types} TEXT NOT NULL,
  {cname_version_constraints} TEXT NOT NULL,
  {cname_is_active} BOOLEAN NOT NULL,
  PRIMARY KEY ({cname_table_name}, {cname_version_number})
)
        ",
            tname = TNAME,
            cname_table_name = CNAME_TABLE_NAME,
            cname_version_number = CNAME_VERSION_NUMBER,
            cname_column_data_types = CNAME_COLUMN_DATA_TYPES,
            cname_version_constraints = CNAME_VERSION_CONSTRAINTS,
            cname_is_active = CNAME_IS_ACTIVE
        );

        sqlx::query(&sql)
            .execute(sqlite_conn)
            .await
            .map_err(InfraError::from)?;

        Ok(())
    }

    pub(in crate::sqlite) fn new(sqlite_tx: Rc<RefCell<SqliteTx>>) -> Self {
        Self { sqlite_tx }
    }

    /// # Returns
    ///
    /// Ascending-ordered vec of key: (table_name, version_number)
    pub(in crate::sqlite) async fn select_active_versions(
        &self,
        vtable_id: &VTableId,
    ) -> ApllodbResult<Vec<ActiveVersion>> {
        let sql = format!(
            r#"
            SELECT {cname_table_name}, {cname_version_number}, {cname_column_data_types}, {cname_version_constraints}, {cname_is_active}
              FROM {tname}
              WHERE {cname_table_name} = "{table_name}" AND {cname_is_active}
              ORDER BY {cname_table_name}, {cname_version_number}
            "#,
            tname = TNAME,
            cname_table_name = CNAME_TABLE_NAME,
            cname_version_number = CNAME_VERSION_NUMBER,
            cname_column_data_types = CNAME_COLUMN_DATA_TYPES,
            cname_version_constraints = CNAME_VERSION_CONSTRAINTS,
            cname_is_active = CNAME_IS_ACTIVE,
            table_name = vtable_id.table_name().as_str(),
        );

        let tname = TableName::new(TNAME)?;

        let models: Vec<VersionMetadataModel> = self
            .sqlite_tx
            .borrow_mut()
            .query(
                &sql,
                &tname,
                &[
                    &self.cdt_table_name(),
                    &self.cdt_version_number(),
                    &self.cdt_column_data_types(),
                    &self.cdt_version_constraints(),
                    &self.cdt_is_active(),
                ],
                &[],
            )
            .await?
            .map(VersionMetadataModel::try_from)
            .collect::<ApllodbResult<_>>()?;

        models
            .into_iter()
            .map(|m| m.into_active_version(vtable_id))
            .collect()
    }

    // TODO 消す
    pub(in crate::sqlite) async fn select_active_version(
        &self,
        vtable_id: &VTableId,
        version_id: &VersionId,
    ) -> ApllodbResult<ActiveVersion> {
        use apllodb_immutable_schema_engine_domain::entity::Entity;

        let versions = self.select_active_versions(vtable_id).await?;
        versions
            .into_iter()
            .find(|v| v.id() == version_id)
            .ok_or_else(|| {
                ApllodbError::new(
                    ApllodbErrorKind::UndefinedTable,
                    format!(
                        "table `{:?}` not found (or every version is inactive)",
                        vtable_id.table_name()
                    ),
                    None,
                )
            })
    }

    /// # Failures
    ///
    /// - [SerializationError](apllodb_shared_components::ApllodbErrorKind::SerializationError) when:
    ///   - Somehow failed to serialize part of [VTable](foobar.html).
    pub(in crate::sqlite) async fn insert(&self, version: &ActiveVersion) -> ApllodbResult<()> {
        let model = VersionMetadataModel::from(version);

        let sql = format!(
            r#"
            INSERT INTO {tname} ({cname_table_name}, {cname_version_number}, {cname_column_data_types}, {cname_version_constraints}, {cname_is_active})
              VALUES ("{table_name}", {version_number}, {column_data_types}, {version_constraints}, {is_active});
            "#,
            tname = TNAME,
            cname_table_name = CNAME_TABLE_NAME,
            cname_version_number = CNAME_VERSION_NUMBER,
            cname_column_data_types = CNAME_COLUMN_DATA_TYPES,
            cname_version_constraints = CNAME_VERSION_CONSTRAINTS,
            cname_is_active = CNAME_IS_ACTIVE,
            table_name = model.serialized_table_name(),
            version_number = model.serialized_version_number(),
            column_data_types = model.serialized_column_data_types()?,
            version_constraints = model.serialized_version_constraints()?,
            is_active = model.serialized_is_active()
        );

        self.sqlite_tx
            .borrow_mut()
            .execute(&sql)
            .await
            .map_err(|e| match e.kind() {
                ApllodbErrorKind::UniqueViolation => ApllodbError::new(
                    ApllodbErrorKind::DuplicateTable,
                    format!(
                        "table `{:?}`, version `{:?}` is already created",
                        &model.table_name, &model.version_number
                    ),
                    Some(Box::new(e)),
                ),
                _ => e,
            })?;

        Ok(())
    }

    fn cdt_table_name(&self) -> ColumnDataType {
        ColumnDataType::new(
            ColumnName::new(CNAME_TABLE_NAME).unwrap(),
            SqlType::text(),
            false,
        )
    }
    fn cdt_version_number(&self) -> ColumnDataType {
        ColumnDataType::new(
            ColumnName::new(CNAME_VERSION_NUMBER).unwrap(),
            SqlType::integer(),
            false,
        )
    }
    fn cdt_column_data_types(&self) -> ColumnDataType {
        ColumnDataType::new(
            ColumnName::new(CNAME_COLUMN_DATA_TYPES).unwrap(),
            SqlType::text(),
            false,
        )
    }
    fn cdt_version_constraints(&self) -> ColumnDataType {
        ColumnDataType::new(
            ColumnName::new(CNAME_VERSION_CONSTRAINTS).unwrap(),
            SqlType::text(),
            false,
        )
    }
    fn cdt_is_active(&self) -> ColumnDataType {
        ColumnDataType::new(
            ColumnName::new(CNAME_IS_ACTIVE).unwrap(),
            SqlType::boolean(),
            false,
        )
    }
}
