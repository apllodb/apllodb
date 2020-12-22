use super::column_constraint_kind::ColumnConstraintKind;
use crate::error::ApllodbResult;
use serde::{Deserialize, Serialize};

/// Constraints for column.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct ColumnConstraints {
    kinds: Vec<ColumnConstraintKind>,
}

impl Default for ColumnConstraints {
    fn default() -> Self {
        Self { kinds: vec![] }
    }
}

impl ColumnConstraints {
    /// TODO UNIQUE, PK は共存できないというバリデーション
    pub fn new(kinds: Vec<ColumnConstraintKind>) -> ApllodbResult<Self> {
        Ok(Self { kinds })
    }

    /// Ref to seq of [ColumnConstraintKind](enum.ColumnConstraintKind.html).
    pub fn kinds(&self) -> &[ColumnConstraintKind] {
        &self.kinds
    }
}
