use crate::{
    data_structure::{DataTypeKind, SqlValue},
    error::ApllodbErrorKind,
    traits::SqlConvertible,
};
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
        fn core<T: SqlConvertible, F: FnOnce(T) -> Constant>(
            sql_value: &SqlValue,
            rust_val_to_constant: F,
        ) -> Constant {
            let msg = "DatatypeMismatch never happen if pattern-match here is correct";

            match sql_value.unpack::<Option<T>>() {
                Ok(opt) => match opt {
                    None => Constant::Null,
                    Some(v) => rust_val_to_constant(v),
                },
                Err(e) => match e.kind() {
                    ApllodbErrorKind::DatatypeMismatch => {
                        let v = sql_value.unpack::<T>().expect(msg);
                        rust_val_to_constant(v)
                    }
                    _ => panic!("unexpected error `{}` with sql_value=`{:?}`", e, sql_value),
                },
            }
        }

        match sql_value.data_type().kind() {
            DataTypeKind::SmallInt => core::<i16, _>(sql_value, Self::from),
            DataTypeKind::Integer => core::<i32, _>(sql_value, Self::from),
            DataTypeKind::BigInt => core::<i64, _>(sql_value, Self::from),
            DataTypeKind::Text => core::<String, _>(sql_value, Self::from),
        }
    }
}
