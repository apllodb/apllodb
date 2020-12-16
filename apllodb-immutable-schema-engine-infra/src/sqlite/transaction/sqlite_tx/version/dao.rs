pub(in crate::sqlite::transaction::sqlite_tx) mod create_table_sql_for_version;
pub(in crate::sqlite::transaction::sqlite_tx) mod sqlite_table_name_for_version;

use crate::sqlite::{
    row_iterator::SqliteRowIterator, sqlite_rowid::SqliteRowid, sqlite_types::VRREntriesInVersion,
    to_sql_string::ToSqlString, transaction::sqlite_tx::SqliteTx,
};
use apllodb_immutable_schema_engine_domain::{
    row::{immutable_row::ImmutableRow, pk::apparent_pk::ApparentPrimaryKey},
    version::{active_version::ActiveVersion, id::VersionId},
    vtable::VTable,
};
use apllodb_shared_components::{
    data_structure::ColumnDataType,
    data_structure::ColumnName,
    data_structure::ColumnReference,
    data_structure::{Expression, TableName},
    error::ApllodbResult,
};
use apllodb_storage_engine_interface::Row;
use create_table_sql_for_version::CreateTableSqlForVersion;
use std::collections::{hash_map::Entry, HashMap, VecDeque};

#[cfg(test)]
pub(in crate::sqlite::transaction::sqlite_tx) use create_table_sql_for_version::test_wrapper::CreateTableSqlForVersionTestWrapper;

use self::sqlite_table_name_for_version::SqliteTableNameForVersion;

#[derive(Debug)]
pub(in crate::sqlite) struct VersionDao<'dao, 'db: 'dao> {
    sqlite_tx: &'dao SqliteTx<'db>,
}

pub(in crate::sqlite::transaction::sqlite_tx) const CNAME_NAVI_ROWID: &str = "_navi_rowid";

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
        use apllodb_immutable_schema_engine_domain::entity::Entity;

        let column_data_types = version.column_data_types();
        // Filter existing and requested columns.
        let existing_projection: Vec<&ColumnDataType> = column_data_types
            .iter()
            .filter(|cdt| projection.contains(&cdt.column_ref().as_column_name()))
            .collect();

        if existing_projection.is_empty() {
            // PK-only ImmutableRow
            let pk_rows = vrr_entries_in_version
                .map(|e| e.into_pk_only_row())
                .collect::<ApllodbResult<VecDeque<ImmutableRow>>>()?;
            Ok(SqliteRowIterator::from(pk_rows))
        } else {
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

            let sql = format!(
                "
SELECT {version_navi_rowid}, {non_pk_column_names} FROM {version_table}
  WHERE {version_navi_rowid} IN (:navi_rowids)
", // FIXME prevent SQL injection
                non_pk_column_names = existing_projection.to_sql_string(),
                version_table = sqlite_table_name.to_sql_string(),
                version_navi_rowid = CNAME_NAVI_ROWID,
            );
            let mut stmt = self.sqlite_tx.prepare(&sql)?;

            let (navi_rowids, pks): (Vec<SqliteRowid>, Vec<ApparentPrimaryKey>) =
                vrr_entries_in_version
                    .map(|e| (e.id().clone(), e.into_pk()))
                    .unzip();

            let row_iter = stmt.query_named(
                &[(":navi_rowids", &navi_rowids)],
                &existing_projection,
                &void_projection,
            )?;

            let mut rowid_vs_row = HashMap::<SqliteRowid, ImmutableRow>::new();
            for mut row in row_iter {
                rowid_vs_row.insert(
                    row.get(&ColumnReference::new(
                        sqlite_table_name.clone(),
                        ColumnName::new(CNAME_NAVI_ROWID)?,
                    ))?,
                    row,
                );
            }

            let mut rows_with_pk = VecDeque::<ImmutableRow>::new();
            for (rowid, pk) in navi_rowids.into_iter().zip(pks) {
                if let Entry::Occupied(oe) = rowid_vs_row.entry(rowid.clone()) {
                    let (_, mut row_wo_pk) = oe.remove_entry();
                    row_wo_pk.append(pk.into_colvals())?;
                    rows_with_pk.push_back(row_wo_pk)
                } else {
                    panic!(
                        "navi_rowid={} is requested to table `{}` but it's not found",
                        rowid.to_sql_string(),
                        sqlite_table_name
                    );
                }
            }

            Ok(SqliteRowIterator::from(rows_with_pk))
        }
    }

    pub(in crate::sqlite::transaction::sqlite_tx) fn insert(
        &self,
        version_id: &VersionId,
        vrr_id: &SqliteRowid,
        column_values: &HashMap<ColumnName, Expression>,
    ) -> ApllodbResult<()> {
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

        self.sqlite_tx.execute_named(&sql, &[])?;

        Ok(())
    }
}
