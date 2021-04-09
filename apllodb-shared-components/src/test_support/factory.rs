#![allow(missing_docs)]

//! Factory methods for testing

use crate::{
    attribute::attribute_name::AttributeName,
    correlation::{
        aliased_correlation_name::AliasedCorrelationName, correlation_alias::CorrelationAlias,
        correlation_name::CorrelationName,
    },
    field::{aliased_field_name::AliasedFieldName, field_alias::FieldAlias, field_name::FieldName},
    record_schema::RecordSchema,
    AliasName, BooleanExpression, ColumnDataType, ColumnName, ComparisonFunction, DatabaseName,
    Expression, LogicalFunction, NnSqlValue, Records, Row, SqlType, SqlValue, SqlValues, TableName,
    UnaryOperator,
};
use rand::Rng;

impl DatabaseName {
    /// randomly generate a database name
    pub fn random() -> Self {
        Self::new(random_id()).unwrap()
    }
}

impl TableName {
    /// randomly generate a table name
    pub fn random() -> Self {
        Self::new(random_id()).unwrap()
    }
}

impl AliasedFieldName {
    pub fn factory(table_name: &str, column_name: &str) -> Self {
        Self::new(FieldName::factory(table_name, column_name), None)
    }

    pub fn with_corr_alias(self, correlation_alias: &str) -> Self {
        let field_name = self.field_name.with_corr_alias(correlation_alias);
        Self::new(field_name, None)
    }

    pub fn with_field_alias(self, field_alias: &str) -> Self {
        let alias = FieldAlias::factory(field_alias);
        Self::new(self.field_name, Some(alias))
    }
}

impl FieldName {
    pub fn factory(table_name: &str, column_name: &str) -> Self {
        Self::new(
            AliasedCorrelationName::factory(table_name),
            AttributeName::factory(column_name),
        )
    }

    pub fn with_corr_alias(self, correlation_alias: &str) -> Self {
        let aliased_correlation_name = self.aliased_correlation_name.with_alias(correlation_alias);
        Self::new(aliased_correlation_name, self.attribute_name)
    }
}

impl FieldAlias {
    pub fn factory(field_alias: &str) -> Self {
        Self::new(field_alias).unwrap()
    }
}

impl AliasedCorrelationName {
    pub fn factory(table_name: &str) -> Self {
        Self::new(CorrelationName::factory(table_name), None)
    }

    pub fn with_alias(self, correlation_alias: &str) -> Self {
        let alias = CorrelationAlias::factory(correlation_alias);
        Self::new(self.correlation_name, Some(alias))
    }
}

impl CorrelationName {
    pub fn factory(table_name: &str) -> Self {
        Self::TableNameVariant(TableName::factory(table_name))
    }
}

impl CorrelationAlias {
    pub fn factory(correlation_alias: &str) -> Self {
        Self::new(correlation_alias).unwrap()
    }
}

impl AttributeName {
    pub fn factory(column_name: &str) -> Self {
        Self::ColumnNameVariant(ColumnName::factory(column_name))
    }
}

impl TableName {
    pub fn factory(table_name: &str) -> Self {
        Self::new(table_name).unwrap()
    }
}

impl ColumnName {
    pub fn factory(column_name: &str) -> Self {
        Self::new(column_name).unwrap()
    }
}

impl AliasName {
    pub fn factory(alias_name: &str) -> Self {
        Self::new(alias_name).unwrap()
    }
}

impl ColumnDataType {
    pub fn factory(column_name: &str, sql_type: SqlType, nullable: bool) -> Self {
        Self::new(ColumnName::factory(column_name), sql_type, nullable)
    }
}
impl Expression {
    pub fn factory_null() -> Self {
        Self::ConstantVariant(SqlValue::Null)
    }

    pub fn factory_integer(integer: i32) -> Self {
        Self::ConstantVariant(SqlValue::factory_integer(integer))
    }

    pub fn factory_uni_op(unary_operator: UnaryOperator, expression: Expression) -> Self {
        Self::UnaryOperatorVariant(unary_operator, Box::new(expression))
    }

    pub fn factory_eq(left: Expression, right: Expression) -> Self {
        Self::BooleanExpressionVariant(BooleanExpression::factory_eq(left, right))
    }

    pub fn factory_and(left: BooleanExpression, right: BooleanExpression) -> Self {
        Self::BooleanExpressionVariant(BooleanExpression::LogicalFunctionVariant(
            LogicalFunction::AndVariant {
                left: Box::new(left),
                right: Box::new(right),
            },
        ))
    }
}

impl BooleanExpression {
    pub fn factory_eq(left: Expression, right: Expression) -> Self {
        BooleanExpression::ComparisonFunctionVariant(ComparisonFunction::EqualVariant {
            left: Box::new(left),
            right: Box::new(right),
        })
    }
}

impl SqlValue {
    pub fn factory_integer(integer: i32) -> Self {
        Self::NotNull(NnSqlValue::factory_integer(integer))
    }

    pub fn factory_bool(bool_: bool) -> Self {
        Self::NotNull(NnSqlValue::factory_bool(bool_))
    }
}

impl NnSqlValue {
    pub fn factory_integer(integer: i32) -> Self {
        Self::Integer(integer)
    }

    pub fn factory_bool(bool_: bool) -> Self {
        Self::Boolean(bool_)
    }
}

impl Row {
    pub fn factory(sql_values: Vec<SqlValue>) -> Self {
        Self::new(SqlValues::new(sql_values))
    }

    /// WARN: internal SqlValues might get different from RecordFieldRefSchema
    pub fn naive_join(self, right: Self) -> Self {
        let mut sql_values = self.into_values();
        for right_sql_value in right.into_values() {
            sql_values.append(right_sql_value);
        }
        Self::new(sql_values)
    }
}

impl Records {
    pub fn factory(schema: RecordSchema, records: Vec<Row>) -> Self {
        Self::new(schema, records)
    }
}

impl RecordSchema {
    pub fn factory(aliased_field_names: Vec<AliasedFieldName>) -> Self {
        Self::from(aliased_field_names)
    }

    pub fn joined(&self, right: &Self) -> Self {
        let mut left = self.to_aliased_field_names().to_vec();
        let mut right = right.to_aliased_field_names().to_vec();
        left.append(&mut right);
        Self::from(left)
    }
}

fn random_id() -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .map(char::from)
        .filter(|c| ('a'..='z').contains(c))
        .take(10)
        .collect::<String>()
}
