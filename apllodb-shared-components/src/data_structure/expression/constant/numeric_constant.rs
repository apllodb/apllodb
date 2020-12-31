use serde::{Deserialize, Serialize};

use crate::Constant;

/// Numeric constant.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum NumericConstant {
    /// Integer constant.
    IntegerConstantVariant(IntegerConstant),
}

/// Integer constant.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum IntegerConstant {
    /// 64-bit signed integer
    I64(i64),
    /// 32-bit signed integer
    I32(i32),
    /// 16-bit signed integer
    I16(i16),
}
impl IntegerConstant {
    /// Get as i64 value
    pub fn as_i64(&self) -> i64 {
        match self {
            IntegerConstant::I64(v) => *v as i64,
            IntegerConstant::I32(v) => *v as i64,
            IntegerConstant::I16(v) => *v as i64,
        }
    }
}

impl From<i64> for Constant {
    fn from(v: i64) -> Self {
        Self::NumericConstantVariant(NumericConstant::IntegerConstantVariant(
            IntegerConstant::I64(v),
        ))
    }
}
impl From<i32> for Constant {
    fn from(v: i32) -> Self {
        Self::NumericConstantVariant(NumericConstant::IntegerConstantVariant(
            IntegerConstant::I32(v),
        ))
    }
}
impl From<i16> for Constant {
    fn from(v: i16) -> Self {
        Self::NumericConstantVariant(NumericConstant::IntegerConstantVariant(
            IntegerConstant::I16(v),
        ))
    }
}
