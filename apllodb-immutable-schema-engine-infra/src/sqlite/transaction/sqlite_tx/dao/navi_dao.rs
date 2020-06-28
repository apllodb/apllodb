mod create_table_sql_for_navi;
mod navi;

pub(in crate::sqlite) use navi::Navi;

use crate::sqlite::{sqlite_error::map_sqlite_err, sqlite_rowid::SqliteRowid, SqliteTx};
use apllodb_immutable_schema_engine_domain::{
    ApparentPrimaryKey, Revision, VTable, VTableId, VersionId, VersionNumber,
};
use apllodb_shared_components::error::ApllodbResult;
use create_table_sql_for_navi::CreateTableSqlForNavi;
use log::error;

#[derive(Debug)]
pub(in crate::sqlite) struct NaviDao<'tx, 'db: 'tx> {
    sqlite_tx: &'tx SqliteTx<'db>,
}

const TNAME_SUFFIX: &str = "navi";
const CNAME_ROWID: &str = "_rowid_";
const CNAME_REVISION: &str = "revision";
const CNAME_VERSION_NUMBER: &str = "version_number";

impl<'tx, 'db: 'tx> NaviDao<'tx, 'db> {
    pub(in crate::sqlite::transaction::sqlite_tx) fn new(sqlite_tx: &'tx SqliteTx<'db>) -> Self {
        Self { sqlite_tx }
    }

    pub(in crate::sqlite::transaction::sqlite_tx) fn create_table(
        &self,
        vtable: &VTable,
    ) -> ApllodbResult<()> {
        let sql = CreateTableSqlForNavi::from(vtable);
        self.sqlite_tx.execute_named(sql.as_str(), &vec![])?;
        Ok(())
    }

    pub(in crate::sqlite::transaction::sqlite_tx) fn probe(
        &self,
        vtable_id: &VTableId,
        apk: &ApparentPrimaryKey,
    ) -> ApllodbResult<Navi> {
        use crate::sqlite::to_sql_string::ToSqlString;
        use apllodb_storage_engine_interface::PrimaryKey;

        let apk_column_names_sql = apk
            .column_names()
            .iter()
            .map(|cn| cn.as_str())
            .collect::<Vec<&str>>()
            .join(", ");

        let sql = format!(
            "
SELECT {}, {}
  FROM {}__{}
  WHERE 
    {}
  ORDER BY {} DESC
  LIMIT 1;
",
            CNAME_ROWID,
            apk_column_names_sql,
            vtable_id.table_name(),
            TNAME_SUFFIX,
            apk.to_condition_expression()?.to_sql_string(),
            CNAME_REVISION
        );

        let mut stmt = self.sqlite_tx.prepare(&sql)?;
        let mut row_iter = stmt.query_named(&[], &vec![], apk.column_data_types()).map_err(|e| {
            map_sqlite_err(
                e,
                format!(
                    "SQLite raised an error while selecting table `{}`'s navi table",
                    vtable_id.table_name()
                ),
            )
        })?;
        let opt_row = row_iter.next().map_err(|e| {
            map_sqlite_err(
                e,
                format!(
                    "SQLite raised an error while fetching row of table `{}`'s navi table",
                    vtable_id.table_name()
                ),
            )
        })?;

        let navi = match opt_row {
            None => Navi::NotExist,
            Some(r) => {
                let rowid = SqliteRowid(r.get::<_, i64>(CNAME_ROWID).map_err(|e| {
                    map_sqlite_err(
                        e,
                        format!(
                            "failed to get `{}.{}` from SQLite",
                            vtable_id.table_name(),
                            CNAME_ROWID
                        ),
                    )
                })?);
                let revision = Revision::from(r.get::<_, i64>(CNAME_REVISION).map_err(|e| {
                    map_sqlite_err(
                        e,
                        format!(
                            "failed to get `{}.{}` from SQLite",
                            vtable_id.table_name(),
                            CNAME_REVISION
                        ),
                    )
                })?);
                let opt_version_number = r
                    .get::<_, Option<i64>>(CNAME_VERSION_NUMBER)
                    .map_err(|e| {
                        map_sqlite_err(
                            e,
                            format!(
                                "failed to get `{}.{}` from SQLite",
                                vtable_id.table_name(),
                                CNAME_VERSION_NUMBER
                            ),
                        )
                    })?
                    .map(VersionNumber::from);
                match opt_version_number {
                    None => Navi::Deleted { rowid, revision },
                    Some(version_number) => Navi::Exist {
                        rowid,
                        revision,
                        version_number,
                    },
                }
            }
        };
        Ok(navi)
    }

    /// Returns lastly inserted row's ROWID.
    pub(in crate::sqlite::transaction::sqlite_tx) fn insert(
        &self,
        apk: ApparentPrimaryKey,
        revision: Revision,
        version_id: &VersionId,
    ) -> ApllodbResult<SqliteRowid> {
        use crate::sqlite::to_sql_string::ToSqlString;
        use apllodb_storage_engine_interface::PrimaryKey;

        let vtable_id = version_id.vtable_id();

        let sql = format!(
            "INSERT INTO {}__{} ({}, {}, {}) VALUES ({}, :revision, :version_number);",
            vtable_id.table_name(),
            TNAME_SUFFIX,
            apk.column_names()
                .iter()
                .map(|cn| cn.as_str())
                .collect::<Vec<&str>>()
                .join(", "),
            CNAME_REVISION,
            CNAME_VERSION_NUMBER,
            apk.sql_values()
                .iter()
                .map(|sql_value| sql_value.to_sql_string())
                .collect::<Vec<String>>()
                .join(", "),
        );

        let _ = self
            .sqlite_tx
            .execute_named(
                &sql,
                &[
                    (":revision", &revision.to_sql_string()),
                    (
                        ":version_number",
                        &version_id.version_number().to_sql_string(),
                    ),
                ],
            )
            .map_err(|e| {
                error!("unexpected SQLite error: {:?}", e);
                map_sqlite_err(
                    e,
                    format!(
                        "SQLite raised an error inserting into table navi table of `{}`",
                        vtable_id.table_name()
                    ),
                )
            })?;

        let rowid = self.sqlite_tx.last_insert_rowid();
        Ok(SqliteRowid(rowid))
    }
}
