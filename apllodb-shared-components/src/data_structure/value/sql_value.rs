mod sql_convertible;

pub use sql_convertible::SqlConvertible;

use crate::{data_structure::DataTypeKind, error::ApllodbResult};
use serde::{Deserialize, Serialize};

/// SQL-typed value that is efficiently compressed.
///
/// A storage engine may (or may not) save `SqlValue`'s serialized instance as-is.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct SqlValue {
    ty: DataTypeKind,
    raw: Vec<u8>,
}

impl SqlValue {
    pub fn pack<T>(into_type: &DataTypeKind, rust_value: &T) -> ApllodbResult<Self>
    where
        T: SqlConvertible,
    {
        let raw = T::pack(into_type, rust_value)?;
        Ok(Self {
            ty: into_type.clone(),
            raw,
        })
    }

    pub fn unpack<T>(&self) -> ApllodbResult<T>
    where
        T: SqlConvertible,
    {
        T::unpack(&self.ty, &self.raw)
    }
}

#[cfg(test)]
mod tests_i32 {
    use super::SqlValue;
    use crate::{data_structure::DataTypeKind, error::ApllodbResult};

    #[test]
    fn test_pack_unpack() -> ApllodbResult<()> {
        let testset = vec![0, 1, -1, i32::MAX, i32::MIN];

        for v in testset {
            let sql_value = SqlValue::pack(&DataTypeKind::Integer, &v)?;
            let unpacked: i32 = sql_value.unpack()?;
            assert_eq!(unpacked, v);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests_option {
    use super::SqlValue;
    use crate::{
        data_structure::DataTypeKind,
        error::{ApllodbErrorKind, ApllodbResult},
    };

    #[test]
    fn test_pack_unpack() -> ApllodbResult<()> {
        let testset = vec![None, Some(-1)];

        for v in testset {
            let sql_value = SqlValue::pack(&DataTypeKind::Integer, &v)?;
            let unpacked: Option<i32> = sql_value.unpack()?;
            assert_eq!(unpacked, v);
        }
        Ok(())
    }

    #[test]
    fn test_failure_data_type_mismatch() -> ApllodbResult<()> {
        let testset = vec![-1];

        for v in testset {
            let sql_value = SqlValue::pack(&DataTypeKind::Integer, &v)?;
            match sql_value.unpack::<Option<i32>>() {
                Err(e) => assert_eq!(*e.kind(), ApllodbErrorKind::DatatypeMismatch),
                Ok(_) => panic!("should be DataTypeMismatch error"),
            }
        }
        Ok(())
    }
}
