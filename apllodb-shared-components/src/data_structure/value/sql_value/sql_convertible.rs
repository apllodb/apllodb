use crate::{
    data_structure::DataTypeKind,
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use serde::{de::DeserializeOwned, Serialize};
use std::{any::type_name, collections::HashSet};

pub trait SqlConvertible: Serialize + DeserializeOwned + std::fmt::Debug {
    fn pack(rust_value: &Self) -> ApllodbResult<Vec<u8>> {
        let v = bincode::serialize(&rust_value).map_err(|e| {
            ApllodbError::new(
                ApllodbErrorKind::SerializationError,
                format!("failed to pack Rust value: {:?}", rust_value),
                Some(Box::new(e)),
            )
        })?;
        Ok(v)
    }

    fn unpack(raw: &[u8]) -> ApllodbResult<Self> {
        let v = bincode::deserialize(raw).map_err(|e| {
            ApllodbError::new(
                ApllodbErrorKind::DeserializationError,
                format!("failed to unpack data as {}", type_name::<Self>()),
                Some(Box::new(e)),
            )
        })?;
        Ok(v)
    }

    fn to_sql_types() -> HashSet<DataTypeKind>;

    fn from_sql_types() -> HashSet<DataTypeKind>;
}

impl SqlConvertible for i32 {
    fn to_sql_types() -> HashSet<DataTypeKind> {
        use DataTypeKind::*;
        [Integer, BigInt].iter().cloned().collect()
    }

    fn from_sql_types() -> HashSet<DataTypeKind> {
        use DataTypeKind::*;
        [SmallInt, Integer].iter().cloned().collect()
    }
}

impl<T: SqlConvertible> SqlConvertible for Option<T> {
    fn to_sql_types() -> HashSet<DataTypeKind> {
        T::to_sql_types()
    }

    fn from_sql_types() -> HashSet<DataTypeKind> {
        T::from_sql_types()
    }
}
