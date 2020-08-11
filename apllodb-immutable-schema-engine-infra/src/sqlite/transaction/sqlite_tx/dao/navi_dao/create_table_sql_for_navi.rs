use serde::{Deserialize, Serialize};
use apllodb_immutable_schema_engine_domain::vtable::VTable;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize)]
pub(super) struct CreateTableSqlForNavi(String);

impl CreateTableSqlForNavi {
    pub(super) fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&VTable> for CreateTableSqlForNavi {
    fn from(vtable: &VTable) -> Self {
        use crate::sqlite::to_sql_string::ToSqlString;

        // TODO Set primary key for performance.

        let sql = format!(
            "
CREATE TABLE {}__{} (
    {},
    {} INTEGER NOT NULL,
    {} INTEGER
)
        ",
            vtable.table_name(),
            super::TNAME_SUFFIX,
            vtable
                .table_wide_constraints()
                .pk_column_data_types()
                .iter()
                .map(|cdt| cdt.to_sql_string())
                .collect::<Vec<String>>()
                .join(",\n  "),
            super::CNAME_REVISION,
            super::CNAME_VERSION_NUMBER
        );

        Self(sql)
    }
}
