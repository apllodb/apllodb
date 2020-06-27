mod create_table_sql_for_navi;

use crate::sqlite::sqlite_error::map_sqlite_err;
use apllodb_immutable_schema_engine_domain::VTable;
use apllodb_shared_components::error::ApllodbResult;
use create_table_sql_for_navi::CreateTableSqlForNavi;

#[derive(Debug)]
pub(in crate::sqlite) struct NaviDao<'tx, 'db: 'tx> {
    sqlite_tx: &'tx rusqlite::Transaction<'db>,
}

const TNAME_SUFFIX: &str = "navi";
const CNAME_REVISION: &str = "revision";
const CNAME_VERSION_NUMBER: &str = "version_number";

impl<'tx, 'db: 'tx> NaviDao<'tx, 'db> {
    pub(in crate::sqlite::transaction::sqlite_tx) fn new(
        sqlite_tx: &'tx rusqlite::Transaction<'db>,
    ) -> Self {
        Self { sqlite_tx }
    }

    pub(in crate::sqlite::transaction::sqlite_tx) fn create_table(
        &self,
        vtable: &VTable,
    ) -> ApllodbResult<()> {
        let sql = CreateTableSqlForNavi::from(vtable);

        self.sqlite_tx
            .execute(sql.as_str(), rusqlite::params![])
            .map(|_| ())
            .map_err(|e| {
                map_sqlite_err(
                    e,
                    format!(
                        "backend sqlite3 raised an error on creating navi table for `{}`",
                        vtable.table_name()
                    ),
                )
            })?;
        Ok(())
    }
}
