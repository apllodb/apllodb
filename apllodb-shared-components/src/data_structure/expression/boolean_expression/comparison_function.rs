use serde::{Deserialize, Serialize};

use crate::data_structure::expression::Expression;

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
