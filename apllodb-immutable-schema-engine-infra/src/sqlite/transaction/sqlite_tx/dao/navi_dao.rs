mod create_table_sql_for_navi;
mod navi;

pub(in crate::sqlite) use navi::Navi;

use crate::sqlite::{sqlite_rowid::SqliteRowid, SqliteTx};
use apllodb_immutable_schema_engine_domain::{
    ApparentPrimaryKey, Revision, VTable, VTableId, VersionId, VersionNumber,
};
use apllodb_shared_components::{
    data_structure::{ColumnDataType, ColumnName, DataType, DataTypeKind},
    error::ApllodbResult,
};
use create_table_sql_for_navi::CreateTableSqlForNavi;

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
        use apllodb_storage_engine_interface::{PrimaryKey, Row};

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

        let cdt_rowid = self.cdt_rowid();
        let mut column_data_types = vec![&cdt_rowid];
        let apk_column_data_types = apk.column_data_types();
        column_data_types.append(
            &mut apk_column_data_types
                .iter()
                .map(|acdt| acdt)
                .collect::<Vec<&ColumnDataType>>(),
        );

        let mut row_iter = stmt.query_named(&[], &column_data_types)?;
        let opt_row = row_iter.next();

        let navi = match opt_row {
            None => Navi::NotExist,
            Some(r) => {
                let r = r?;
                let rowid = SqliteRowid(r.get::<i64>(&ColumnName::new(CNAME_ROWID)?)?);
                let revision = Revision::from(r.get::<i64>(&ColumnName::new(CNAME_REVISION)?)?);
                let opt_version_number = r
                    .get::<Option<i64>>(&ColumnName::new(CNAME_VERSION_NUMBER)?)?
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

        let _ = self.sqlite_tx.execute_named(
            &sql,
            &[
                (":revision", &revision),
                (":version_number", version_id.version_number()),
            ],
        )?;

        Ok(self.sqlite_tx.last_insert_rowid())
    }

    fn cdt_rowid(&self) -> ColumnDataType {
        ColumnDataType::new(
            ColumnName::new(CNAME_ROWID).unwrap(),
            DataType::new(DataTypeKind::BigInt, false),
        )
    }
}
