use apllodb_shared_components::data_structure::{
    ColumnDataType, ColumnName, DataType, DataTypeKind,
};

pub(in crate::sqlite) trait ToSqlString {
    fn to_sql_string(&self) -> String;
}

impl ToSqlString for ColumnName {
    fn to_sql_string(&self) -> String {
        format!("{}", self)
    }
}

impl ToSqlString for DataTypeKind {
    fn to_sql_string(&self) -> String {
        use DataTypeKind::*;

        match self {
            SmallInt | Integer | BigInt => "INTEGER",
        }
        .to_string()
    }
}

impl ToSqlString for DataType {
    fn to_sql_string(&self) -> String {
        format!(
            "{}{}",
            self.kind().to_sql_string(),
            if self.nullable() { "" } else { " NOT NULL" }
        )
    }
}

impl ToSqlString for ColumnDataType {
    fn to_sql_string(&self) -> String {
        format!(
            "{} {}",
            self.column_name().to_sql_string(),
            self.data_type().to_sql_string(),
        )
    }
}
