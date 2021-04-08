use serde::{Deserialize, Serialize};

use crate::{ApllodbResult, CorrelationIndex, FieldIndex, FullFieldReference, data_structure::record::record_pos::RecordPos};

/// Internally has similar structure as `Vec<FullFieldReference>` and works with [SqlValues](crate::SqlValues) with the same length
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct RecordFieldRefSchema(Vec<FullFieldReference>);

impl RecordFieldRefSchema {
    /// Constructor
    pub fn new(full_field_references: Vec<FullFieldReference>) -> Self {
        Self(full_field_references)
    }

    /// Number of fields
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// is empty?
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// # Failures
    ///
    /// see: [FieldIndex::peek](crate::FieldIndex::peek)
    pub fn resolve_index(&self, index: &FieldIndex) -> ApllodbResult<RecordPos> {
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
    pub fn filter_by_correlations(&self, correlation_indexes: &[CorrelationIndex]) -> Self {
        let new_ffrs: Vec<FullFieldReference> = self
            .0
            .iter()
            .filter(|ffr| {
                correlation_indexes.iter().any(|correlation_index| {
                    correlation_index.matches(ffr.as_correlation_reference())
                })
            })
            .cloned()
            .collect();
        Self(new_ffrs)
    }

    /// get raw FFR
    pub fn as_full_field_references(&self) -> &[FullFieldReference] {
        &self.0
    }
}
