pub(crate) mod record_index;
pub(crate) mod record_schema;

use std::{collections::HashMap, sync::Arc};

use apllodb_shared_components::{
    AliasedFieldName, ApllodbErrorKind, ApllodbResult, Expression, Ordering, RPos, Schema,
    SchemaIndex, SqlValue, SqlValueHashKey,
};

/// Seq of [Row](crate::Row)s.
#[derive(Clone, PartialEq, Debug)]
pub struct Records {
    schema: Arc<RecordSchema>,
    inner: Vec<SqlValues>,
}

impl Records {
    /// Constructor
    pub fn new<IntoValues: Into<SqlValues>, I: IntoIterator<Item = IntoValues>>(
        schema: RecordSchema,
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

    /// ref to schema
    pub fn as_schema(&self) -> &RecordSchema {
        self.schema.as_ref()
    }

    /// makes SqlValues
    pub fn into_sql_values(self) -> Vec<SqlValues> {
        self.inner.into_iter().collect()
    }

    /// Filter records that satisfy the given `condition`.
    pub fn selection(self, condition: &Expression) -> ApllodbResult<Self> {
        match condition {
            Expression::ConstantVariant(sql_value) => {
                if sql_value.to_bool()? {
                    Ok(self)
                } else {
                    Ok(Self {
                        schema: self.schema,
                        inner: vec![],
                    })
                }
            }
            _ => {
                let schema = self.as_schema().clone();
                self.into_iter()
                    .filter_map(|r| {
                        match condition
                            .to_sql_value(Some((&r, &schema)))
                            .and_then(|sql_value| sql_value.to_bool())
                        {
                            Ok(false) => None,
                            Ok(true) => Some(Ok(r)),
                            Err(e) => Some(Err(e)),
                        }
                    })
                    .collect::<ApllodbResult<Vec<Row>>>()
                    .map(|records| Self::new(schema, records))
            }
        }
    }

    /// Horizontally shrink records.
    ///
    /// # Failures
    ///
    /// - [InvalidName](crate::ApllodbErrorKind::InvalidName) when:
    ///   - Specified field does not exist in this record.
    pub fn projection(self, indexes: &[SchemaIndex]) -> ApllodbResult<Self> {
        let new_schema = self.schema.projection(indexes)?;

        let projection_positions = indexes
            .iter()
            .map(|idx| {
                let (pos, _) = self.schema.index(idx)?;
                Ok(pos)
            })
            .collect::<ApllodbResult<Vec<RPos>>>()?;

        let new_inner: Vec<SqlValues> = self
            .inner
            .into_iter()
            .map(|sql_values| sql_values.projection(&projection_positions))
            .collect();

        Ok(Self::new(new_schema, new_inner))
    }

    /// ORDER BY
    pub fn sort(mut self, field_orderings: &[(SchemaIndex, Ordering)]) -> ApllodbResult<Self> {
        assert!(!field_orderings.is_empty(), "parser should avoid this case");

        // TODO check if type in FieldIndex is PartialOrd

        let schema = self.schema.clone();

        self.inner.sort_by(|a, b| {
            let mut res = std::cmp::Ordering::Equal;

            for (index, ord) in field_orderings {
                let (pos, _) = schema
                    .index(&index)
                    .unwrap_or_else(|_| panic!("must be valid field: `{}`", index));

                let a_val = a.get(pos);
                let b_val = b.get(pos);

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

    /// Join algorithm using hash table.
    /// It can be used with join keys' equality (like `ON t.id = s.t_id`).
    /// This algorithm's time-complexity is `max[O(len(self)), O(len(right_records))]` but uses relatively large memory.
    ///
    /// # Failures
    ///
    /// - [InvalidName](crate::ApllodbErrorKind::InvalidName) when:
    ///   - Specified field does not exist in any record.
    pub fn hash_join(
        self,
        joined_schema: RecordSchema,
        right_records: Records,
        self_join_field: &SchemaIndex,
        right_join_field: &SchemaIndex,
    ) -> ApllodbResult<Self> {
        joined_schema.assert_all_named();

        fn helper_get_sql_value(
            joined_name: &AliasedFieldName,
            schema: &RecordSchema,
            record: &Row,
        ) -> Option<ApllodbResult<SqlValue>> {
            schema.index(&SchemaIndex::from(joined_name)).map_or_else(
                |e| {
                    if matches!(e.kind(), ApllodbErrorKind::InvalidName) {
                        None
                    } else {
                        Some(Err(e))
                    }
                },
                |(pos, _)| {
                    let res_sql_value = record.get_sql_value(pos).map(|v| v.clone());
                    Some(res_sql_value)
                },
            )
        }

        fn helper_join_records(
            joined_schema: &RecordSchema,
            left_schema: &RecordSchema,
            right_schema: &RecordSchema,
            left_record: Row,
            right_record: Row,
        ) -> ApllodbResult<Row> {
            let sql_values: Vec<SqlValue> = joined_schema
                .to_aliased_field_names()
                .iter()
                .map(|joined_name| {
                    helper_get_sql_value(joined_name, left_schema, &left_record)
                        .or_else(|| helper_get_sql_value(joined_name, right_schema, &right_record))
                        .expect("left or right must have AliasedFieldName in joined_schema")
                })
                .collect::<ApllodbResult<_>>()?;
            let sql_values = SqlValues::new(sql_values);
            Ok(Row::new(sql_values))
        }

        let self_schema = self.as_schema().clone();
        let right_schema = right_records.as_schema().clone();

        let (self_join_idx, _) = self.as_schema().index(self_join_field)?;
        let (right_join_idx, _) = right_records.as_schema().index(&right_join_field)?;

        // TODO Create hash table from smaller input.
        let mut hash_table = HashMap::<SqlValueHashKey, Vec<Row>>::new();

        for left_record in self {
            let left_sql_value = left_record.get_sql_value(self_join_idx)?;
            hash_table
                .entry(SqlValueHashKey::from(left_sql_value))
                // FIXME Clone less. If join keys are unique, no need for clone.
                .and_modify(|records| records.push(left_record.clone()))
                .or_insert_with(|| vec![left_record]);
        }

        let mut records = Vec::<Row>::new();
        for right_record in right_records {
            let right_sql_value = right_record.get_sql_value(right_join_idx)?;
            if let Some(left_records) = hash_table.get(&SqlValueHashKey::from(right_sql_value)) {
                records.append(
                    &mut left_records
                        .iter()
                        .map(|left_record| {
                            helper_join_records(
                                &joined_schema,
                                &self_schema,
                                &right_schema,
                                left_record.clone(),
                                right_record.clone(),
                            )
                        })
                        .collect::<ApllodbResult<Vec<Row>>>()?,
                );
            }
        }

        Ok(Records::new(joined_schema, records))
    }
}

impl Iterator for Records {
    type Item = Row;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.is_empty() {
            None
        } else {
            let values = self.inner.remove(0);
            Some(Row::new(values))
        }
    }
}
