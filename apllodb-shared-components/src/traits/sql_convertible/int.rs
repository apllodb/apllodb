use crate::SqlType;

use super::SqlConvertible;
use std::collections::HashSet;

impl SqlConvertible for i16 {
    fn to_sql_types() -> HashSet<SqlType> {
        vec![SqlType::small_int(), SqlType::integer(), SqlType::big_int()]
            .into_iter()
            .collect()
    }

    fn from_sql_types() -> HashSet<SqlType> {
        vec![SqlType::small_int()].into_iter().collect()
    }
}

impl SqlConvertible for i32 {
    fn to_sql_types() -> HashSet<SqlType> {
        vec![SqlType::integer(), SqlType::big_int()]
            .into_iter()
            .collect()
    }

    fn from_sql_types() -> HashSet<SqlType> {
        vec![SqlType::small_int(), SqlType::integer()]
            .into_iter()
            .collect()
    }
}

impl SqlConvertible for i64 {
    fn to_sql_types() -> HashSet<SqlType> {
        vec![SqlType::big_int()].into_iter().collect()
    }

    fn from_sql_types() -> HashSet<SqlType> {
        vec![SqlType::small_int(), SqlType::integer(), SqlType::big_int()]
            .into_iter()
            .collect()
    }
}

#[cfg(test)]
mod tests_i32 {
    use crate::{
        data_structure::value::sql_value::SqlValue,
        error::{kind::ApllodbErrorKind, ApllodbResult},
        test_support::setup,
        NNSqlValue, SqlType,
    };

    #[test]
    fn test_pack_unpack() -> ApllodbResult<()> {
        setup();

        let rust_values = vec![0, 1, -1, i32::MAX, i32::MIN];

        for v in rust_values {
            let sql_value = NNSqlValue::pack(SqlType::integer(), &v)?;
            let unpacked: i32 = sql_value.unpack()?;
            assert_eq!(unpacked, v);
        }
        Ok(())
    }

    #[test]
    fn test_pack_failure_data_type_mismatch_length() {
        setup();

        let v: i32 = 1;
        match SqlValue::pack(SqlType::small_int(), &v) {
            Err(e) => assert_eq!(*e.kind(), ApllodbErrorKind::DatatypeMismatch),
            _ => unreachable!(),
        }
    }
}
