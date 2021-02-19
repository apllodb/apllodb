pub(crate) mod record_field_ref_schema;

use std::sync::Arc;

use crate::{
    ApllodbResult, Expression, FieldIndex, FullFieldReference, Ordering, Record, SqlValue,
    SqlValues,
};

use self::record_field_ref_schema::RecordFieldRefSchema;

/// Iterator of [Record](crate::Record)s.
///
/// Note that Record is always generated from RecordIterator, who has ownership to [RecordFieldRefSchema](crate::RecordFieldRefSchema).
#[derive(Clone, PartialEq, Debug)]
pub struct RecordIterator {
    schema: Arc<RecordFieldRefSchema>,
    inner: Vec<SqlValues>,
}

impl RecordIterator {
    /// Constructor
    pub fn new<IntoValues: Into<SqlValues>, I: IntoIterator<Item = IntoValues>>(
        schema: RecordFieldRefSchema,
        it: I,
    ) -> Self {
        Self {
            schema: Arc::new(schema),
            inner: it
                .into_iter()
                .map(|into_values| into_values.into())
                .collect(),
        }
    }

    /// get FullFieldReferences
    pub fn as_full_field_references(&self) -> &[FullFieldReference] {
        self.schema.as_full_field_references()
    }

    /// ref to schema
    pub fn as_schema(&self) -> &RecordFieldRefSchema {
        self.schema.as_ref()
    }

    /// makes SqlValues
    pub fn into_sql_values(self) -> Vec<SqlValues> {
        self.inner.into_iter().collect()
    }

    /// Filter records that satisfy the given `condition`.
    pub fn selection(self, condition: &Expression) -> ApllodbResult<Self> {
        let records: Vec<Record> = self
            .into_iter()
            .filter_map(|record| match record.selection(condition) {
                Ok(false) => None,
                Ok(true) => Some(Ok(record)),
                Err(e) => Some(Err(e)),
            })
            .collect::<ApllodbResult<_>>()?;

        Ok(Self::from(records))
    }

    /// Shrink records into record with specified `fields`.
    ///
    /// # Failures
    ///
    /// - [InvalidName](crate::ApllodbErrorKind::InvalidName) when:
    ///   - Specified field does not exist in this record.
    pub fn projection(self, projection: &[FieldIndex]) -> ApllodbResult<Self> {
        let projection_idxs = projection
            .iter()
            .map(|index| self.schema.resolve_index(index))
            .collect::<ApllodbResult<Vec<usize>>>()?;

        let new_schema = self.schema.projection(projection)?;

        let new_inner: Vec<SqlValues> = self
            .inner
            .into_iter()
            .map(|sql_values| sql_values.projection(&projection_idxs))
            .collect();

        Ok(Self::new(new_schema, new_inner))
    }

    /// ORDER BY
    pub fn sort(mut self, field_orderings: &[(FieldIndex, Ordering)]) -> ApllodbResult<Self> {
        assert!(!field_orderings.is_empty(), "parser should avoid this case");

        // TODO check if type in FieldIndex is PartialOrd

        let schema = self.schema.clone();

        self.inner.sort_by(|a, b| {
            let mut res = std::cmp::Ordering::Equal;

            for (index, ord) in field_orderings {
                let idx = schema
                    .resolve_index(&index)
                    .unwrap_or_else(|_| panic!("must be valid field: `{}`", index));

                let a_val = a.get(idx);
                let b_val = b.get(idx);

                match a_val.sql_compare(&b_val).unwrap_or_else(|_| {
                    panic!(
                    "two records in the same RecordIterator must have the same type for field `{}`",
                    index
                )
                }) {
                    crate::SqlCompareResult::Eq => res = std::cmp::Ordering::Equal,
                    crate::SqlCompareResult::LessThan => {
                        match ord {
                            Ordering::Asc => {
                                res = std::cmp::Ordering::Less;
                            }
                            Ordering::Desc => {
                                res = std::cmp::Ordering::Greater;
                            }
                        }
                        break;
                    }
                    crate::SqlCompareResult::GreaterThan => {
                        match ord {
                            Ordering::Asc => {
                                res = std::cmp::Ordering::Greater;
                            }
                            Ordering::Desc => {
                                res = std::cmp::Ordering::Less;
                            }
                        }
                        break;
                    }
                    crate::SqlCompareResult::Null => {
                        // NULL comes last, regardless of ASC/DESC
                        if let SqlValue::Null = a_val {
                            res = std::cmp::Ordering::Greater
                        } else {
                            res = std::cmp::Ordering::Less
                        }
                        break;
                    }
                    crate::SqlCompareResult::NotEq => {
                        unreachable!("sort key `{}` must be at least PartialOrd", index)
                    }
                }
            }
            res
        });
        Ok(self)
    }
}

impl Iterator for RecordIterator {
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.is_empty() {
            None
        } else {
            let values = self.inner.remove(0);
            Some(Record::new(self.schema.clone(), values))
        }
    }
}

impl From<Vec<Record>> for RecordIterator {
    fn from(rs: Vec<Record>) -> Self {
        let schema = if rs.is_empty() {
            RecordFieldRefSchema::new_for_select(vec![])
        } else {
            let r = rs.first().unwrap();
            r.schema().clone()
        };

        Self::new(schema, rs)
    }
}
