use apllodb_immutable_schema_engine_domain::VTable;
use serde::{Deserialize, Serialize};

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

        let sql = format!(
            "
CREATE TABLE {}__navi (
    {},
    revision INTEGER NOT NULL,
    version_number INTEGER NOT NULL
)
        ",
            vtable.table_name(),
            vtable
                .table_wide_constraints()
                .apparent_pk_column_data_types()
                .iter()
                .map(|cdt| cdt.to_sql_string())
                .collect::<Vec<String>>()
                .join(",\n  ")
        );

        Self(sql)
    }
}
