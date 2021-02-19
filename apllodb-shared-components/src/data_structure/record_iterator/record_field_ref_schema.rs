use serde::{Deserialize, Serialize};

use crate::{ApllodbResult, ColumnName, CorrelationName, FieldIndex, FieldReference, FromItem, FullFieldReference, TableName, SelectFieldReference};

/// Internally has similar structure as `Vec<FullFieldReference>` and works with [SqlValues](crate::SqlValues) with the same length
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct RecordFieldRefSchema(Vec<FullFieldReference>);

impl RecordFieldRefSchema {
    /// Constructor
    ///
    /// Each field may contain alias.
    pub fn new_for_select(full_field_references: Vec<FullFieldReference>) -> Self {
        Self(full_field_references)
    }

    /// Constructor
    pub fn new_for_modification(table_name: TableName, column_names: Vec<ColumnName>) -> Self {
        let correlation_name = CorrelationName::from(table_name);
        let ffrs: Vec<FullFieldReference> = column_names
            .into_iter()
            .map(|column_name| {
                let sfr = SelectFieldReference::new(
                    Some(correlation_name),
                    FieldReference::from(column_name),
                );
                sfr.resolve(Some(FromItem::TableVariant(TableWithAlias {
                    table_name

                })))
            })
            .collect();
    }

    /// # Failures
    ///
    /// see: [FieldIndex::peek](crate::FieldIndex::peek)
    pub(crate) fn resolve_index(&self, index: &FieldIndex) -> ApllodbResult<usize> {
        let (idx, _) = index.peek(self.0.clone())?;
        Ok(idx)
    }

    pub(crate) fn projection(&self, projection: &[FieldIndex]) -> ApllodbResult<Self> {
        let new_ffrs: Vec<FullFieldReference> = projection
            .iter()
            .map(|index| {
                let (_, ffr) = index.peek(self.0.clone())?;
                Ok(ffr.clone())
            })
            .collect::<ApllodbResult<_>>()?;
        Ok(Self(new_ffrs))
    }

    pub(crate) fn joined(&self, right: &Self) -> Self {
        let mut left = self.0.clone();
        let mut right = right.0.clone();
        left.append(&mut right);
        Self(left)
    }

    pub(crate) fn as_full_field_references(&self) -> &[FullFieldReference] {
        &self.0
    }
}
