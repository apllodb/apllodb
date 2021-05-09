pub(in crate::sqlite::transaction::sqlite_tx) mod create_table_sql_for_version;
pub(in crate::sqlite::transaction::sqlite_tx) mod sqlite_table_name_for_version;

use crate::sqlite::{
    sqlite_rowid::SqliteRowid, sqlite_types::VrrEntriesInVersion, to_sql_string::ToSqlString,
    transaction::sqlite_tx::SqliteTx,
};
use apllodb_immutable_schema_engine_domain::{
    entity::Entity,
    row::pk::apparent_pk::ApparentPrimaryKey,
    row_projection_result::RowProjectionResult,
    version::{active_version::ActiveVersion, id::VersionId},
};
use apllodb_shared_components::{
    ApllodbError, ApllodbResult, Schema, SchemaIndex, SqlState, SqlType, SqlValue,
};
use apllodb_storage_engine_interface::{
    ColumnDataType, ColumnName, Row, RowSchema, Rows, TableName,
};
use create_table_sql_for_version::CreateTableSqlForVersion;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use self::sqlite_table_name_for_version::SqliteTableNameForVersion;

pub(in crate::sqlite::transaction::sqlite_tx) const CNAME_NAVI_ROWID: &str = "_navi_rowid";

#[derive(Debug)]
pub(in crate::sqlite) struct VersionDao {
    sqlite_tx: Rc<RefCell<SqliteTx>>,
}

struct Attributes(HashMap<ColumnName, SqlValue>);
impl Attributes {
    fn join(self, attr: Self) -> Self {
        let h: HashMap<ColumnName, SqlValue> = self.0.into_iter().chain(attr.0).collect();
        Self(h)
    }

    /// # Failures
    ///
    /// - [NameErrorNotFound](apllodb_shared_components::SqlState::NameErrorNotFound) when:
    ///   - schema has unregistered ColumnName.
    fn into_row(mut self, schema: &RowSchema) -> ApllodbResult<Row> {
        let sql_values: Vec<SqlValue> = schema
            .table_column_names()
            .iter()
            .map(|tc| {
                self.0.remove(tc.as_column_name()).ok_or_else(|| {
                    ApllodbError::name_error_not_found(format!(
                        "column `{}` does not exist in this Attributes",
                        tc.as_column_name().as_str()
                    ))
                })
            })
            .collect::<ApllodbResult<_>>()?;

        Ok(Row::new(sql_values))
    }
}

impl VersionDao {
    pub(in crate::sqlite::transaction::sqlite_tx) fn table_name(
        version_id: &VersionId,
    ) -> TableName {
        SqliteTableNameForVersion::new(version_id).to_full_table_name()
    }

    pub(in crate::sqlite::transaction::sqlite_tx) fn new(sqlite_tx: Rc<RefCell<SqliteTx>>) -> Self {
        Self { sqlite_tx }
    }

    pub(in crate::sqlite::transaction::sqlite_tx) async fn create_table(
        &self,
        version: &ActiveVersion,
    ) -> ApllodbResult<()> {
        let sql = CreateTableSqlForVersion::from(version);
        self.sqlite_tx.borrow_mut().execute(sql.as_str()).await?;
        Ok(())
    }

    /// Fetches only existing columns from SQLite, and makes Rows together with ApparentPrimaryKey from VrrEntriesInVersion.
    pub(in crate::sqlite::transaction::sqlite_tx) async fn probe_in_version(
        &self,
        version: &ActiveVersion,
        vrr_entries_in_version: VrrEntriesInVersion,
        projection: &RowProjectionResult,
    ) -> ApllodbResult<Rows> {
        let ret_schema = RowSchema::from(projection.clone());

        let (navi_rowids, pks): (Vec<SqliteRowid>, Vec<ApparentPrimaryKey>) =
            vrr_entries_in_version
                .map(|e| (e.id().clone(), e.into_pk()))
                .unzip();

        let mut pk_attrs = self.pk_attrs(&navi_rowids, pks);

        let non_pk_eff_prj = projection.non_pk_effective_projection(version.id())?;
        let non_pk_void_prj = projection.non_pk_void_projection(version.id())?;

        let all_attrs: HashMap<SqliteRowid, Attributes> =
            if non_pk_eff_prj.is_empty() && non_pk_void_prj.is_empty() {
                pk_attrs
            } else {
                let mut non_pk_attrs = self
                    .non_pk_attrs(version, &navi_rowids, &non_pk_eff_prj, &non_pk_void_prj)
                    .await?;
                navi_rowids
                    .iter()
                    .map(|navi_rowid| {
                        let pk_attrs = pk_attrs.remove(navi_rowid).expect("checked");
                        let non_pk_attrs = non_pk_attrs.remove(navi_rowid).expect("checked");
                        (navi_rowid.clone(), pk_attrs.join(non_pk_attrs))
                    })
                    .collect()
            };

        let rows: Vec<Row> = all_attrs
            .into_iter()
            .map(|(_, attrs)| attrs.into_row(&ret_schema))
            .collect::<ApllodbResult<_>>()?;

        Ok(Rows::new(ret_schema, rows))
    }

    pub(in crate::sqlite::transaction::sqlite_tx) async fn insert(
        &self,
        version_id: &VersionId,
        vrr_id: &SqliteRowid,
        column_values: &HashMap<ColumnName, SqlValue>,
    ) -> ApllodbResult<()> {
        let sqlite_table_name = Self::table_name(version_id);
        let sql = format!(
            "
        INSERT INTO {tname}
          ({navi_rowid}{comma_if_non_pk_column_names}{non_pk_column_names})
          VALUES ({navi_rowid_val}{comma_if_non_pk_column_names}{non_pk_column_values})
        ", // FIXME might lead to SQL injection.
            tname = sqlite_table_name.as_str(),
            navi_rowid = CNAME_NAVI_ROWID,
            navi_rowid_val = vrr_id.0,
            comma_if_non_pk_column_names = if column_values.is_empty() { "" } else { ", " },
            non_pk_column_names = column_values.keys().collect::<Vec<_>>().to_sql_string(),
            non_pk_column_values = column_values.values().collect::<Vec<_>>().to_sql_string(),
        );

        self.sqlite_tx.borrow_mut().execute(&sql).await?;

        Ok(())
    }

    fn pk_attrs(
        &self,
        navi_rowids: &[SqliteRowid],
        pks: Vec<ApparentPrimaryKey>,
    ) -> HashMap<SqliteRowid, Attributes> {
        navi_rowids
            .iter()
            .zip(pks.into_iter())
            .map(|(navi_rowid, pk)| {
                let inner: HashMap<ColumnName, SqlValue> = pk
                    .into_zipped()
                    .into_iter()
                    .map(|(cn, nn_sql_value)| (cn, SqlValue::NotNull(nn_sql_value)))
                    .collect();
                (navi_rowid.clone(), Attributes(inner))
            })
            .collect()
    }

    /// # Panics
    ///
    /// If both non_pk_eff_prj and non_pk_void_prj are empty
    async fn non_pk_attrs(
        &self,
        version: &ActiveVersion,
        navi_rowids: &[SqliteRowid],
        non_pk_eff_prj: &[ColumnName],
        non_pk_void_prj: &[ColumnName],
    ) -> ApllodbResult<HashMap<SqliteRowid, Attributes>> {
        assert!(!non_pk_eff_prj.is_empty() || !non_pk_void_prj.is_empty());

        let sqlite_table_name = Self::table_name(version.id());

        let sql = format!(
            "
SELECT {version_navi_rowid}{comma_if_non_pk_column}{non_pk_column_names}{comma_if_void_projection}{void_projection} FROM {version_table}
WHERE {version_navi_rowid} IN ({navi_rowids})
", // FIXME prevent SQL injection
            comma_if_non_pk_column = if non_pk_eff_prj.is_empty() {
                ""
            } else {
                ", "
            },
            non_pk_column_names = non_pk_eff_prj
                .to_sql_string(),
            comma_if_void_projection = if non_pk_void_prj.is_empty() {""} else {", "},
            void_projection = non_pk_void_prj
            .iter()
            .map(|cn| format!("NULL {}", cn.to_sql_string()))
            .collect::<Vec<_>>()
            .to_sql_string(),
            version_table = sqlite_table_name.to_sql_string(),
            version_navi_rowid = CNAME_NAVI_ROWID,
            navi_rowids=navi_rowids.to_sql_string(),
        );

        let mut effective_prj_cdts: Vec<&ColumnDataType> = version
            .column_data_types()
            .iter()
            .filter(|cdt| non_pk_eff_prj.contains(cdt.column_name()))
            .collect();
        let cdt_navi_rowid = self.cdt_navi_rowid();

        let non_pk_eff_cdt_with_navi_rowid = {
            effective_prj_cdts.push(&cdt_navi_rowid);
            effective_prj_cdts
        };

        let rows_from_version = self
            .sqlite_tx
            .borrow_mut()
            .query(
                &sql,
                version.vtable_id().table_name(),
                &non_pk_eff_cdt_with_navi_rowid,
                non_pk_void_prj,
            )
            .await?;

        let schema_from_version = rows_from_version.as_schema().clone();
        let ret: HashMap<SqliteRowid, Attributes> = rows_from_version
            .map(|row| {
                let (navi_rowid_pos, _) =
                    schema_from_version.index(&SchemaIndex::from(CNAME_NAVI_ROWID))?;
                let rowid = row.get(navi_rowid_pos)?.expect("must be NOT NULL");

                let attrs = schema_from_version
                    .table_column_names_with_pos()
                    .into_iter()
                    .map(|(pos, tc)| {
                        let v = row.get_sql_value(pos)?;
                        Ok((tc.as_column_name().clone(), v.clone()))
                    })
                    .collect::<ApllodbResult<_>>()?;

                Ok((rowid, Attributes(attrs)))
            })
            .collect::<ApllodbResult<_>>()?;

        Ok(ret)
    }
}

impl VersionDao {
    fn cdt_navi_rowid(&self) -> ColumnDataType {
        ColumnDataType::new(
            ColumnName::new(CNAME_NAVI_ROWID).unwrap(),
            SqlType::big_int(),
            false,
        )
    }
}
