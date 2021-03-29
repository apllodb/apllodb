use crate::{ApllodbResult, NnSqlValue};

use super::SqlConvertible;

impl SqlConvertible for bool {
    fn into_sql_value(self) -> NnSqlValue {
        NnSqlValue::Boolean(self)
    }

    fn try_from_bool(v: &bool) -> ApllodbResult<Self> {
        Ok(*v)
    }
}
