mod int;
mod text;

use crate::{
    error::{kind::ApllodbErrorKind, ApllodbError, ApllodbResult},
    SqlType,
};
use serde::{de::DeserializeOwned, Serialize};
use std::{any::type_name, collections::HashSet, hash::Hash};

/// Rust values which can be serialized into / deserialized from binary and can have bidirectional mapping to/from SQL [SqlValue](crate::SqlValue).
pub trait SqlConvertible: Serialize + DeserializeOwned + std::fmt::Debug + Hash {
    /// Serialize into binary.
    /// Default implementation should be fast enough.
    ///
    /// # Failures
    ///
    /// - [SerializationError](crate::ApllodbErrorKind::SerializationError) when:
    ///   - failed in serialization.
    fn pack(rust_value: &Self) -> ApllodbResult<Vec<u8>> {
        bincode::serialize(&rust_value).map_err(|e| {
            ApllodbError::new(
                ApllodbErrorKind::SerializationError,
                format!("failed to pack Rust value: {:?}", rust_value),
                Some(Box::new(e)),
            )
        })
    }

    /// Deserialize from binary.
    /// Default implementation should be fast enough.
    ///
    /// # Failures
    ///
    /// - [DeserializationError](crate::ApllodbErrorKind::DeserializationError) when:
    ///   - failed in deserialization.
    fn unpack(raw: &[u8]) -> ApllodbResult<Self> {
        bincode::deserialize(raw).map_err(|e| {
            ApllodbError::new(
                ApllodbErrorKind::DeserializationError,
                format!("failed to unpack data as {}", type_name::<Self>()),
                Some(Box::new(e)),
            )
        })
    }

    /// SQL types which can hold all values of Self.
    fn to_sql_types() -> HashSet<SqlType>;

    /// SQL types all of whose values can be held by Self.
    fn from_sql_types() -> HashSet<SqlType>;
}
