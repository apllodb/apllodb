use super::sqlite_rowid::SqliteRowid;
use apllodb_immutable_schema_engine_domain::{
    row::pk::full_pk::revision::Revision, version::version_number::VersionNumber,
};
use apllodb_shared_components::{
    BooleanExpression, ColumnDataType, ColumnName, ComparisonFunction, Expression,
    FullFieldReference, I64LooseType, LogicalFunction, NnSqlValue, NumericComparableType, SqlType,
    SqlValue, StringComparableLoseType, TableName, UnaryOperator,
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
impl<T: ToSqlString> ToSqlString for Vec<T> {
    fn to_sql_string(&self) -> String {
        self.iter()
            .map(|v| v.to_sql_string())
            .collect::<Vec<String>>()
            .join(", ")
    }
}

impl ToSqlString for String {
    fn to_sql_string(&self) -> String {
        self.clone()
    }
}

impl ToSqlString for str {
    fn to_sql_string(&self) -> String {
        self.to_string()
    }
}

impl ToSqlString for TableName {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl ToSqlString for ColumnName {
    fn to_sql_string(&self) -> String {
        self.as_str().to_string()
    }
}

impl ToSqlString for FullFieldReference {
    fn to_sql_string(&self) -> String {
        self.to_string()
    }
}

impl ToSqlString for SqlType {
    fn to_sql_string(&self) -> String {
        match self {
            SqlType::NumericComparable(n) => match n {
                NumericComparableType::I64Loose(i) => match i {
                    I64LooseType::SmallInt => "SMALLINT",
                    I64LooseType::Integer => "INTEGER",
                    I64LooseType::BigInt => "BIGINT",
                },
            },
            SqlType::StringComparableLoose(s) => match s {
                StringComparableLoseType::Text => "TEXT",
            },
            SqlType::BooleanComparable => "BOOLEAN",
        }
        .to_string()
    }
}

impl ToSqlString for ColumnDataType {
    fn to_sql_string(&self) -> String {
        format!(
            "{} {} {}",
            self.column_name().to_sql_string(),
            self.sql_type().to_sql_string(),
            if self.nullable() { "" } else { "NOT NULL" }
        )
    }
}

impl ToSqlString for SqlValue {
    fn to_sql_string(&self) -> String {
        self.to_string()
    }
}
impl ToSqlString for NnSqlValue {
    fn to_sql_string(&self) -> String {
        self.to_string()
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

impl ToSqlString for UnaryOperator {
    fn to_sql_string(&self) -> String {
        match self {
            UnaryOperator::Minus => "-".to_string(),
        }
    }
}

impl ToSqlString for Expression {
    fn to_sql_string(&self) -> String {
        match self {
            Expression::ConstantVariant(c) => c.to_sql_string(),
            Expression::SchemaIndexVariant(ffr) => ffr.to_sql_string(),
            Expression::BooleanExpressionVariant(boolean_expr) => boolean_expr.to_sql_string(),
            Expression::UnaryOperatorVariant(uni_op, expr) => {
                format!("{} {}", uni_op.to_sql_string(), expr.to_sql_string())
            }
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
