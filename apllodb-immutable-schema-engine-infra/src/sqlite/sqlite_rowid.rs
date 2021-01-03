use std::collections::HashSet;

use apllodb_immutable_schema_engine_domain::version_revision_resolver::vrr_id::VRRId;

use apllodb_shared_components::{SqlConvertible, SqlType};
use serde::{Deserialize, Serialize};

/// See: https://www.sqlite.org/lang_createtable.html#rowid
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct SqliteRowid(pub i64);

impl VRRId for SqliteRowid {}

impl SqlConvertible for SqliteRowid {
    fn to_sql_types() -> HashSet<SqlType> {
        vec![SqlType::big_int()].into_iter().collect()
    }

    fn from_sql_types() -> HashSet<SqlType> {
        vec![SqlType::small_int(), SqlType::integer(), SqlType::big_int()]
            .into_iter()
            .collect()
    }
}
