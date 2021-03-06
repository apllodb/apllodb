use crate::{ApllodbResult, NNSqlValue};

use super::SqlConvertible;

impl SqlConvertible for bool {
    fn into_sql_value(self) -> NNSqlValue {
        NNSqlValue::Boolean(self)
    }

    fn try_from_bool(v: &bool) -> ApllodbResult<Self> {
        Ok(*v)
    }
}
