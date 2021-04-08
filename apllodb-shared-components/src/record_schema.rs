use crate::{
    record_index::named_record_index::NamedRecordIndex, AliasedFieldName, ApllodbError,
    ApllodbErrorKind, ApllodbResult, RecordIndex, RecordPos,
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
    pub(crate) fn assert_all_named(&self) {
        assert!(self.inner.iter().all(|(_, opt)| opt.is_some()));
    }

    /// Finds a pair of (RecordPos, AliasedFieldName) of a field specified by RecordIndex.
    ///
    /// # Failures
    ///
    /// - [InvalidName](crate::ApllodbErrorKind::InvalidName) when:
    ///   - no field matches to this RecordIndex.
    /// - [AmbiguousColumn](crate::ApllodbErrorKind::AmbiguousColumn) when:
    ///   - more than 1 of fields match to this FieldIndex.
    pub(crate) fn index(
        &self,
        named_idx: &NamedRecordIndex,
    ) -> ApllodbResult<(RecordPos, AliasedFieldName)> {
        let matching_pair: Vec<(RecordPos, AliasedFieldName)> = self
            .inner
            .iter()
            .filter_map(|(pos, opt_field)| {
                opt_field
                    .as_ref()
                    .map(|field| {
                        if field.matches(named_idx) {
                            Some((*pos, field.clone()))
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
                format!("no field matches to: {:?}", named_idx),
                None,
            ))
        } else {
            Err(ApllodbError::new(
                ApllodbErrorKind::AmbiguousColumn,
                format!("more than 1 fields match to: {:?}", named_idx),
                None,
            ))
        }
    }

    /// Filter specified fields
    pub(crate) fn projection(&self, indexes: &[NamedRecordIndex]) -> ApllodbResult<Self> {
        let new_inner: Vec<(RecordPos, Option<AliasedFieldName>)> = indexes
            .iter()
            .map(|index| {
                let (pos, name) = self.index(index)?;
                Ok((pos, Some(name)))
            })
            .collect::<ApllodbResult<_>>()?;
        Ok(Self { inner: new_inner })
    }

    /// get raw AliasFieldNames
    ///
    /// # Panics
    ///
    /// if any field is unnamed (even un-aliased) constant.
    pub(crate) fn to_aliased_field_names(&self) -> Vec<AliasedFieldName> {
        self.assert_all_named();
        self.inner
            .iter()
            .map(|(_, opt_name)| opt_name.as_ref().expect("already checked").clone())
            .collect()
    }
}

impl From<Vec<AliasedFieldName>> for RecordSchema {
    fn from(names: Vec<AliasedFieldName>) -> Self {
        Self {
            inner: names
                .into_iter()
                .enumerate()
                .map(|(raw_pos, name)| (RecordPos::new(raw_pos), Some(name)))
                .collect(),
        }
    }
}
