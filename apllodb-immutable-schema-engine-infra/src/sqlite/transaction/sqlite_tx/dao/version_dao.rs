mod create_table_sql_for_version;
mod sqlite_table_name_for_version;

pub(in crate::sqlite::transaction::sqlite_tx::dao) use sqlite_table_name_for_version::SqliteTableNameForVersion;

use super::{navi_dao, NaviDao};
use crate::sqlite::{sqlite_rowid::SqliteRowid, SqliteRowIterator, SqliteTx};
use apllodb_immutable_schema_engine_domain::{
    ActiveVersion, PKColumnNames, NonPKColumnDataType, NonPKColumnName, VersionId,
};
use apllodb_shared_components::{
    data_structure::{ColumnDataType, ColumnName, Expression, TableName},
    error::ApllodbResult,
};
use create_table_sql_for_version::CreateTableSqlForVersion;
use std::collections::HashMap;

#[cfg(test)]
pub(in crate::sqlite::transaction::sqlite_tx::dao) use create_table_sql_for_version::test_wrapper::CreateTableSqlForVersionTestWrapper;

#[derive(Debug)]
pub(in crate::sqlite) struct VersionDao<'tx, 'db: 'tx> {
    sqlite_tx: &'tx SqliteTx<'db>,
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

impl<'tx, 'db: 'tx> VersionDao<'tx, 'db> {
    pub(in crate::sqlite::transaction::sqlite_tx) fn new(sqlite_tx: &'tx SqliteTx<'db>) -> Self {
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

    /// Fetches only existing columns from SQLite, joining ApparentPrimaryKey from navi table.
    pub(in crate::sqlite::transaction::sqlite_tx) fn join_with_navi(
        &self,
        version: &ActiveVersion,
        navi_rowids: &[SqliteRowid],
        apk_column_names: &PKColumnNames,
        non_pk_column_names: &[NonPKColumnName],
    ) -> ApllodbResult<SqliteRowIterator> {
        use crate::sqlite::to_sql_string::ToSqlString;
        use apllodb_immutable_schema_engine_domain::Entity;

        let non_pk_column_names: Vec<ColumnName> =
            non_pk_column_names.iter().map(|cn| cn.0.clone()).collect();

        let non_pk_column_data_types = version.column_data_types();
        // Filter existing and requested columns.
        let non_pk_column_data_types: Vec<&NonPKColumnDataType> = non_pk_column_data_types
            .iter()
            .filter(|non_pk_cdt| {
                let cdt = &non_pk_cdt.0;
                non_pk_column_names.contains(cdt.column_name())
            })
            .collect();

        let sqlite_table_name = Self::table_name(version.id(), true);

        let sql = format!(
            "
SELECT {apk_column_names}{comma_if_non_pk_column_names}{non_pk_column_names} FROM {version_table}
  INNER JOIN {navi_table}
    ON {version_table}.{version_navi_rowid} = {navi_table}.{navi_rowid}
  WHERE {version_table}.{version_navi_rowid} IN (:navi_rowids)
", // FIXME prevent SQL injection
            apk_column_names = apk_column_names.to_sql_string(),
            comma_if_non_pk_column_names = if non_pk_column_data_types.is_empty() {
                ""
            } else {
                ", "
            },
            non_pk_column_names = non_pk_column_data_types
                .iter()
                .map(|non_pk_cdt| {
                    let non_pk_cn = non_pk_cdt.column_name();
                    let cn = non_pk_cn.0;
                    cn.as_str().into()
                })
                .collect::<Vec<String>>()
                .join(", "),
            version_table = sqlite_table_name.to_sql_string(),
            navi_table = NaviDao::table_name(version.vtable_id())?.to_sql_string(),
            version_navi_rowid = CNAME_NAVI_ROWID,
            navi_rowid = navi_dao::CNAME_ROWID,
        );

        let mut stmt = self.sqlite_tx.prepare(&sql)?;

        // TODO APKのCDTも
        let column_data_types: Vec<ColumnDataType> = non_pk_column_data_types
            .iter()
            .map(|non_pk_cdt| non_pk_cdt.0.clone())
            .collect();

        let row_iter = stmt.query_named(
            &[(":navi_rowids", &navi_rowids)],
            &column_data_types
                .iter()
                .map(|cdt| cdt)
                .collect::<Vec<&ColumnDataType>>(),
        )?;
        Ok(row_iter)
    }

    pub(in crate::sqlite::transaction::sqlite_tx) fn insert(
        &self,
        version_id: &VersionId,
        navi_rowid: SqliteRowid,
        column_values: &HashMap<NonPKColumnName, Expression>,
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
            navi_rowid_val = navi_rowid.0,
            comma_if_non_pk_column_names = if column_values.is_empty() { "" } else { ", " },
            non_pk_column_names = column_values
                .keys()
                .map(|cn| cn.0.as_str())
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
