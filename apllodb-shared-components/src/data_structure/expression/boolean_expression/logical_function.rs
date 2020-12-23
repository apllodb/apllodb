use super::BooleanExpression;
use serde::{Deserialize, Serialize};

/// AND, OR, NOT
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum LogicalFunction {
    /// `AND` operation
    AndVariant {
        /// Left operand
        left: Box<BooleanExpression>,
        /// Right operand
        right: Box<BooleanExpression>,
    },
}
