use crate::{
    data_structure::DataTypeKind,
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use serde::Serialize;

pub trait SqlConvertible: Serialize + Sized {
    fn pack(rust_value: &Self) -> ApllodbResult<Vec<u8>>;

    fn unpack(raw: &[u8]) -> ApllodbResult<Self>;

    fn is_acceptable(from_type: &DataTypeKind) -> bool;
}

/// i32 <- INT
impl SqlConvertible for i32 {
    fn pack(rust_value: &Self) -> ApllodbResult<Vec<u8>> {
        let v = bincode::serialize(&rust_value).map_err(|e| {
            ApllodbError::new(
                ApllodbErrorKind::SerializationError,
                format!("failed to serialize data as u32"),
                Some(Box::new(e)),
            )
        })?;
        Ok(v)
    }

    fn unpack(raw: &[u8]) -> ApllodbResult<Self> {
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
