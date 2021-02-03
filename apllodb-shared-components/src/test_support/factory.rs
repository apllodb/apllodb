#![allow(missing_docs)]

//! Factory methods for testing

use crate::{
    data_structure::reference::{
        correlation_reference::CorrelationReference, field_reference::FieldReference,
    },
    AliasName, ColumnDataType, ColumnName, DatabaseName, Expression, FieldIndex,
    FullFieldReference, NNSqlValue, SqlType, SqlValue, TableName, UnaryOperator,
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

impl FullFieldReference {
    pub fn factory(table_name: &str, column_name: &str) -> Self {
        let corr = CorrelationReference::TableNameVariant(TableName::factory(table_name));
        let field = FieldReference::ColumnNameVariant(ColumnName::factory(column_name));
        Self::new(corr, field)
    }

    pub fn with_corr_alias(mut self, correlation_alias: &str) -> Self {
        self.set_correlation_alias(AliasName::factory(correlation_alias));
        self
    }

    pub fn with_field_alias(mut self, field_alias: &str) -> Self {
        self.set_field_alias(AliasName::factory(field_alias));
        self
    }

    pub fn into_field_index(self) -> FieldIndex {
        FieldIndex::InFullFieldReference(self)
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
