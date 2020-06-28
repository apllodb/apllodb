use serde::{Deserialize, Serialize};
use super::BooleanExpression;

/// AND, OR, NOT
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum LogicalFunction {
    /// `AND` operation
    AndVariant {
        left: Box<BooleanExpression>,
        right: Box<BooleanExpression>,
    },
}
