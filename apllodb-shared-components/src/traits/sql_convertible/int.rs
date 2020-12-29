use crate::data_structure::data_type::{data_type_kind::DataTypeKind, DataType};

use super::{not_null_data_types, SqlConvertible};
use std::collections::HashSet;

impl SqlConvertible for i16 {
    fn to_sql_types() -> HashSet<DataType> {
        not_null_data_types(&[
            DataTypeKind::SmallInt,
            DataTypeKind::Integer,
            DataTypeKind::BigInt,
        ])
    }

    fn from_sql_types() -> HashSet<DataType> {
        not_null_data_types(&[DataTypeKind::SmallInt])
    }
}

impl SqlConvertible for i32 {
    fn to_sql_types() -> HashSet<DataType> {
        not_null_data_types(&[DataTypeKind::Integer, DataTypeKind::BigInt])
    }

    fn from_sql_types() -> HashSet<DataType> {
        not_null_data_types(&[DataTypeKind::SmallInt, DataTypeKind::Integer])
    }
}

impl SqlConvertible for i64 {
    fn to_sql_types() -> HashSet<DataType> {
        not_null_data_types(&[DataTypeKind::BigInt])
    }

    fn from_sql_types() -> HashSet<DataType> {
        not_null_data_types(&[
            DataTypeKind::SmallInt,
            DataTypeKind::Integer,
            DataTypeKind::BigInt,
        ])
    }
}

#[cfg(test)]
mod tests_i32 {
    use crate::{data_structure::{data_type::{DataType, data_type_kind::DataTypeKind}, value::sql_value::SqlValue}, error::{kind::ApllodbErrorKind, ApllodbResult}};

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
