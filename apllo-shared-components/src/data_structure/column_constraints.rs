use super::column_constraint_kind::ColumnConstraintKind;
use crate::error::AplloResult;
use serde::{Deserialize, Serialize};

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct ColumnConstraints {
    kinds: Vec<ColumnConstraintKind>,
}

impl ColumnConstraints {
    /// TODO UNIQUE, PK は共存できないというバリデーション
    pub fn new(kinds: Vec<ColumnConstraintKind>) -> AplloResult<Self> {
        Ok(Self { kinds })
    }

    /// Ref to seq of [ColumnConstraintKind](enum.ColumnConstraintKind.html).
    pub fn kinds(&self) -> &[ColumnConstraintKind] {
        &self.kinds
    }
}
