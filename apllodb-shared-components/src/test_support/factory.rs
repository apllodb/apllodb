#![allow(missing_docs)]

//! Factory methods for testing

use crate::{
    AliasName, BooleanExpression, ComparisonFunction, DatabaseName, Expression, LogicalFunction,
    NnSqlValue, SqlValue, UnaryOperator,
};
use rand::Rng;

impl DatabaseName {
    /// randomly generate a database name
    pub fn random() -> Self {
        Self::new(random_id()).unwrap()
    }
}

impl AliasName {
    pub fn factory(alias_name: &str) -> Self {
        Self::new(alias_name).unwrap()
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

pub fn random_id() -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .map(char::from)
        .filter(|c| ('a'..='z').contains(c))
        .take(10)
        .collect::<String>()
}
