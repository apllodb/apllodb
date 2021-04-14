use std::collections::HashSet;

use apllodb_shared_components::{RPos, Schema};
use serde::{Deserialize, Serialize};

use crate::field::aliased_field_name::AliasedFieldName;

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
    inner: Vec<(RPos, Option<AliasedFieldName>)>,
}

impl Schema for RecordSchema {
    type Name = AliasedFieldName;

    fn new(names_with_pos: Vec<(RPos, Option<AliasedFieldName>)>) -> Self
    where
        Self: Sized,
    {
        Self {
            inner: names_with_pos,
        }
    }

    fn names_with_pos(&self) -> Vec<(RPos, Option<AliasedFieldName>)> {
        self.inner.clone()
    }

    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl RecordSchema {
    pub(crate) fn assert_all_named(&self) {
        assert!(self.inner.iter().all(|(_, opt)| opt.is_some()));
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

impl From<HashSet<AliasedFieldName>> for RecordSchema {
    /// Makes unique & sorted AliasedFieldNames
    fn from(names: HashSet<AliasedFieldName>) -> Self {
        let mut vec: Vec<AliasedFieldName> = names.into_iter().collect();
        vec.sort();
        vec.dedup();

        Self {
            inner: vec
                .into_iter()
                .enumerate()
                .map(|(raw_pos, name)| (RPos::new(raw_pos), Some(name)))
                .collect(),
        }
    }
}
