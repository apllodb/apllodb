use std::collections::HashSet;

use apllodb_immutable_schema_engine_domain::version_revision_resolver::vrr_id::VRRId;

use apllodb_shared_components::{
    data_structure::{DataType, DataTypeKind},
    traits::{not_null_sql_types, SqlConvertible},
};
use serde::{Deserialize, Serialize};

/// See: https://www.sqlite.org/lang_createtable.html#rowid
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct SqliteRowid(pub i64);

impl VRRId for SqliteRowid {}

impl SqlConvertible for SqliteRowid {
    fn to_sql_types() -> HashSet<DataType> {
        use DataTypeKind::*;
        not_null_sql_types(&[BigInt])
    }

    fn from_sql_types() -> HashSet<DataType> {
        use DataTypeKind::*;
        not_null_sql_types(&[SmallInt, Integer, BigInt])
    }
}
