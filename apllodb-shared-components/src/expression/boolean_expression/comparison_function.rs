use serde::{Deserialize, Serialize};

use crate::Expression;

/// Comparison function and its operands
#[derive(Clone, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum ComparisonFunction {
    /// `=` operation
    EqualVariant {
        /// Left operand
        left: Box<Expression>,
        /// Right operand
        right: Box<Expression>,
    },
}
