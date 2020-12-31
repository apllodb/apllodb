use crate::data_structure::data_type::{data_type_kind::DataTypeKind, DataType};

use super::{not_null_data_types, SqlConvertible};
use std::collections::HashSet;

impl SqlConvertible for String {
    fn to_sql_types() -> HashSet<DataType> {
        not_null_data_types(&[DataTypeKind::Text])
    }

    fn from_sql_types() -> HashSet<DataType> {
        not_null_data_types(&[DataTypeKind::Text])
    }
}
