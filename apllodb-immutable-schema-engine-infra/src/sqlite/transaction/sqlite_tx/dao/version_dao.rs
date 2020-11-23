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
use apllodb_shared_components::{
    data_structure::ColumnName,
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
        vrr_entries_in_version: VRREntriesInVersion<
            'dao,
            'db,
            ApllodbImmutableSchemaEngine,
            SqliteTypes,
        >,
        projection: &[ColumnName],
    ) -> ApllodbResult<SqliteRowIterator> {
        todo!()
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
