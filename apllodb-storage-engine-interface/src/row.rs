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
    /// Retrieve (and remove) an SqlValue from this row.
    fn get_sql_value(&mut self, colref: &ColumnReference) -> ApllodbResult<SqlValue>;

    /// Retrieve (and remove) an SqlValue from this row and return it as Rust type.
    ///
    /// # Failures
    ///
    /// - [UndefinedColumn](a.html) when:
    ///   - `column_name` is not in this Row.
    fn get<T: SqlConvertible>(&mut self, colref: &ColumnReference) -> ApllodbResult<T> {
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
