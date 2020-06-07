mod sql_convertible;

pub use sql_convertible::SqlConvertible;

use crate::{data_structure::DataType, error::ApllodbResult};
use serde::{Deserialize, Serialize};

/// SQL-typed value that is efficiently compressed.
///
/// A storage engine may (or may not) save `SqlValue`'s serialized instance as-is.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct SqlValue {
    data_type: DataType,
    raw: Vec<u8>,
}

impl SqlValue {
    pub fn pack<T>(into_type: &DataType, rust_value: &T) -> ApllodbResult<Self>
    where
        T: SqlConvertible,
    {
        let raw = T::pack(into_type, rust_value)?;
        Ok(Self {
            data_type: into_type.clone(),
            raw,
        })
    }

    pub fn unpack<T>(&self) -> ApllodbResult<T>
    where
        T: SqlConvertible,
    {
        T::unpack(&self.data_type, &self.raw)
    }
}

#[cfg(test)]
mod tests_i32 {
    use super::SqlValue;
    use crate::{
        data_structure::{DataType, DataTypeKind},
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

#[cfg(test)]
mod tests_option {
    use super::SqlValue;
    use crate::{
        data_structure::{DataType, DataTypeKind},
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
