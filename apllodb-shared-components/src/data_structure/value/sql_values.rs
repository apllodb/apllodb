use std::ops::Index;

use serde::{Deserialize, Serialize};

use crate::{Record, SqlValue};

/// Seq of [SqlValue](crate::SqlValue).
#[derive(Clone, PartialEq, Hash, Debug, Serialize, Deserialize, new)]
pub struct SqlValues(Vec<SqlValue>);

/// used for `INSERT INTO t (a, b, c) SELECT x, y, z FROM s;`, for example.
impl From<Record> for SqlValues {
    fn from(_r: Record) -> Self {
        todo!()
    }
}

impl Index<usize> for SqlValues {
    type Output = SqlValue;

    fn index(&self, index: usize) -> &Self::Output {
        self.0.get(index).expect("index out of range")
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
    /// If SqlValues is like this:
    ///
    /// ```text
    /// 'a', 'b', 'c', 'd'
    /// ```
    ///
    /// and `right` is:
    ///
    /// ```text
    /// 'x', 'y', 'z'
    /// ```
    ///
    /// then result is:
    ///
    /// ```text
    /// 'a', 'b', 'c', 'd', 'x', 'y', 'z'
    /// ```
    pub fn join(&mut self, mut right: Self) {
        self.0.append(&mut right.0)
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
    pub fn projection(mut self, idxs: &[usize]) -> Self {
        let mut sorted_idxs = idxs.to_vec();
        sorted_idxs.sort();

        let mut cnt_moved = 0;

        let mut new_inner_with_order: Vec<(SqlValue, usize)> = sorted_idxs
            .into_iter()
            .map(|idx| {
                let order = idxs.iter().position(|x| *x == idx).unwrap();
                let ret = (self.0.remove(idx - cnt_moved), order);
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
    use crate::{NNSqlValue, SqlValue, SqlValues};

    #[test]
    fn test_projection() {
        struct TestDatum {
            input: Vec<NNSqlValue>,
            projection: Vec<usize>,
            output: Vec<NNSqlValue>,
        }

        let test_data = vec![
            TestDatum {
                input: vec![
                    NNSqlValue::SmallInt(0),
                    NNSqlValue::SmallInt(100),
                    NNSqlValue::SmallInt(200),
                ],
                projection: vec![0, 1, 2],
                output: vec![
                    NNSqlValue::SmallInt(0),
                    NNSqlValue::SmallInt(100),
                    NNSqlValue::SmallInt(200),
                ],
            },
            TestDatum {
                input: vec![
                    NNSqlValue::SmallInt(0),
                    NNSqlValue::SmallInt(100),
                    NNSqlValue::SmallInt(200),
                ],
                projection: vec![2, 0, 1],
                output: vec![
                    NNSqlValue::SmallInt(200),
                    NNSqlValue::SmallInt(0),
                    NNSqlValue::SmallInt(100),
                ],
            },
            TestDatum {
                input: vec![
                    NNSqlValue::SmallInt(0),
                    NNSqlValue::SmallInt(100),
                    NNSqlValue::SmallInt(200),
                ],
                projection: vec![1],
                output: vec![NNSqlValue::SmallInt(100)],
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
                input.projection(&test_datum.projection),
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
