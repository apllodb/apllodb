use apllodb_shared_components::{
    data_structure::{ColumnName, SqlValue},
    error::ApllodbResult,
    traits::SqlConvertible,
};
use serde::{de::DeserializeOwned, Serialize};
use std::{fmt::Debug, hash::Hash};

/// Primary Key.
pub trait PrimaryKey: Eq + PartialEq + Hash + Debug + Serialize + DeserializeOwned {
    #[doc(hidden)]
    fn get_core(&self, column_name: &ColumnName) -> ApllodbResult<&SqlValue>;

    /// Get value from a PK column.
    ///
    /// # Failures
    ///
    /// - [UndefinedColumn](a.html) when:
    ///   - `column_name` is not in this Row.
    fn get<T: SqlConvertible>(&self, column_name: &ColumnName) -> ApllodbResult<T> {
        let sql_value = self.get_core(column_name)?;
        Ok(sql_value.unpack()?)
    }
}
