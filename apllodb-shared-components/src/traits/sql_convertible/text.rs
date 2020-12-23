use crate::data_structure::column::{data_type::DataType, data_type_kind::DataTypeKind};

use super::{not_null_sql_types, SqlConvertible};
use std::collections::HashSet;

impl SqlConvertible for String {
    fn to_sql_types() -> HashSet<DataType> {
        not_null_sql_types(&[DataTypeKind::Text])
    }

    fn from_sql_types() -> HashSet<DataType> {
        not_null_sql_types(&[DataTypeKind::Text])
    }
}
