use super::SqlConvertible;
use crate::{
    data_structure::DataTypeKind,
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};

/// SQL-typed value that is efficiently compressed.
///
/// A storage engine may (or may not) save `SqlValue`'s serialized instance as-is.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct SqlValue {
    ty: DataTypeKind,
    raw: Vec<u8>,
}

impl SqlValue {
    pub fn pack<T>(into_type: &DataTypeKind, rust_value: &T) -> ApllodbResult<Self>
    where
        T: SqlConvertible,
    {
        if T::is_acceptable(&into_type) {
            let raw = T::pack(rust_value)?;
            Ok(Self {
                ty: into_type.clone(),
                raw,
            })
        } else {
            Err(ApllodbError::new(
                ApllodbErrorKind::DatatypeMismatch,
                format!("cannot convert data into SQL `{:?}`", into_type),
                None,
            ))
        }
    }

    pub fn unpack<T>(&self) -> ApllodbResult<T>
    where
        T: SqlConvertible,
    {
        if T::is_acceptable(&self.ty) {
            T::unpack(&self.raw)
        } else {
            Err(ApllodbError::new(
                ApllodbErrorKind::DatatypeMismatch,
                format!("cannot convert data from SQL `{:?}`", &self.ty),
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
