mod boolean_expression;
mod constant;

pub use boolean_expression::{BooleanExpression, ComparisonFunction, LogicalFunction};
pub use constant::{CharacterConstant, Constant, IntegerConstant, NumericConstant, TextConstant};

use super::ColumnName;
use serde::{Deserialize, Serialize};

/// Expression.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum Expression {
    /// Constant
    ConstantVariant(Constant),

    /// Reference to column value
    ColumnNameVariant(ColumnName),

    /// Boolean expression
    BooleanExpressionVariant(BooleanExpression),
}
