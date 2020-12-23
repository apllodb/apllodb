use apllodb_shared_components::{
    data_structure::{ColumnName, SqlValue},
    error::ApllodbResult,
    traits::SqlConvertible,
};
use serde::{de::DeserializeOwned, Serialize};
use std::{fmt::Debug, hash::Hash};

/// Primary Key.
pub trait PrimaryKey: Eq + PartialEq + Hash + Debug + Serialize + DeserializeOwned {
    /// Get [SqlValue](apllodb_shared_components::data_structure::value::sql_value::SqlValue) from a PK column.
    fn get_sql_value(&self, column_name: &ColumnName) -> ApllodbResult<&SqlValue>;

    /// Get Rust value from a PK column.
    ///
    /// # Failures
    ///
    /// - [UndefinedColumn](a.html) when:
    ///   - `column_name` is not in this Row.
    fn get<T: SqlConvertible>(&self, column_name: &ColumnName) -> ApllodbResult<T> {
        let sql_value = self.get_sql_value(column_name)?;
        Ok(sql_value.unpack()?)
    }
}
