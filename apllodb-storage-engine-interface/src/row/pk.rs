use apllodb_shared_components::{ApllodbResult, ColumnName, SqlConvertible, SqlValue};

/// Primary Key.
pub trait PrimaryKey: Eq + PartialEq {
    /// Get [SqlValue](apllodb_shared_components::SqlValue) from a PK column.
    ///
    /// # Failures
    ///
    /// - [UndefinedColumn](apllodb_shared_components::ApllodbErrorKind::UndefinedColumn) when:
    ///   - `column_name` is not in this PK.
    fn get_sql_value(&self, column_name: &ColumnName) -> ApllodbResult<&SqlValue>;

    /// Get Rust value from a PK column.
    ///
    /// # Failures
    ///
    /// - [UndefinedColumn](apllodb_shared_components::ApllodbErrorKind::UndefinedColumn) when:
    ///   - `column_name` is not in this PK.
    fn get<T: SqlConvertible>(&self, column_name: &ColumnName) -> ApllodbResult<T> {
        let sql_value = self.get_sql_value(column_name)?;
        Ok(sql_value.unpack()?)
    }
}
