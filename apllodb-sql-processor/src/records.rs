pub(crate) mod record;
pub(crate) mod record_index;
pub(crate) mod record_schema;

use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use apllodb_shared_components::{
    ApllodbResult, Expression, RPos, Schema, SchemaIndex, SqlCompareResult, SqlValue,
    SqlValueHashKey,
};
use apllodb_storage_engine_interface::{Row, Rows};

use crate::{aliaser::Aliaser, select::ordering::Ordering};

use self::{record::Record, record_schema::RecordSchema};

/// Seq of [Record](crate::Record)s.
#[derive(Clone, PartialEq, Debug)]
pub struct Records {
    schema: Arc<RecordSchema>,
    inner: Vec<Record>,
}

impl Records {
    /// Constructor
    pub(crate) fn new<IntoRecord: Into<Record>, I: IntoIterator<Item = IntoRecord>>(
        schema: Arc<RecordSchema>,
        it: I,
    ) -> Self {
        Self {
            schema,
            inner: it
                .into_iter()
                .map(|into_values| into_values.into())
                .collect(),
        }
    }

    pub(crate) fn from_rows(rows: Rows, aliaser: Aliaser) -> Self {
        let record_schema = Arc::new(RecordSchema::from_row_schema(rows.as_schema(), aliaser));
        Self::new(
            record_schema.clone(),
            rows.map(|row| Record::new(record_schema.clone(), row)),
        )
    }

    /// ref to schema
    pub(crate) fn as_schema(&self) -> &RecordSchema {
        self.schema.as_ref()
    }

    /// makes SqlValues
    pub(crate) fn into_rows(self) -> Vec<Row> {
        self.inner.into_iter().map(|record| record.row).collect()
    }

    /// Filter records that satisfy the given `condition`.
    pub(crate) fn selection(self, condition: &Expression) -> ApllodbResult<Self> {
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
                let schema = self.schema.clone();
                self.into_iter()
                    .filter_map(|r| {
                        match condition
                            .to_sql_value_for_expr_with_index(&|index| {
                                r.get_sql_value(index).map(|v| v.clone())
                            })
                            .and_then(|sql_value| sql_value.to_bool())
                        {
                            Ok(false) => None,
                            Ok(true) => Some(Ok(r)),
                            Err(e) => Some(Err(e)),
                        }
                    })
                    .collect::<ApllodbResult<Vec<Record>>>()
                    .map(|records| Self::new(schema, records))
            }
        }
    }

    /// Horizontally shrink records. Order of fields is kept between input Record and output.
    ///
    /// # Failures
    ///
    /// - [InvalidName](apllodb_shared_components::ApllodbErrorKind::InvalidName) when:
    ///   - Specified field does not exist in this record.
    pub(crate) fn projection(self, indexes: &HashSet<SchemaIndex>) -> ApllodbResult<Self> {
        let new_schema = Arc::new(self.schema.projection(indexes)?);

        let projection_positions = indexes
            .iter()
            .map(|idx| {
                let (pos, _) = self.schema.index(idx)?;
                Ok(pos)
            })
            .collect::<ApllodbResult<HashSet<RPos>>>()?;

        let new_inner: Vec<Record> = self
            .inner
            .into_iter()
            .map(|record| {
                let row = record.row.projection(&projection_positions);
                Record::new(new_schema.clone(), row)
            })
            .collect();

        Ok(Self::new(new_schema.clone(), new_inner))
    }

    /// ORDER BY
    pub(crate) fn sort(
        mut self,
        field_orderings: &[(SchemaIndex, Ordering)],
    ) -> ApllodbResult<Self> {
        assert!(!field_orderings.is_empty(), "parser should avoid this case");

        // TODO check if type in FieldIndex is PartialOrd

        self.inner.sort_by(|a_record, b_record| {
            let mut res = std::cmp::Ordering::Equal;

            for (index, ord) in field_orderings {
                let a_val = a_record
                    .get_sql_value(index)
                    .expect(format!("must be valid field: `{}`", index).as_str());
                let b_val = b_record
                    .get_sql_value(index)
                    .expect(format!("must be valid field: `{}`", index).as_str());

                match a_val.sql_compare(&b_val).unwrap_or_else(|_| {
                    panic!(
                    "two records in the same RecordIterator must have the same type for field `{}`",
                    index
                )
                }) {
                    SqlCompareResult::Eq => res = std::cmp::Ordering::Equal,
                    SqlCompareResult::LessThan => {
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
                    SqlCompareResult::GreaterThan => {
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
                    SqlCompareResult::Null => {
                        // NULL comes last, regardless of ASC/DESC
                        if let SqlValue::Null = a_val {
                            res = std::cmp::Ordering::Greater
                        } else {
                            res = std::cmp::Ordering::Less
                        }
                        break;
                    }
                    SqlCompareResult::NotEq => {
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
    /// - [InvalidName](apllodb_shared_components::ApllodbErrorKind::InvalidName) when:
    ///   - Specified field does not exist in any record.
    pub(crate) fn hash_join(
        self,
        joined_schema: Arc<RecordSchema>,
        right_records: Records,
        self_join_field: &SchemaIndex,
        right_join_field: &SchemaIndex,
    ) -> ApllodbResult<Self> {
        joined_schema.assert_all_named();

        fn helper_join_records(
            joined_schema: Arc<RecordSchema>,
            left_record: Record,
            right_record: Record,
        ) -> ApllodbResult<Record> {
            let sql_values: Vec<SqlValue> = joined_schema
                .to_aliased_field_names()
                .iter()
                .map(|joined_name| {
                    left_record
                        .helper_get_sql_value(joined_name)
                        .or_else(|| right_record.helper_get_sql_value(joined_name))
                        .expect("left or right must have AliasedFieldName in joined_schema")
                })
                .collect::<ApllodbResult<_>>()?;

            Ok(Record::new(joined_schema.clone(), Row::new(sql_values)))
        }

        // TODO Create hash table from smaller input.
        let mut hash_table = HashMap::<SqlValueHashKey, Vec<Record>>::new();

        for left_record in self {
            let left_sql_value = left_record.get_sql_value(self_join_field)?;
            hash_table
                .entry(SqlValueHashKey::from(left_sql_value))
                // FIXME Clone less. If join keys are unique, no need for clone.
                .and_modify(|records| records.push(left_record.clone()))
                .or_insert_with(|| vec![left_record]);
        }

        let mut records = Vec::<Record>::new();
        for right_record in right_records {
            let right_sql_value = right_record.get_sql_value(right_join_field)?;
            if let Some(left_records) = hash_table.get(&SqlValueHashKey::from(right_sql_value)) {
                records.append(
                    &mut left_records
                        .iter()
                        .map(|left_record| {
                            helper_join_records(
                                joined_schema.clone(),
                                left_record.clone(),
                                right_record.clone(),
                            )
                        })
                        .collect::<ApllodbResult<Vec<Record>>>()?,
                );
            }
        }

        Ok(Records::new(joined_schema.clone(), records))
    }
}

impl Iterator for Records {
    type Item = Record;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.is_empty() {
            None
        } else {
            let record = self.inner.remove(0);
            Some(record)
        }
    }
}
