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

    /// Filter fields specified by CorrelationIndex.
    /// Used for a storage engine access.
    pub fn filter_by_correlation(
        &self,
        correlation_index: &CorrelationIndex,
    ) -> ApllodbResult<Self> {
        let new_ffrs: Vec<FullFieldReference> = self
            .0
            .iter()
            .filter(|ffr| correlation_index.matches(ffr))
            .collect();
        Ok(Self(new_ffrs))
    }

    /// get raw FFR
    pub fn as_full_field_references(&self) -> &[FullFieldReference] {
        &self.0
    }
}
