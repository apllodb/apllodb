use super::sqlite_rowid::SqliteRowid;
use apllodb_immutable_schema_engine_domain::{PKColumnNames, Revision, VersionNumber, row::column::non_pk_column::{NonPKColumnName, NonPKColumnDataType}};
use apllodb_shared_components::data_structure::{
    BooleanExpression, CharacterConstant, ColumnDataType, ColumnName, ComparisonFunction, Constant,
    DataType, DataTypeKind, Expression, IntegerConstant, LogicalFunction, NumericConstant,
    SqlValue, TableName, TextConstant,
};

pub(in crate::sqlite) trait ToSqlString {
    fn to_sql_string(&self) -> String;
}

impl<T: ToSqlString + ?Sized> ToSqlString for &T {
    fn to_sql_string(&self) -> String {
        (*self).to_sql_string()
    }
}

impl<T: ToSqlString> ToSqlString for [T] {
    fn to_sql_string(&self) -> String {
        self.iter()
            .map(|t| t.to_sql_string())
            .collect::<Vec<String>>()
            .join(", ")
    }
}

impl ToSqlString for String {
    fn to_sql_string(&self) -> String {
        format!("{}", self)
    }
}

impl ToSqlString for str {
    fn to_sql_string(&self) -> String {
        format!("{}", self)
    }
}

impl ToSqlString for TableName {
    fn to_sql_string(&self) -> String {
        format!("{}", self)
    }
}

impl ToSqlString for ColumnName {
    fn to_sql_string(&self) -> String {
        format!("{}", self)
    }
}

impl ToSqlString for PKColumnNames {
    fn to_sql_string(&self) -> String {
        self.column_names()
            .iter()
            .map(|cn| cn.to_sql_string())
            .collect::<Vec<String>>()
            .join(", ")
    }
}

impl ToSqlString for DataTypeKind {
    fn to_sql_string(&self) -> String {
        use DataTypeKind::*;

        match self {
            SmallInt | Integer | BigInt => "INTEGER",
            Text => "TEXT",
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

impl ToSqlString for NonPKColumnDataType {
    fn to_sql_string(&self) -> String {
        self.0.to_sql_string()
    }
}
impl ToSqlString for NonPKColumnName {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl ToSqlString for SqlValue {
    fn to_sql_string(&self) -> String {
        let constant = Constant::from(self);
        constant.to_sql_string()
    }
}

impl ToSqlString for Constant {
    fn to_sql_string(&self) -> String {
        match self {
            Constant::NumericConstantVariant(n) => match n {
                NumericConstant::IntegerConstantVariant(IntegerConstant(i)) => format!("{}", i),
            },
            Constant::CharacterConstantVariant(c) => match c {
                CharacterConstant::TextConstantVariant(TextConstant(s)) => s.clone(),
            },
        }
    }
}

impl ToSqlString for LogicalFunction {
    fn to_sql_string(&self) -> String {
        match self {
            LogicalFunction::AndVariant { left, right } => {
                format!("({} AND {})", left.to_sql_string(), right.to_sql_string())
            }
        }
    }
}
impl ToSqlString for ComparisonFunction {
    fn to_sql_string(&self) -> String {
        match self {
            ComparisonFunction::EqualVariant { left, right } => {
                format!("({} = {})", left.to_sql_string(), right.to_sql_string())
            }
        }
    }
}
impl ToSqlString for BooleanExpression {
    fn to_sql_string(&self) -> String {
        match self {
            BooleanExpression::LogicalFunctionVariant(lf) => format!("({})", lf.to_sql_string()),
            BooleanExpression::ComparisonFunctionVariant(cf) => format!("({})", cf.to_sql_string()),
        }
    }
}
impl ToSqlString for Expression {
    fn to_sql_string(&self) -> String {
        match self {
            Expression::ConstantVariant(c) => c.to_sql_string(),
            Expression::ColumnNameVariant(column_name) => column_name.to_sql_string(),
            Expression::BooleanExpressionVariant(boolean_expr) => boolean_expr.to_sql_string(),
        }
    }
}

impl ToSqlString for Revision {
    fn to_sql_string(&self) -> String {
        format!("{}", self.to_u64())
    }
}

impl ToSqlString for VersionNumber {
    fn to_sql_string(&self) -> String {
        format!("{}", self.to_u64())
    }
}

impl ToSqlString for SqliteRowid {
    fn to_sql_string(&self) -> String {
        format!("{}", self.0)
    }
}
