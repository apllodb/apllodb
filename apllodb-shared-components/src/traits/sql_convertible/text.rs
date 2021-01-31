use crate::{ApllodbResult, NNSqlValue};

use super::SqlConvertible;

impl SqlConvertible for String {
    fn into_sql_value(self) -> NNSqlValue {
        NNSqlValue::Text(self)
    }

    fn try_from_string(v: &str) -> ApllodbResult<Self> {
        Ok(v.to_string())
    }
}
