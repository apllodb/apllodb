use crate::{data_structure::DataType, error::ApllodbResult, traits::SqlConvertible};
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
    /// Convert rust_value into `SqlValue`
    ///
    /// # Failures
    ///
    /// - Errors from `T::pack()`.
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
