use serde::{Deserialize, Serialize};

use super::general_data_type::{GeneralDataType, OrderedGeneralDataType};

/// Data type kind.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum DataTypeKind {
    /// 2-byte signed integer.
    SmallInt,

    /// 4-byte signed integer.
    Integer,

    /// 8-byte signed integer.
    BigInt,

    /// Arbitrary length text (UTF-8).
    Text,
}

impl DataTypeKind {
    /// Its GeneralDataType.
    pub fn general_data_type(&self) -> GeneralDataType {
        match self {
            DataTypeKind::SmallInt | DataTypeKind::Integer | DataTypeKind::BigInt => {
                GeneralDataType::Ordered(OrderedGeneralDataType::Number)
            }
            DataTypeKind::Text => GeneralDataType::Ordered(OrderedGeneralDataType::Text),
        }
    }
}
