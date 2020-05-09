use serde::{Deserialize, Serialize};

/// Data type kind.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum DataTypeKind {
    SmallInt,
    Integer,
    BigInt,
}
