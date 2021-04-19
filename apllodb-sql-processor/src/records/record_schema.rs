use std::collections::HashSet;

use apllodb_shared_components::{RPos, Schema};
use apllodb_storage_engine_interface::RowSchema;
use sorted_vec::SortedSet;

use crate::{
    aliaser::Aliaser, correlation::aliased_correlation_name::AliasedCorrelationName,
    field::aliased_field_name::AliasedFieldName,
};

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
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub(crate) struct RecordSchema {
    named: SortedSet<AliasedFieldName>,
    unnamed_len: usize,
}

impl Schema for RecordSchema {
    type Name = AliasedFieldName;

    fn new(names: HashSet<AliasedFieldName>, unnamed_fields_len: usize) -> Self
    where
        Self: Sized,
    {
        Self {
            named: SortedSet::from_unsorted(names.into_iter().collect()),
            unnamed_len: unnamed_fields_len,
        }
    }

    fn names_with_pos(&self) -> Vec<(RPos, Option<AliasedFieldName>)> {
        let mut v: Vec<(RPos, Option<AliasedFieldName>)> = self
            .named
            .iter()
            .enumerate()
            .map(|(raw_pos, afn)| (RPos::new(raw_pos), Some(afn.clone())))
            .collect();

        for _ in 0..self.unnamed_len {
            let pos = v.len();
            v.push((RPos::new(pos), None))
        }
        v
    }

    fn len(&self) -> usize {
        self.named.len() + self.unnamed_len
    }
}

impl RecordSchema {
    pub(crate) fn assert_all_named(&self) {
        assert!(self.unnamed_len == 0);
    }

    pub(crate) fn from_row_schema(row_schema: &RowSchema, aliaser: Aliaser) -> Self {
        let aliased_field_names: HashSet<AliasedFieldName> = row_schema
            .table_column_names()
            .iter()
            .map(|tc| aliaser.alias(tc))
            .collect();
        Self::from(aliased_field_names)
    }

    /// get raw AliasFieldNames
    ///
    /// # Panics
    ///
    /// if any field is unnamed (even un-aliased) constant.
    pub(crate) fn to_aliased_field_names(&self) -> Vec<AliasedFieldName> {
        self.assert_all_named();
        self.named.iter().cloned().collect()
    }

    /// Filter fields specified by AliasedCorrelationName.
    /// Used for a storage engine access.
    pub(crate) fn filter_by_correlations(
        &self,
        from_item_correlations: &[AliasedCorrelationName],
    ) -> Self {
        let new_afns: HashSet<AliasedFieldName> = self
            .to_aliased_field_names()
            .iter()
            .filter(|afn| {
                from_item_correlations
                    .iter()
                    .any(|need_corr| need_corr == &afn.field_name.aliased_correlation_name)
            })
            .cloned()
            .collect();
        Self::from(new_afns)
    }
}

impl From<HashSet<AliasedFieldName>> for RecordSchema {
    fn from(names: HashSet<AliasedFieldName>) -> Self {
        Self::new(names, 0)
    }
}
