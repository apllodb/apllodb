use serde::{Deserialize, Serialize};

use crate::{ApllodbResult, FieldIndex, FullFieldReference};

/// Internally has similar structure as `Vec<FullFieldReference>` and works with [SqlValues](crate::SqlValues) with the same length
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct RecordFieldRefSchema(Vec<FullFieldReference>);

impl RecordFieldRefSchema {
    /// # Failures
    ///
    /// see: [FieldIndex::peek](crate::FieldIndex::peek)
    pub fn resolve_index(&self, index: &FieldIndex) -> ApllodbResult<usize> {
        let (idx, _) = index.peek(&self.0)?;
        Ok(idx)
    }
}
