use std::ops::Index;

use serde::{Deserialize, Serialize};

use crate::{RecordPos, Row, SqlValue};

/// Seq of [SqlValue](crate::SqlValue).
#[derive(Clone, PartialEq, Hash, Debug, Serialize, Deserialize, new)]
pub struct SqlValues(Vec<SqlValue>);

/// used for `INSERT INTO t (a, b, c) SELECT x, y, z FROM s;`, for example.
impl From<Row> for SqlValues {
    fn from(r: Row) -> Self {
        r.into_values()
    }
}

impl Index<RecordPos> for SqlValues {
    type Output = SqlValue;

    fn index(&self, pos: RecordPos) -> &Self::Output {
        self.0.get(pos.to_usize()).expect("index out of range")
    }
}

impl Iterator for SqlValues {
    type Item = SqlValue;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.0.remove(0))
        }
    }
}

impl SqlValues {
    /// add SqlValue to list
    pub fn append(&mut self, sql_value: SqlValue) {
        self.0.push(sql_value)
    }

    /// get ref to SqlValue
    pub fn get(&self, pos: RecordPos) -> &SqlValue {
        self.0.index(pos.to_usize())
    }

    /// extract SqlValue and remove from list
    pub fn remove(&mut self, pos: RecordPos) -> SqlValue {
        self.0.remove(pos.to_usize())
    }

    /// If SqlValues is like this:
    ///
    /// ```text
    /// 'a', 'b', 'c', 'd'
    /// ```
    ///
    /// and `idxs = [3, 0]`, then result is:
    ///
    /// ```text
    /// 'd', 'a'
    /// ```
    pub fn projection(mut self, positions: &[RecordPos]) -> Self {
        let mut sorted_idxs = positions.to_vec();
        sorted_idxs.sort_unstable();

        let mut cnt_moved = 0;

        let mut new_inner_with_order: Vec<(SqlValue, usize)> = sorted_idxs
            .into_iter()
            .map(|pos| {
                let order = positions.iter().position(|x| *x == pos).unwrap();
                let ret = (self.0.remove(pos.to_usize() - cnt_moved), order);
                cnt_moved += 1;
                ret
            })
            .collect();

        let new_inner: Vec<SqlValue> = {
            new_inner_with_order.sort_by_key(|v| v.1);
            new_inner_with_order.into_iter().map(|v| v.0).collect()
        };
        Self(new_inner)
    }
}

#[cfg(test)]
mod tests {
    use crate::{NnSqlValue, RecordPos, SqlValue, SqlValues};

    #[test]
    fn test_projection() {
        struct TestDatum {
            input: Vec<NnSqlValue>,
            projection: Vec<usize>,
            output: Vec<NnSqlValue>,
        }

        let test_data = vec![
            TestDatum {
                input: vec![
                    NnSqlValue::SmallInt(0),
                    NnSqlValue::SmallInt(100),
                    NnSqlValue::SmallInt(200),
                ],
                projection: vec![0, 1, 2],
                output: vec![
                    NnSqlValue::SmallInt(0),
                    NnSqlValue::SmallInt(100),
                    NnSqlValue::SmallInt(200),
                ],
            },
            TestDatum {
                input: vec![
                    NnSqlValue::SmallInt(0),
                    NnSqlValue::SmallInt(100),
                    NnSqlValue::SmallInt(200),
                ],
                projection: vec![2, 0, 1],
                output: vec![
                    NnSqlValue::SmallInt(200),
                    NnSqlValue::SmallInt(0),
                    NnSqlValue::SmallInt(100),
                ],
            },
            TestDatum {
                input: vec![
                    NnSqlValue::SmallInt(0),
                    NnSqlValue::SmallInt(100),
                    NnSqlValue::SmallInt(200),
                ],
                projection: vec![1],
                output: vec![NnSqlValue::SmallInt(100)],
            },
        ];

        for test_datum in test_data {
            let input = SqlValues::new(
                test_datum
                    .input
                    .into_iter()
                    .map(SqlValue::NotNull)
                    .collect(),
            );
            assert_eq!(
                input.projection(
                    test_datum
                        .projection
                        .into_iter()
                        .map(RecordPos::new)
                        .collect::<Vec<_>>()
                        .as_slice(),
                ),
                SqlValues::new(
                    test_datum
                        .output
                        .into_iter()
                        .map(SqlValue::NotNull)
                        .collect(),
                )
            );
        }
    }
}
