/// Creates HashMap.
///
/// # Examples
///
/// ```
/// use apllodb_shared_components::hmap;
/// use std::collections::HashMap;
///
/// let h = hmap! { "k" => "v" };
///
/// let mut h2: HashMap<&str, &str> = HashMap::new();
/// h2.insert("k", "v");
///
/// assert_eq!(h, h2);
/// ```
#[macro_export]
macro_rules! hmap {
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
    };
}

/// Creates [Record](apllodb_shared_components::Record).
///
/// # Examples
///
/// ```
/// use crate::record;
/// use apllodb_shared_components::{ColumnName, ColumnReference, DataType, DataTypeKind, FieldIndex, SqlValue, TableName};
///
/// let colref = ColumnReference::new(TableName::new("t")?, ColumnName::new("c")?);
///
/// let r1 = record! { FieldIndex::from(colref.clone()) => SqlValue::pack(SqlType::integer(), &123i32)? };
/// let r2 = record! { FieldIndex::from(colref.clone()) => SqlValue::pack(SqlType::integer(), &456i32)? };
///
/// assert_ne!(r1, r2);
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
