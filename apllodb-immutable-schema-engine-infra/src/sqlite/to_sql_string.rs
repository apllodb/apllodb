use apllodb_shared_components::data_structure::{
    ColumnDataType, ColumnName, Constant, DataType, DataTypeKind, Expression, IntegerConstant,
    NumericConstant,
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

impl ToSqlString for Expression {
    fn to_sql_string(&self) -> String {
        match self {
            Expression::ConstantVariant(c) => match c {
                Constant::NumericConstantVariant(n) => match n {
                    NumericConstant::IntegerConstantVariant(IntegerConstant(i)) => format!("{}", i),
                },
            },
            Expression::ColumnNameVariant(column_name) => column_name.to_sql_string(),
        }
    }
}
