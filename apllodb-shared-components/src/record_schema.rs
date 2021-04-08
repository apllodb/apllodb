use crate::{
    AliasedFieldName, ApllodbError, ApllodbErrorKind, ApllodbResult, RecordIndex, RecordPos,
};
use serde::{Deserialize, Serialize};

/// Schema of records.
///
/// Given the following SQL:
///
/// ```sql
/// SELECT c1, t.c2, c3 AS c3a, t.c4 AS c4a, ta.c5, ta.c6 AS c6a, 777, 888 as a888 FROM t AS ta;
/// ```
///
/// then the schema of the resulting records is:
///
/// | `RecordPos` | `Option<AliasedFieldName>` |
/// |--|--|
/// | 0 | (t;ta).c1 |
/// | 1 | (t;ta).c2 |
/// | 2 | (t;ta).c3 ; c3a |
/// | 3 | (t;ta).c4 ; c4a |
/// | 4 | (t;ta).c5 |
/// | 5 | (t;ta).c6 ; c6a |
/// | 6 | - |
/// | 7 | - ; a888 |
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize)]
pub struct RecordSchema {
    inner: Vec<(RecordPos, Option<AliasedFieldName>)>,
}

impl RecordSchema {
    /// Finds a RecordPos of a field specified by RecordIndex.
    ///
    /// # Failures
    ///
    /// - [InvalidColumnReference](crate::ApllodbErrorKind::InvalidColumnReference) when:
    ///   - index is a form of RecordPos and it is out of range of this schema.
    /// - [InvalidName](crate::ApllodbErrorKind::InvalidName) when:
    ///   - no field matches to this RecordIndex.
    /// - [AmbiguousColumn](crate::ApllodbErrorKind::AmbiguousColumn) when:
    ///   - more than 1 of fields match to this FieldIndex.
    pub fn index(&self, idx: &RecordIndex) -> ApllodbResult<RecordPos> {
        match idx {
            RecordIndex::Pos(pos) => {
                if pos.to_usize() > self.inner.len() - 1 {
                    Err(ApllodbError::new(
                        ApllodbErrorKind::InvalidColumnReference,
                        format!("index out of range: {:?}", idx),
                        None,
                    ))
                } else {
                    Ok(*pos)
                }
            }
            RecordIndex::Name(named_idx) => {
                let matching_pos: Vec<RecordPos> = self
                    .inner
                    .iter()
                    .filter_map(|(pos, opt_field)| {
                        opt_field
                            .as_ref()
                            .map(|field| {
                                if field.matches(named_idx) {
                                    Some(*pos)
                                } else {
                                    None
                                }
                            })
                            .flatten()
                    })
                    .collect();

                if matching_pos.len() == 1 {
                    matching_pos
                        .first()
                        .map(|p| *p)
                        .ok_or_else(|| unreachable!())
                } else if matching_pos.is_empty() {
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
    }
}
