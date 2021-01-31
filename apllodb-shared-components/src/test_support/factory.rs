#![allow(missing_docs)]

//! Factory methods for testing

use crate::{
    ColumnName, ColumnReference, DatabaseName, Expression, FieldIndex, NNSqlValue, SqlValue,
    TableName, UnaryOperator,
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

impl FieldIndex {
    pub fn factory_colref(column_reference: ColumnReference) -> Self {
        Self::InColumnReference(column_reference)
    }
}

impl ColumnReference {
    pub fn factory(table_name: &str, column_name: &str) -> Self {
        Self::new(
            TableName::factory(table_name),
            ColumnName::factory(column_name),
        )
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
}

impl SqlValue {
    pub fn factory_integer(integer: i32) -> Self {
        Self::NotNull(NNSqlValue::factory_integer(integer))
    }
}

impl NNSqlValue {
    pub fn factory_integer(integer: i32) -> Self {
        Self::Integer(integer)
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
