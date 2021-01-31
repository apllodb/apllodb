/// Creates [Record](apllodb_shared_components::Record).
///
/// # Examples
///
/// ```
/// use apllodb_storage_engine_interface::record;
/// use apllodb_shared_components::{ApllodbResult, ColumnName, ColumnReference, FieldIndex, NNSqlValue, SqlType, SqlValue, TableName};
///
/// fn main() -> ApllodbResult<()> {
///     let colref = ColumnReference::new(TableName::new("t")?, ColumnName::new("c")?);
///
///     let r1 = record! { FieldIndex::InColumnReference(colref.clone()) => SqlValue::NotNull(NNSqlValue::Integer(123)) };
///     let r2 = record! { FieldIndex::InColumnReference(colref.clone()) => SqlValue::NotNull(NNSqlValue::Integer(456)) };
///
///     assert_ne!(r1, r2);
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! record {
    { $($field_index:expr => $sql_value:expr),+ } => {
        {
            let mut fields = std::collections::HashMap::new();
            $(
                fields.insert($field_index, $sql_value);
            )+
            apllodb_shared_components::Record::new(fields)
        }
    };
}
