use crate::{
    record_index::named_record_index::NamedRecordIndex, AliasedFieldName, ApllodbError,
    ApllodbErrorKind, ApllodbResult, RecordIndex, RPos, TableColumnName,
};
use serde::{Deserialize, Serialize};

use super::row_index::RowIndex;

/// Schema of [Row](crate::Row)s holding pairs of (RowPos, TableColumnName).
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct RowSchema {
    inner: Vec<(RPos, TableColumnName)>,
}

impl RowSchema {
    /// Finds a pair of (RecordPos, AliasedFieldName) of a field specified by RecordIndex.
    ///
    /// # Failures
    ///
    /// - [InvalidName](crate::ApllodbErrorKind::InvalidName) when:
    ///   - no field matches to this RowIndex.
    /// - [AmbiguousColumn](crate::ApllodbErrorKind::AmbiguousColumn) when:
    ///   - more than 1 of fields match to this FieldIndex.
    pub(crate) fn index(&self, idx: &RowIndex) -> ApllodbResult<(RPos, TableColumnName)> {
        let matching_pair: Vec<(RPos, TableColumnName)> = self
            .inner
            .iter()
            .filter_map(|(pos, opt_tc)| {
                opt_tc
                    .as_ref()
                    .map(|tc| {
                        if tc.matches(idx) {
                            Some((*pos, tc.clone()))
                        } else {
                            None
                        }
                    })
                    .flatten()
            })
            .collect();

        if matching_pair.len() == 1 {
            matching_pair.first().cloned().ok_or_else(|| unreachable!())
        } else if matching_pair.is_empty() {
            Err(ApllodbError::new(
                ApllodbErrorKind::InvalidName,
                format!("no field matches to: {:?}", idx),
                None,
            ))
        } else {
            Err(ApllodbError::new(
                ApllodbErrorKind::AmbiguousColumn,
                format!("more than 1 fields match to: {:?}", idx),
                None,
            ))
        }
    }
}
