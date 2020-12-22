use super::{not_null_sql_types, SqlConvertible};
use crate::data_structure::{DataType, DataTypeKind};
use std::collections::HashSet;

impl SqlConvertible for String {
    fn to_sql_types() -> HashSet<DataType> {
        use DataTypeKind::*;
        not_null_sql_types(&[Text])
    }

    fn from_sql_types() -> HashSet<DataType> {
        use DataTypeKind::*;
        not_null_sql_types(&[Text])
    }
}
