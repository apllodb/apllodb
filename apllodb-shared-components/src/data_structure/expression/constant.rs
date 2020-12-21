use crate::data_structure::{DataTypeKind, SqlValue};
use serde::{Deserialize, Serialize};

/// Constant.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum Constant {
    /// NULL
    Null,

    /// Numeric constant.
    NumericConstantVariant(NumericConstant),

    /// Character(s) constant.
    CharacterConstantVariant(CharacterConstant),
}

/// Numeric constant.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum NumericConstant {
    /// Integer constant.
    IntegerConstantVariant(IntegerConstant),
}

/// Integer constant.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct IntegerConstant(pub i64); // TODO re-think about data size

impl From<i64> for Constant {
    fn from(v: i64) -> Self {
        Self::NumericConstantVariant(NumericConstant::IntegerConstantVariant(IntegerConstant(v)))
    }
}
impl From<i32> for Constant {
    fn from(v: i32) -> Self {
        Self::NumericConstantVariant(NumericConstant::IntegerConstantVariant(IntegerConstant(
            v as i64,
        )))
    }
}
impl From<i16> for Constant {
    fn from(v: i16) -> Self {
        Self::NumericConstantVariant(NumericConstant::IntegerConstantVariant(IntegerConstant(
            v as i64,
        )))
    }
}

/// Character(s) constant.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum CharacterConstant {
    /// Text constant.
    TextConstantVariant(TextConstant),
}
/// Text constant (arbitrary length; UTF-8).
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct TextConstant(pub String);

impl From<String> for Constant {
    fn from(v: String) -> Self {
        Self::CharacterConstantVariant(CharacterConstant::TextConstantVariant(TextConstant(v)))
    }
}

impl From<&SqlValue> for Constant {
    fn from(sql_value: &SqlValue) -> Self {
        let msg = "DatatypeMismatch never happen if pattern-match here is correct";

        match sql_value.data_type().kind() {
            DataTypeKind::SmallInt => {
                let v: i16 = sql_value.unpack().expect(msg);
                Self::from(v)
            }
            DataTypeKind::Integer => {
                let v: i32 = sql_value.unpack().expect(msg);
                Self::from(v)
            }
            DataTypeKind::BigInt => {
                let v: i64 = sql_value.unpack().expect(msg);
                Self::from(v)
            }
            DataTypeKind::Text => {
                let v: String = sql_value.unpack().expect(msg);
                Self::from(v)
            }
        }
    }
}
