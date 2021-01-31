use serde::{Deserialize, Serialize};

/// unary operator for an expression
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub enum UnaryOperator {
    /// -
    Minus,
}
