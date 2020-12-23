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
    /// Constructor
    ///
    /// TODO validate mixed UNIQUE and PRIMARY KEY
    pub fn new(kinds: Vec<ColumnConstraintKind>) -> ApllodbResult<Self> {
        Ok(Self { kinds })
    }

    /// Ref to seq of [ColumnConstraintKind](crate::ColumnConstraintKind).
    pub fn kinds(&self) -> &[ColumnConstraintKind] {
        &self.kinds
    }
}
