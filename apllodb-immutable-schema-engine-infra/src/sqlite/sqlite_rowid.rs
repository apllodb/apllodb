use apllodb_immutable_schema_engine_domain::version_revision_resolver::vrr_id::VRRId;

use apllodb_shared_components::{ApllodbResult, NNSqlValue, SqlConvertible};
use serde::{Deserialize, Serialize};

/// See: https://www.sqlite.org/lang_createtable.html#rowid
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct SqliteRowid(pub i64);

impl VRRId for SqliteRowid {}

impl SqlConvertible for SqliteRowid {
    fn into_sql_value(self) -> NNSqlValue {
        NNSqlValue::BigInt(self.0)
    }

    fn try_from_i16(v: &i16) -> ApllodbResult<Self> {
        Ok(Self(*v as i64))
    }

    fn try_from_i32(v: &i32) -> ApllodbResult<Self> {
        Ok(Self(*v as i64))
    }

    fn try_from_i64(v: &i64) -> ApllodbResult<Self> {
        Ok(Self(*v))
    }
}
