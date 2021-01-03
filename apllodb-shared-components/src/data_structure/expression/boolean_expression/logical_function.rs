use super::BooleanExpression;
use serde::{Deserialize, Serialize};

/// AND, OR, NOT
#[derive(Clone, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum LogicalFunction {
    /// `AND` operation
    AndVariant {
        /// Left operand
        left: Box<BooleanExpression>,
        /// Right operand
        right: Box<BooleanExpression>,
    },
}
