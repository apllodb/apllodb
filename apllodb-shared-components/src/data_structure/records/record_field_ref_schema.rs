use serde::{Deserialize, Serialize};

use crate::{ApllodbResult, FieldIndex, FullFieldReference};

/// Internally has similar structure as `Vec<FullFieldReference>` and works with [SqlValues](crate::SqlValues) with the same length
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct RecordFieldRefSchema(Vec<FullFieldReference>);

impl RecordFieldRefSchema {
    /// Constructor
    pub fn new(full_field_references: Vec<FullFieldReference>) -> Self {
        Self(full_field_references)
    }

    /// # Failures
    ///
    /// see: [FieldIndex::peek](crate::FieldIndex::peek)
    pub fn resolve_index(&self, index: &FieldIndex) -> ApllodbResult<usize> {
        let (idx, _) = index.peek(&self.0)?;
        Ok(idx)
    }

    /// Filter specified fields
    pub fn projection(&self, projection: &[FieldIndex]) -> ApllodbResult<Self> {
        let new_ffrs: Vec<FullFieldReference> = projection
            .iter()
            .map(|index| {
                let (_, ffr) = index.peek(&self.0)?;
                Ok(ffr.clone())
            })
            .collect::<ApllodbResult<_>>()?;
        Ok(Self(new_ffrs))
    }

    /// Join 2 schema
    pub fn joined(&self, right: &Self) -> Self {
        let mut left = self.0.clone();
        let mut right = right.0.clone();
        left.append(&mut right);
        Self(left)
    }

    /// get raw FFR
    pub fn as_full_field_references(&self) -> &[FullFieldReference] {
        &self.0
    }
}
