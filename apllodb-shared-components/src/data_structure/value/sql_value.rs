mod sql_convertible;

pub use sql_convertible::SqlConvertible;

use crate::{
    data_structure::DataTypeKind,
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use serde::{Deserialize, Serialize};
use std::any::type_name;

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
        if T::to_sql_types().contains(into_type) {
            let raw = T::pack(rust_value)?;
            Ok(Self {
                ty: into_type.clone(),
                raw,
            })
        } else {
            Err(ApllodbError::new(
                ApllodbErrorKind::DatatypeMismatch,
                format!(
                    "cannot convert Rust value `{:?}` into SQL type `{:?}`",
                    rust_value, into_type
                ),
                None,
            ))
        }
    }

    pub fn unpack<T>(&self) -> ApllodbResult<T>
    where
        T: SqlConvertible,
    {
        if T::from_sql_types().contains(&self.ty) {
            T::unpack(&self.raw)
        } else {
            Err(ApllodbError::new(
                ApllodbErrorKind::DatatypeMismatch,
                format!(
                    "cannot convert data from SQL type `{:?}` into Rust type `{}`",
                    &self.ty,
                    type_name::<T>()
                ),
                None,
            ))
        }
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
