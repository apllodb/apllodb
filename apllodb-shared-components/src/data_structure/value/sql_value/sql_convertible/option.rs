use super::SqlConvertible;
use crate::data_structure::DataType;
use std::collections::HashSet;

impl<T: SqlConvertible> SqlConvertible for Option<T> {
    fn to_sql_types() -> HashSet<DataType> {
        T::to_sql_types()
            .iter()
            .map(|dt| DataType::new(dt.kind().clone(), true))
            .collect()
    }

    fn from_sql_types() -> HashSet<DataType> {
        T::from_sql_types()
            .iter()
            .map(|dt| DataType::new(dt.kind().clone(), true))
            .collect()
    }
}

#[cfg(test)]
mod tests_option {
    use crate::{
        data_structure::{DataType, DataTypeKind, SqlValue},
        error::{ApllodbErrorKind, ApllodbResult},
    };

    #[test]
    fn test_pack_unpack_some() -> ApllodbResult<()> {
        let testset = vec![Some(0), Some(1), Some(-1), Some(i32::MAX), Some(i32::MIN)];

        for v in testset {
            let sql_value = SqlValue::pack(&DataType::new(DataTypeKind::Integer, true), &v)?;
            let unpacked: Option<i32> = sql_value.unpack()?;
            assert_eq!(unpacked, v);
        }
        Ok(())
    }

    #[test]
    fn test_pack_unpack_none() -> ApllodbResult<()> {
        let sql_value = SqlValue::pack(&DataType::new(DataTypeKind::Integer, true), &None::<i32>)?;
        let unpacked: Option<i32> = sql_value.unpack()?;
        assert_eq!(unpacked, None);
        Ok(())
    }

    #[test]
    fn test_pack_failure_data_type_mismatch_nullability() {
        let v = 1i32;

        match SqlValue::pack(&DataType::new(DataTypeKind::Integer, true), &v) {
            Err(e) => assert_eq!(*e.kind(), ApllodbErrorKind::DatatypeMismatch),
            _ => unreachable!(),
        }
    }
}
