mod create_table_sql_for_version;
mod sqlite_table_name_for_version;

pub(in crate::sqlite::transaction::sqlite_tx::dao) use sqlite_table_name_for_version::SqliteTableNameForVersion;

use crate::sqlite::{
    row_iterator::SqliteRowIterator, sqlite_rowid::SqliteRowid, sqlite_types::VRREntriesInVersion,
    transaction::sqlite_tx::SqliteTx,
};
use apllodb_immutable_schema_engine_domain::{
    version::{active_version::ActiveVersion, id::VersionId},
    version_revision_resolver::vrr_id::VRRId,
    vtable::VTable,
};
use apllodb_shared_components::{
    data_structure::ColumnDataType,
    data_structure::ColumnName,
    data_structure::ColumnReference,
    data_structure::{Expression, TableName},
    error::ApllodbResult,
};
use create_table_sql_for_version::CreateTableSqlForVersion;
use std::collections::HashMap;

#[cfg(test)]
pub(in crate::sqlite::transaction::sqlite_tx::dao) use create_table_sql_for_version::test_wrapper::CreateTableSqlForVersionTestWrapper;

#[derive(Debug)]
pub(in crate::sqlite) struct VersionDao<'dao, 'db: 'dao> {
    sqlite_tx: &'dao SqliteTx<'db>,
}

pub(in crate::sqlite::transaction::sqlite_tx::dao) const CNAME_NAVI_ROWID: &str = "_navi_rowid";

impl VersionDao<'_, '_> {
    pub(in crate::sqlite::transaction::sqlite_tx) fn table_name(
        version_id: &VersionId,
        is_active: bool,
    ) -> TableName {
        SqliteTableNameForVersion::new(version_id, is_active).to_full_table_name()
    }
}

impl<'dao, 'db: 'dao> VersionDao<'dao, 'db> {
    pub(in crate::sqlite::transaction::sqlite_tx) fn new(sqlite_tx: &'dao SqliteTx<'db>) -> Self {
        Self { sqlite_tx }
    }

    pub(in crate::sqlite::transaction::sqlite_tx) fn create_table(
        &self,
        version: &ActiveVersion,
    ) -> ApllodbResult<()> {
        let sql = CreateTableSqlForVersion::from(version);
        self.sqlite_tx.execute_named(sql.as_str(), &[])?;
        Ok(())
    }

    /// Fetches only existing columns from SQLite, and makes SqliteRowIterator together with ApparentPrimaryKey from VRREntriesInVersion.
    pub(in crate::sqlite::transaction::sqlite_tx) fn probe_in_version(
        &self,
        vtable: &VTable,
        version: &ActiveVersion,
        vrr_entries_in_version: VRREntriesInVersion<'dao, 'db>,
        projection: &[ColumnName],
    ) -> ApllodbResult<SqliteRowIterator> {
        use crate::sqlite::to_sql_string::ToSqlString;
        use apllodb_immutable_schema_engine_domain::entity::Entity;

        let column_data_types = version.column_data_types();
        // Filter existing and requested columns.
        let existing_projection: Vec<&ColumnDataType> = column_data_types
            .iter()
            .filter(|cdt| projection.contains(&cdt.column_ref().as_column_name()))
            .collect();
        let void_projection: Vec<ColumnReference> = projection
            .iter()
            .filter(|prj_cn| {
                column_data_types
                    .iter()
                    .any(|cdt| cdt.column_ref().as_column_name() == *prj_cn)
            })
            .map(|prj_cn| ColumnReference::new(vtable.table_name().clone(), prj_cn.clone()))
            .collect();
        let sqlite_table_name = Self::table_name(version.id(), true);
        let pk_column_names = vtable.table_wide_constraints().pk_column_names();

        // TODO ここ、JOINするんじゃなくてVersionテーブルだけから取って、rowを結合する
        let sql = format!(
            "
SELECT {pk_column_names}{comma_if_non_pk_column_names}{non_pk_column_names} FROM {version_table}
  INNER JOIN {navi_table}
    ON {version_table}.{version_navi_rowid} = {navi_table}.{navi_rowid}
  WHERE {version_table}.{version_navi_rowid} IN (:navi_rowids)
", // FIXME prevent SQL injection
            pk_column_names = pk_column_names.to_sql_string(),
            comma_if_non_pk_column_names = if existing_projection.is_empty() {
                ""
            } else {
                ", "
            },
            non_pk_column_names = existing_projection.to_sql_string(),
            version_table = sqlite_table_name.to_sql_string(),
            // navi_table = NaviDao::table_name(version.vtable_id()),
            navi_table = "TODO",
            version_navi_rowid = CNAME_NAVI_ROWID,
            // navi_rowid = navi_dao::CNAME_ROWID,
            navi_rowid = "TODO",
        );
        let mut stmt = self.sqlite_tx.prepare(&sql)?;

        let navi_rowids: Vec<SqliteRowid> =
            vrr_entries_in_version.map(|e| e.id().clone()).collect();

        let row_iter = stmt.query_named(
            &[(":navi_rowids", &navi_rowids)],
            &existing_projection,
            &void_projection,
        )?;
        Ok(row_iter)
    }

    pub(in crate::sqlite::transaction::sqlite_tx) fn insert(
        &self,
        version_id: &VersionId,
        vrr_id: &SqliteRowid,
        column_values: &HashMap<ColumnName, Expression>,
    ) -> ApllodbResult<()> {
        use crate::sqlite::to_sql_string::ToSqlString;

        let sqlite_table_name = Self::table_name(version_id, true);
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

        self.sqlite_tx.execute_named(&sql, &vec![])?;

        Ok(())
    }
}
