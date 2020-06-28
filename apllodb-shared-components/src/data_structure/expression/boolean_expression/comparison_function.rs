use crate::data_structure::Expression;
use serde::{Deserialize, Serialize};

/// Comparison function and its operators
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum ComparisonFunction {
    /// `=` operation
    EqualVariant {
        left: Box<Expression>,
        right: Box<Expression>,
    },
}
