use crate::SqlType;

use super::SqlConvertible;
use std::collections::HashSet;

impl SqlConvertible for String {
    fn to_sql_types() -> HashSet<SqlType> {
        vec![SqlType::text()].into_iter().collect()
    }

    fn from_sql_types() -> HashSet<SqlType> {
        vec![SqlType::text()].into_iter().collect()
    }
}
