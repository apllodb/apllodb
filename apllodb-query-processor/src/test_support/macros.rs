/// Creates [Record](apllodb_shared_components::Record).
///
/// # Examples
///
/// ```
/// use crate::record;
/// use apllodb_shared_components::{DataType, DataTypeKind, SqlValue};
///
/// let r1 = record! { "field" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &123i32)? };
/// let r2 = record! { "field" => SqlValue::pack(&DataType::new(DataTypeKind::Integer, false), &456i32)? };
///
/// assert_ne!(r1, r2);
/// ```
#[macro_export]
macro_rules! record {
    { $($field_str:expr => $sql_value:expr),+ } => {
        {
            let mut fields = std::collections::HashMap::new();
            $(
                fields.insert(apllodb_shared_components::FieldIndex::from($field_str), $sql_value);
            )+
            apllodb_shared_components::Record::new(fields)
        }
    };
}
