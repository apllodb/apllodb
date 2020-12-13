mod pk;

pub use pk::PrimaryKey;

use apllodb_shared_components::{data_structure::SqlValue, error::ApllodbResult};
use apllodb_shared_components::{
    data_structure::{ColumnReference, ColumnValue},
    traits::SqlConvertible,
};

/// Row representation used in storage engine.
/// Row, unlike `Record`, does not deal with `Expression`s.
pub trait Row {
    #[doc(hidden)]
    fn get_sql_value(&self, colref: &ColumnReference) -> ApllodbResult<&SqlValue>;

    /// Get value from column.
    ///
    /// # Failures
    ///
    /// - [UndefinedColumn](a.html) when:
    ///   - `column_name` is not in this Row.
    fn get<T: SqlConvertible>(&self, colref: &ColumnReference) -> ApllodbResult<T> {
        let sql_value = self.get_sql_value(colref)?;
        Ok(sql_value.unpack()?)
    }

    /// Append column values to this row.
    ///
    /// # Failures
    ///
    /// - [DuplicateColumn](a.html) when:
    ///   - Same ColumnReference is already in this row.
    fn append(&mut self, colvals: Vec<ColumnValue>) -> ApllodbResult<()>;
}
