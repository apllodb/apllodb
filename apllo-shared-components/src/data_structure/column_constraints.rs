use serde::{Deserialize, Serialize};
use super::column_constraint_kind::ColumnConstraintKind;

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct ColumnConstraints {
    kinds: Vec<ColumnConstraintKind>,
}
