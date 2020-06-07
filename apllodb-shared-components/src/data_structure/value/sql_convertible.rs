use crate::{
    data_structure::DataTypeKind,
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use serde::Serialize;

pub trait SqlConvertible: Serialize + Sized {
    fn pack(into_type: &DataTypeKind, rust_value: &Self) -> ApllodbResult<Vec<u8>> {
        if Self::is_acceptable(&into_type) {
            Self::pack_aux(rust_value)
        } else {
            Err(ApllodbError::new(
                ApllodbErrorKind::DatatypeMismatch,
                format!("cannot convert data into SQL `{:?}`", into_type),
                None,
            ))
        }
    }

    fn unpack(from_type: &DataTypeKind, raw: &[u8]) -> ApllodbResult<Self> {
        if Self::is_acceptable(&from_type) {
            Self::unpack_aux(raw)
        } else {
            Err(ApllodbError::new(
                ApllodbErrorKind::DatatypeMismatch,
                format!("cannot convert data from SQL `{:?}`", from_type),
                None,
            ))
        }
    }

    #[doc(hidden)]
    fn pack_aux(rust_value: &Self) -> ApllodbResult<Vec<u8>>;

    #[doc(hidden)]
    fn unpack_aux(raw: &[u8]) -> ApllodbResult<Self>;

    #[doc(hidden)]
    fn is_acceptable(from_type: &DataTypeKind) -> bool;
}

/// i32 <- INT
impl SqlConvertible for i32 {
    fn pack_aux(rust_value: &Self) -> ApllodbResult<Vec<u8>> {
        let v = bincode::serialize(&rust_value).map_err(|e| {
            ApllodbError::new(
                ApllodbErrorKind::SerializationError,
                format!("failed to serialize data as u32"),
                Some(Box::new(e)),
            )
        })?;
        Ok(v)
    }

    fn unpack_aux(raw: &[u8]) -> ApllodbResult<Self> {
        let v = bincode::deserialize(raw).map_err(|e| {
            ApllodbError::new(
                ApllodbErrorKind::DeserializationError,
                format!("failed to deserialize data as u32"),
                Some(Box::new(e)),
            )
        })?;
        Ok(v)
    }

    fn is_acceptable(ty: &DataTypeKind) -> bool {
        match ty {
            DataTypeKind::SmallInt | DataTypeKind::Integer => true,
            DataTypeKind::BigInt => false,
        }
    }
}

#[cfg(test)]
mod tests_i32 {
    use super::SqlConvertible;
    use crate::{data_structure::DataTypeKind, error::ApllodbResult};

    #[test]
    fn test_pack_unpack() -> ApllodbResult<()> {
        let testset = vec![0, 1, -1, i32::MAX, i32::MIN];

        for v in testset {
            let packed: Vec<u8> = SqlConvertible::pack(&DataTypeKind::Integer, &v)?;
            let unpacked: i32 = SqlConvertible::unpack(&DataTypeKind::Integer, &packed)?;
            assert_eq!(unpacked, v);
        }
        Ok(())
    }
}
