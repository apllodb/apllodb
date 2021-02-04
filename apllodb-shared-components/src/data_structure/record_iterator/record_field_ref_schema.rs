use serde::{Deserialize, Serialize};

use crate::{ApllodbResult, FieldIndex, FullFieldReference};

/// Internally has similar structure as `Vec<FullFieldReference>` and works with [SqlValues](crate::SqlValues) with the same length
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct RecordFieldRefSchema(Vec<FullFieldReference>);

impl RecordFieldRefSchema {
    /// # Failures
    ///
    /// see: [FieldIndex::peek](crate::FieldIndex::peek)
    pub(crate) fn resolve_index(&self, index: &FieldIndex) -> ApllodbResult<usize> {
        let (idx, _) = index.peek(&self.0)?;
        Ok(idx)
    }

    pub(crate) fn projection(&self, projection: &[FieldIndex]) -> ApllodbResult<Self> {
        let new_ffrs: Vec<FullFieldReference> = projection
            .iter()
            .map(|index| {
                let (_, ffr) = index.peek(&self.0)?;
                Ok(ffr.clone())
            })
            .collect::<ApllodbResult<_>>()?;
        Ok(Self(new_ffrs))
    }
}
