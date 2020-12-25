pub(crate) mod comparison_function;
pub(crate) mod logical_function;

use serde::{Deserialize, Serialize};

use self::{comparison_function::ComparisonFunction, logical_function::LogicalFunction};

/// Boolean expression.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum BooleanExpression {
    /// AND, OR, NOT
    LogicalFunctionVariant(LogicalFunction),

    /// Comparison functions
    ComparisonFunctionVariant(ComparisonFunction),
}
