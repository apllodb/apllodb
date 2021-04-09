use crate::{AliasedFieldName, ApllodbResult, RPos, Schema, SchemaIndex};
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
    inner: Vec<(RPos, Option<AliasedFieldName>)>,
}

impl Schema for RecordSchema {
    type Name = AliasedFieldName;

    fn names_with_pos(&self) -> Vec<(RPos, Option<AliasedFieldName>)> {
        self.inner.clone()
    }
}

impl RecordSchema {
    pub(crate) fn assert_all_named(&self) {
        assert!(self.inner.iter().all(|(_, opt)| opt.is_some()));
    }

    /// Filter specified fields
    pub(crate) fn projection(&self, indexes: &[SchemaIndex]) -> ApllodbResult<Self> {
        let new_inner: Vec<(RPos, Option<AliasedFieldName>)> = indexes
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
                .map(|(raw_pos, name)| (RPos::new(raw_pos), Some(name)))
                .collect(),
        }
    }
}
