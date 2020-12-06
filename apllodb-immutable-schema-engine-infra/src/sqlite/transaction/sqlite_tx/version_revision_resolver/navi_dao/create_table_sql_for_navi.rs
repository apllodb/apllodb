use super::NaviDao;
use apllodb_immutable_schema_engine_domain::vtable::VTable;
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
        use apllodb_immutable_schema_engine_domain::entity::Entity;

        // TODO Set primary key for performance.

        let sql = format!(
            "
CREATE TABLE {tname} (
    {pk_coldefs},
    {cname_revision} INTEGER NOT NULL,
    {cname_version_number} INTEGER
)
        ",
            tname = NaviDao::table_name(vtable.id()),
            pk_coldefs = vtable
                .table_wide_constraints()
                .pk_column_data_types()
                .to_sql_string(),
            cname_revision = super::CNAME_REVISION,
            cname_version_number = super::CNAME_VERSION_NUMBER
        );

        Self(sql)
    }
}
