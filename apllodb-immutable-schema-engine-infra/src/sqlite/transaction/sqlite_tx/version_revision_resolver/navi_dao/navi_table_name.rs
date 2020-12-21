use apllodb_shared_components::data_structure::TableName;
use serde::{Deserialize, Serialize};

use crate::sqlite::to_sql_string::ToSqlString;

const TNAME_SUFFIX: &str = "navi";

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub(in crate::sqlite::transaction::sqlite_tx::version_revision_resolver) struct NaviTableName(
    TableName,
);

impl NaviTableName {
    pub(in crate::sqlite::transaction::sqlite_tx::version_revision_resolver) fn to_table_name(&self) -> TableName {
        // better to return TableName("{self.0}__navi") but it might be invalid as ShortName.
        self.0.clone()
    }
}

impl From<TableName> for NaviTableName {
    fn from(table_name: TableName) -> Self {
        Self(table_name)
    }
}

impl ToSqlString for NaviTableName {
    fn to_sql_string(&self) -> String {
        format!("{}__{}", self.0, TNAME_SUFFIX)
    }
}
