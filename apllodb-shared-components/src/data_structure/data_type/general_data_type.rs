use serde::{Deserialize, Serialize};

/// A [DataTypeKind](crate::DataTypeKind) belongs to a GeneralDataType.
/// Some GeneralDataType are **ordered** and others are not.
///
/// See also: [SqlValue](crate::SqlValue)
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum GeneralDataType {
    Ordered(OrderedGeneralDataType),
    NonOrdered(NonOrderedGeneralDataType),
}

/// Ordered types
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum OrderedGeneralDataType {
    Number,
    Text,
}

/// Non-ordered types
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum NonOrderedGeneralDataType {}
