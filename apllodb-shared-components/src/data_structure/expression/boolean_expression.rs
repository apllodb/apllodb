mod comparison_function;
mod logical_function;

pub use comparison_function::ComparisonFunction;
pub use logical_function::LogicalFunction;

use serde::{Deserialize, Serialize};

/// Boolean expression.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum BooleanExpression {
    /// AND, OR, NOT
    LogicalFunctionVariant(LogicalFunction),

    /// Comparison function and its operators
    ComparisonFunctionVariant(ComparisonFunction),
}
