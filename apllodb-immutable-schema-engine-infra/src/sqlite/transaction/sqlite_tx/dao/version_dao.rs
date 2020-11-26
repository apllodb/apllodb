mod create_table_sql_for_version;
mod sqlite_table_name_for_version;

pub(in crate::sqlite::transaction::sqlite_tx::dao) use sqlite_table_name_for_version::SqliteTableNameForVersion;

use crate::{
    external_interface::ApllodbImmutableSchemaEngine,
    sqlite::{
        row_iterator::SqliteRowIterator, sqlite_rowid::SqliteRowid, sqlite_types::SqliteTypes,
        transaction::sqlite_tx::SqliteTx,
    },
};
use apllodb_immutable_schema_engine_domain::{
    version::{active_version::ActiveVersion, id::VersionId},
    version_revision_resolver::vrr_entries_in_version::VRREntriesInVersion,
    vtable::VTable,
};
use apllodb_shared_components::{data_structure::ColumnDataType, data_structure::ColumnName, data_structure::{Expression, TableName}, error::ApllodbResult, data_structure::ColumnReference};
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
        vrr_entries_in_version: VRREntriesInVersion<
            'dao,
            'db,
            ApllodbImmutableSchemaEngine,
            SqliteTypes,
        >,
        projection: &[ColumnName],
    ) -> ApllodbResult<SqliteRowIterator> {
        use crate::sqlite::to_sql_string::ToSqlString;
        use apllodb_immutable_schema_engine_domain::entity::Entity;

        let projection: Vec<String> = projection
            .iter()
            .map(|cn| cn.as_str().to_string())
            .collect();
        let column_data_types = version.column_data_types();
        // Filter existing and requested columns.
        // FIXME これのせいで、v2の c1==NULL が現れないで困っている。
        // SQLitのレイヤで NULL as c1 とやるか、SQLiteはあくまでもそんなカラムは知らんと返し、ImmutableRowに変換する時にNULLをぶち込むか。
        let existing_projection: Vec<&ColumnDataType> = column_data_types
            .iter()
            .filter(|cdt| {
                projection.contains(&cdt.column_ref().as_column_name().as_str().to_string())
            })
            .collect();
        let void_projection: Vec<ColumnReference> = projection
            .iter()
            .filter(|prj_cn| {
                column_data_types
                    .iter()
                    .any(|cdt| cdt.column_ref().as_column_name().as_str() == **prj_cn)
            })
            .cloned()
            .collect();
        let sqlite_table_name = Self::table_name(version.id(), true);
        let pk_column_names = vtable.table_wide_constraints().pk_column_names();
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
            non_pk_column_names = existing_projection
                .iter()
                .map(|non_pk_cdt| {
                    let non_pk_cn = non_pk_cdt.column_ref().as_column_name();
                    non_pk_cn.as_str().into()
                })
                .collect::<Vec<String>>()
                .join(", "),
            version_table = sqlite_table_name.to_sql_string(),
            navi_table = NaviDao::table_name(version.vtable_id()),
            version_navi_rowid = CNAME_NAVI_ROWID,
            navi_rowid = navi_dao::CNAME_ROWID,
        );
        let mut stmt = self.sqlite_tx.prepare(&sql)?;

        let navi_rowids:  = vrr_entries_in_version.map(|e| {
            e.id()
        }
        ).collect();
        
        let row_iter = stmt.query_named(
            &[(":navi_rowids", &navi_rowids)],
            &vtable
                .table_wide_constraints()
                .pk_column_data_types()
                .iter()
                .map(|pk_cdt| pk_cdt)
                .collect::<Vec<&PKColumnDataType>>(),
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
            non_pk_column_names = column_values
                .keys()
                .map(|cn| cn.as_str())
                .collect::<Vec<&str>>()
                .join(", "),
            non_pk_column_values = column_values
                .values()
                .map(|expr| expr.to_sql_string())
                .collect::<Vec<String>>()
                .join(", "),
        );

        self.sqlite_tx.execute_named(&sql, &vec![])?;

        Ok(())
    }
}
