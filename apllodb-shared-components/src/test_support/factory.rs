#![allow(missing_docs)]

//! Factory methods for testing

use std::sync::Arc;

use crate::{
    data_structure::{
        record_iterator::record_field_ref_schema::RecordFieldRefSchema,
        reference::{correlation_reference::CorrelationReference, field_reference::FieldReference},
    },
    AliasName, BooleanExpression, ColumnDataType, ColumnName, ComparisonFunction, DatabaseName,
    Expression, FromItem, FullFieldReference, LogicalFunction, NNSqlValue, Record, RecordIterator,
    SqlType, SqlValue, SqlValues, TableName, TableWithAlias, UnaryOperator,
    UnresolvedFieldReference,
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

impl UnresolvedFieldReference {
    pub fn factory_cn(column_name: &str) -> Self {
        let field = FieldReference::ColumnNameVariant(ColumnName::factory(column_name));
        Self::new(None, field)
    }

    pub fn factory_corr_cn(correlation_reference: &str, column_name: &str) -> Self {
        let corr = CorrelationReference::factory(correlation_reference);
        let field = FieldReference::ColumnNameVariant(ColumnName::factory(column_name));
        Self::new(Some(corr), field)
    }

    pub fn with_field_alias(mut self, field_alias: &str) -> Self {
        self.set_field_alias(AliasName::factory(field_alias));
        self
    }

    pub fn resolve_naive(self) -> FullFieldReference {
        if let Some(corr) = self.as_correlation_reference() {
            let from_item = FromItem::factory(corr.as_str());
            self.resolve(Some(from_item)).unwrap()
        } else {
            self.resolve(None).unwrap()
        }
    }
}

impl CorrelationReference {
    pub fn factory(correlation_reference: &str) -> Self {
        Self::new(correlation_reference).unwrap()
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

impl FromItem {
    pub fn factory(table_name: &str) -> Self {
        FromItem::TableVariant(TableWithAlias::new(TableName::factory(table_name), None))
    }

    pub fn factory_with_corr_alias(table_name: &str, correlation_alias: &str) -> Self {
        FromItem::TableVariant(TableWithAlias::new(
            TableName::factory(table_name),
            Some(AliasName::factory(correlation_alias)),
        ))
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
        Self::NotNull(NNSqlValue::factory_integer(integer))
    }
}

impl NNSqlValue {
    pub fn factory_integer(integer: i32) -> Self {
        Self::Integer(integer)
    }
}

impl Record {
    pub fn factory(fields: Vec<(FullFieldReference, SqlValue)>) -> Self {
        let ffrs: Vec<FullFieldReference> = fields.iter().map(|f| f.0.clone()).collect();
        let sql_values: Vec<SqlValue> = fields.into_iter().map(|f| f.1).collect();

        let schema = RecordFieldRefSchema::new(ffrs);

        Self::new(Arc::new(schema), SqlValues::new(sql_values))
    }

    pub fn as_column_names(&self) -> Vec<ColumnName> {
        self.schema()
            .as_full_field_references()
            .iter()
            .map(|ffr| ffr.as_column_name())
            .cloned()
            .collect()
    }
}

impl RecordIterator {
    pub fn factory(schema: RecordFieldRefSchema, records: Vec<Record>) -> Self {
        Self::new(schema, records)
    }
}

impl RecordFieldRefSchema {
    pub fn factory(full_field_references: Vec<FullFieldReference>) -> Self {
        Self::new(full_field_references)
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
