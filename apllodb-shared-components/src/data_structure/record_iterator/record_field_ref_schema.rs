use serde::{Deserialize, Serialize};

use crate::{ApllodbResult, FieldIndex, FromItem, FullFieldReference};

/// [Record](crate::Record)'s schema.
///
/// Internally has similar structure as `Vec<FullFieldReference>` and works with [SqlValues](crate::SqlValues) with the same length
#[derive(Clone, PartialEq, Hash, Debug, Serialize, Deserialize, new)]
pub struct RecordFieldRefSchema {
    from_item: FromItem,
    fields: Vec<FullFieldReference>,
}

impl RecordFieldRefSchema {
    /// # Failures
    ///
    /// see: [FieldIndex::peek](crate::FieldIndex::peek)
    pub(crate) fn resolve_index(&self, index: &FieldIndex) -> ApllodbResult<usize> {
        let (idx, _) = index.peek(&self)?;
        Ok(idx)
    }

    pub(crate) fn projection(&self, projection: &[FieldIndex]) -> ApllodbResult<Self> {
        let new_ffrs: Vec<FullFieldReference> = projection
            .iter()
            .map(|index| {
                let (_, ffr) = index.peek(&self)?;
                Ok(ffr.clone())
            })
            .collect::<ApllodbResult<_>>()?;
        Ok(Self::new(self.from_item.clone(), new_ffrs))
    }

    pub(crate) fn joined(&self, right: &Self) -> Self {
        assert_eq!(
            self.from_item, right.from_item,
            "Records to join must be from the same SELECT SQL (same FROM item)"
        );

        let (mut left_fields, mut right_fields) = (self.fields.clone(), right.fields.clone());
        left_fields.append(&mut right_fields);
        Self::new(self.from_item.clone(), left_fields)
    }

    pub(crate) fn as_full_field_references(&self) -> &[FullFieldReference] {
        &self.fields
    }
}
