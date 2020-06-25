mod pk;

pub use pk::PrimaryKey;

use apllodb_shared_components::traits::SqlConvertible;
use apllodb_shared_components::{
    data_structure::{ColumnName, SqlValue},
    error::ApllodbResult,
};

/// Row representation used in storage engine.
/// Row, unlike `Record`, does not deal with `Expression`s.
pub trait Row {
    /// Primary Key.
    type PK: PrimaryKey;

    /// Primary Key.
    fn pk(&self) -> &Self::PK;

    #[doc(hidden)]
    fn get_core(&self, column_name: &ColumnName) -> ApllodbResult<&SqlValue>;

    /// Get value from column.
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
