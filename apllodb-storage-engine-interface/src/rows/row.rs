use apllodb_shared_components::{ApllodbResult, RPos, SqlConvertible, SqlValue};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, ops::Index};

/// Primitive row representation used in storage engines and query processor
///
/// Clients do not directly use this struct but does [apllodb-server::Record](apllodb-server::Record) instead.
///
/// Row is meant to be read-only data, created while SELECT.
#[derive(Clone, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct Row {
    values: Vec<SqlValue>,
}

impl Index<RPos> for Row {
    type Output = SqlValue;

    fn index(&self, pos: RPos) -> &Self::Output {
        self.values.get(pos.to_usize()).expect("index out of range")
    }
}

impl Row {
    /// Constructor
    pub fn new(values: Vec<SqlValue>) -> Self {
        Self { values }
    }

    /// Get Rust value from record's field.
    ///
    /// Returns `None` if matching [SqlValue](crate::SqlValue) is NULL.
    ///
    /// # Failures
    ///
    /// - [InvalidName](crate::ApllodbErrorKind::InvalidName) when:
    ///   - Specified field does not exist in this record.
    /// - Errors from [SqlValue::unpack()](x.html).
    pub fn get<T: SqlConvertible>(&self, pos: RPos) -> ApllodbResult<Option<T>> {
        let sql_value = self.get_sql_value(pos)?;
        let ret = match sql_value {
            SqlValue::Null => None,
            SqlValue::NotNull(nn) => Some(nn.unpack()?),
        };
        Ok(ret)
    }

    /// Get [SqlValue](crate::SqlValue) from record's field.
    ///
    /// # Failures
    ///
    /// - [InvalidName](crate::ApllodbErrorKind::InvalidName) when:
    ///   - Specified field does not exist in this record.
    pub fn get_sql_value(&self, pos: RPos) -> ApllodbResult<&SqlValue> {
        let sql_value = self.index(pos);
        Ok(sql_value)
    }

    /// add SqlValue to list
    pub fn append(&mut self, sql_value: SqlValue) {
        self.values.push(sql_value)
    }

    /// extract SqlValue and remove from list
    pub fn remove(&mut self, pos: RPos) -> SqlValue {
        self.values.remove(pos.to_usize())
    }

    /// Get raw representation
    pub fn into_values(self) -> Vec<SqlValue> {
        self.values
    }

    /// If SqlValues is like this:
    ///
    /// ```text
    /// 'a', 'b', 'c', 'd'
    /// ```
    ///
    /// and `positions = (3, 0)`, then result is:
    ///
    /// ```text
    /// 'a', 'd'
    /// ```
    ///
    /// Note that order of columns are kept.
    pub fn projection(mut self, positions: &HashSet<RPos>) -> Self {
        let mut sorted_positions: Vec<RPos> = positions.iter().cloned().collect();
        sorted_positions.sort_unstable();

        // removed inner shrinks. this adjusts index.
        let mut cnt_moved = 0;

        let new_values: Vec<SqlValue> = sorted_positions
            .into_iter()
            .map(|pos| {
                let v = self.remove(RPos::new(pos.to_usize() - cnt_moved));
                cnt_moved += 1;
                v
            })
            .collect();

        Self::new(new_values)
    }
}
