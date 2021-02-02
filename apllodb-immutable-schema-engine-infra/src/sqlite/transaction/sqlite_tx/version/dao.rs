pub(in crate::sqlite::transaction::sqlite_tx) mod create_table_sql_for_version;
pub(in crate::sqlite::transaction::sqlite_tx) mod sqlite_table_name_for_version;

use crate::sqlite::{
    row_iterator::SqliteRowIterator, sqlite_rowid::SqliteRowid, sqlite_types::VRREntriesInVersion,
    to_sql_string::ToSqlString, transaction::sqlite_tx::SqliteTx,
};
use apllodb_immutable_schema_engine_domain::{
    entity::Entity,
    query::projection::ProjectionResult,
    row::{immutable_row::ImmutableRow, pk::apparent_pk::ApparentPrimaryKey},
    version::{active_version::ActiveVersion, id::VersionId},
};
use apllodb_shared_components::{
    ApllodbResult, ColumnDataType, ColumnName, SqlType, SqlValue, TableName,
};
use apllodb_storage_engine_interface::TableColumnReference;
use create_table_sql_for_version::CreateTableSqlForVersion;
use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap, VecDeque},
    rc::Rc,
};

#[cfg(test)]
pub(in crate::sqlite::transaction::sqlite_tx) use create_table_sql_for_version::test_wrapper::CreateTableSqlForVersionTestWrapper;

use self::sqlite_table_name_for_version::SqliteTableNameForVersion;

#[derive(Debug)]
pub(in crate::sqlite) struct VersionDao {
    sqlite_tx: Rc<RefCell<SqliteTx>>,
}

pub(in crate::sqlite::transaction::sqlite_tx) const CNAME_NAVI_ROWID: &str = "_navi_rowid";

impl VersionDao {
    pub(in crate::sqlite::transaction::sqlite_tx) fn table_name(
        version_id: &VersionId,
        is_active: bool,
    ) -> TableName {
        SqliteTableNameForVersion::new(version_id, is_active).to_full_table_name()
    }
}

impl VersionDao {
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

    /// Fetches only existing columns from SQLite, and makes SqliteRowIterator together with ApparentPrimaryKey from VRREntriesInVersion.
    pub(in crate::sqlite::transaction::sqlite_tx) async fn probe_in_version(
        &self,
        version: &ActiveVersion,
        vrr_entries_in_version: VRREntriesInVersion,
        projection: &ProjectionResult,
    ) -> ApllodbResult<SqliteRowIterator> {
        if projection
            .non_pk_effective_projection(version.id())?
            .is_empty()
            && projection.non_pk_void_projection(version.id())?.is_empty()
        {
            // PK-only ImmutableRow
            let pk_rows = vrr_entries_in_version
                .map(|e| e.into_pk_only_row())
                .collect::<ApllodbResult<VecDeque<ImmutableRow>>>()?;
            Ok(SqliteRowIterator::from(pk_rows))
        } else {
            let sqlite_table_name = Self::table_name(version.id(), true);

            let non_pk_effective_projection =
                projection.non_pk_effective_projection(version.id())?;
            let non_pk_void_projection = projection.non_pk_void_projection(version.id())?;

            let (navi_rowids, pks): (Vec<SqliteRowid>, Vec<ApparentPrimaryKey>) =
                vrr_entries_in_version
                    .map(|e| (e.id().clone(), e.into_pk()))
                    .unzip();

            let sql = format!(
                "
SELECT {version_navi_rowid}{comma_if_non_pk_column}{non_pk_column_names}{comma_if_void_projection}{void_projection} FROM {version_table}
  WHERE {version_navi_rowid} IN ({navi_rowids})
", // FIXME prevent SQL injection
                comma_if_non_pk_column = if non_pk_effective_projection.is_empty() {
                    ""
                } else {
                    ", "
                },
                non_pk_column_names = non_pk_effective_projection
                    .to_sql_string(),
                comma_if_void_projection = if non_pk_void_projection.is_empty() {""} else {", "},
                void_projection = non_pk_void_projection
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
                .filter(|cdt| non_pk_effective_projection.contains(cdt.column_name()))
                .collect();
            let cdt_navi_rowid = self.cdt_navi_rowid();
            let mut prj_with_navi_rowid = vec![&cdt_navi_rowid];
            prj_with_navi_rowid.append(&mut effective_prj_cdts);

            let row_iter_from_version = self
                .sqlite_tx
                .borrow_mut()
                .query(
                    &sql,
                    version.vtable_id().table_name(),
                    &prj_with_navi_rowid,
                    non_pk_void_projection
                        .iter()
                        .map(|cn| {
                            TableColumnReference::new(
                                version.vtable_id().table_name().clone(),
                                cn.clone(),
                            )
                        })
                        .collect::<Vec<_>>()
                        .as_slice(),
                )
                .await?;

            let mut rowid_vs_row = HashMap::<SqliteRowid, ImmutableRow>::new();
            for mut row in row_iter_from_version {
                let rowid = row
                    .get(&TableColumnReference::new(
                        version.vtable_id().table_name().clone(),
                        ColumnName::new(CNAME_NAVI_ROWID)?,
                    ))?
                    .expect("must be NOT NULL");

                rowid_vs_row.insert(rowid, row);
            }

            let mut rows_with_pk = VecDeque::<ImmutableRow>::new();
            for (rowid, pk) in navi_rowids.into_iter().zip(pks) {
                if let Entry::Occupied(oe) = rowid_vs_row.entry(rowid.clone()) {
                    let (_, mut row_wo_pk) = oe.remove_entry();
                    for (column_name, nn_sql_value) in pk.into_zipped() {
                        let pk_tcr = TableColumnReference::new(
                            version.vtable_id().table_name().clone(),
                            column_name,
                        );
                        row_wo_pk.append(pk_tcr, SqlValue::NotNull(nn_sql_value))?;
                    }
                    rows_with_pk.push_back(row_wo_pk)
                } else {
                    panic!(
                        "navi_rowid={} is requested to table `{:?}` but it's not found",
                        rowid.to_sql_string(),
                        sqlite_table_name
                    );
                }
            }

            Ok(SqliteRowIterator::from(rows_with_pk))
        }
    }

    pub(in crate::sqlite::transaction::sqlite_tx) async fn insert(
        &self,
        version_id: &VersionId,
        vrr_id: &SqliteRowid,
        column_values: &HashMap<ColumnName, SqlValue>,
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

        self.sqlite_tx.borrow_mut().execute(&sql).await?;

        Ok(())
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
