use serde::{Deserialize, Serialize};

/// Constant.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum Constant {
    NumericConstantVariant(NumericConstant),
}

/// Numeric constant.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum NumericConstant {
    IntegerConstantVariant(IntegerConstant),
}

/// Integer constant.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct IntegerConstant(u64); // TODO re-think about data size

impl From<u64> for Constant {
    fn from(v: u64) -> Self {
        Self::NumericConstantVariant(NumericConstant::IntegerConstantVariant(IntegerConstant(v)))
    }
}
