mod pk;

pub use pk::PrimaryKey;

use apllodb_shared_components::{data_structure::ColumnReference, traits::SqlConvertible};
use apllodb_shared_components::{data_structure::SqlValue, error::ApllodbResult};

/// Row representation used in storage engine.
/// Row, unlike `Record`, does not deal with `Expression`s.
pub trait Row {
    #[doc(hidden)]
    fn get_core(&self, colref: &ColumnReference) -> ApllodbResult<&SqlValue>;

    /// Get value from column.
    ///
    /// # Failures
    ///
    /// - [UndefinedColumn](a.html) when:
    ///   - `column_name` is not in this Row.
    fn get<T: SqlConvertible>(&self, colref: &ColumnReference) -> ApllodbResult<T> {
        let sql_value = self.get_core(colref)?;
        Ok(sql_value.unpack()?)
    }
}
