use serde::{Deserialize, Serialize};

/// Data type kind.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum DataTypeKind {
    /// 2-byte signed integer.
    SmallInt,

    /// 4-byte signed integer.
    Integer,

    /// 8-byte signed integer.
    BigInt,
}
