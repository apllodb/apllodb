use super::{not_null_sql_types, SqlConvertible};
use crate::data_structure::{DataType, DataTypeKind};
use std::collections::HashSet;

impl SqlConvertible for i16 {
    fn to_sql_types() -> HashSet<DataType> {
        use DataTypeKind::*;
        not_null_sql_types(&[SmallInt, Integer, BigInt])
    }

    fn from_sql_types() -> HashSet<DataType> {
        use DataTypeKind::*;
        not_null_sql_types(&[SmallInt])
    }
}

impl SqlConvertible for i32 {
    fn to_sql_types() -> HashSet<DataType> {
        use DataTypeKind::*;
        not_null_sql_types(&[Integer, BigInt])
    }

    fn from_sql_types() -> HashSet<DataType> {
        use DataTypeKind::*;
        not_null_sql_types(&[SmallInt, Integer])
    }
}

impl SqlConvertible for i64 {
    fn to_sql_types() -> HashSet<DataType> {
        use DataTypeKind::*;
        not_null_sql_types(&[BigInt])
    }

    fn from_sql_types() -> HashSet<DataType> {
        use DataTypeKind::*;
        not_null_sql_types(&[SmallInt, Integer, BigInt])
    }
}

#[cfg(test)]
mod tests_i32 {
    use crate::{
        data_structure::{DataType, DataTypeKind, SqlValue},
        error::{ApllodbErrorKind, ApllodbResult},
    };

    #[test]
    fn test_pack_unpack() -> ApllodbResult<()> {
        let rust_values = vec![0, 1, -1, i32::MAX, i32::MIN];

        for v in rust_values {
            let sql_value = SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &v)?;
            let unpacked: i32 = sql_value.unpack()?;
            assert_eq!(unpacked, v);
        }
        Ok(())
    }

    #[test]
    fn test_pack_failure_data_type_mismatch_length() {
        let v: i32 = 1;
        match SqlValue::pack(&DataType::new(DataTypeKind::SmallInt, false), &v) {
            Err(e) => assert_eq!(*e.kind(), ApllodbErrorKind::DatatypeMismatch),
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_pack_failure_data_type_mismatch_nullability() {
        let v: i32 = 1;
        match SqlValue::pack(&DataType::new(DataTypeKind::Integer, true), &v) {
            Err(e) => assert_eq!(*e.kind(), ApllodbErrorKind::DatatypeMismatch),
            _ => unreachable!(),
        }
    }
}
