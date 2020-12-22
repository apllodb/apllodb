mod int;
mod option;
mod text;

use crate::{
    data_structure::{DataType, DataTypeKind},
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use serde::{de::DeserializeOwned, Serialize};
use std::{any::type_name, collections::HashSet};

pub trait SqlConvertible: Serialize + DeserializeOwned + std::fmt::Debug {
    fn pack(into_type: &DataType, rust_value: &Self) -> ApllodbResult<Vec<u8>> {
        if Self::to_sql_types().contains(into_type) {
            let raw = bincode::serialize(&rust_value).map_err(|e| {
                ApllodbError::new(
                    ApllodbErrorKind::SerializationError,
                    format!("failed to pack Rust value: {:?}", rust_value),
                    Some(Box::new(e)),
                )
            })?;
            Ok(raw)
        } else {
            Err(ApllodbError::new(
                ApllodbErrorKind::DatatypeMismatch,
                format!(
                    "cannot convert Rust value `{:?}: {}` into SQL type `{:?}`",
                    rust_value,
                    type_name::<Self>(),
                    into_type
                ),
                None,
            ))
        }
    }

    fn unpack(from_type: &DataType, raw: &[u8]) -> ApllodbResult<Self> {
        if Self::from_sql_types().contains(from_type) {
            let v = bincode::deserialize(raw).map_err(|e| {
                ApllodbError::new(
                    ApllodbErrorKind::DeserializationError,
                    format!("failed to unpack data as {}", type_name::<Self>()),
                    Some(Box::new(e)),
                )
            })?;
            Ok(v)
        } else {
            Err(ApllodbError::new(
                ApllodbErrorKind::DatatypeMismatch,
                format!(
                    "cannot convert data from SQL type `{:?}` into Rust type `{}`",
                    from_type,
                    type_name::<Self>()
                ),
                None,
            ))
        }
    }

    /// SQL types which can hold all values of Self.
    fn to_sql_types() -> HashSet<DataType>;

    /// SQL types all of whose values can be held by Self.
    fn from_sql_types() -> HashSet<DataType>;
}

pub fn not_null_sql_types(kinds: &[DataTypeKind]) -> HashSet<DataType> {
    kinds
        .iter()
        .map(|kind| DataType::new(kind.clone(), false))
        .collect()
}
