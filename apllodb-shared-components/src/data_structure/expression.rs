mod constant;

pub use constant::{Constant, IntegerConstant, NumericConstant};

use super::ColumnName;
use serde::{Deserialize, Serialize};

/// Expression.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum Expression {
    ConstantVariant(Constant),
    ColumnNameVariant(ColumnName),
}
