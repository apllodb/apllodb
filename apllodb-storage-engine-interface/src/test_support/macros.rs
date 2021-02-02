/// Creates [Record](apllodb_shared_components::Record).
///
/// # Examples
///
/// ```
/// use apllodb_storage_engine_interface::record;
/// use apllodb_shared_components::*;
///
/// fn main() -> ApllodbResult<()> {
///     let ffr = FullFieldReference::new(
///         CorrelationReference::TableNameVariant(TableName::new("t")?),
///         FieldReference::ColumnNameVariant(ColumnName::new("c")?),
///     );
///
///     let r1 = record! { ffr.clone() => SqlValue::NotNull(NNSqlValue::Integer(123)) };
///     let r2 = record! { ffr.clone() => SqlValue::NotNull(NNSqlValue::Integer(456)) };
///
///     assert_ne!(r1, r2);
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! record {
    { $($full_field_reference:expr => $sql_value:expr),+ } => {
        {
            let mut fields = std::collections::HashMap::new();
            $(
                fields.insert($full_field_reference, $sql_value);
            )+
            apllodb_shared_components::Record::new(fields)
        }
    };
}
