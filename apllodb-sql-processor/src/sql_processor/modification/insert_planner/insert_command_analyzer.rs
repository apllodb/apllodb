use std::{collections::HashSet, sync::Arc};

use apllodb_shared_components::{ApllodbError, ApllodbResult, Schema, SchemaIndex, SqlValue};
use apllodb_sql_parser::apllodb_ast;
use apllodb_storage_engine_interface::{ColumnName, Row, TableName};

use crate::{
    ast_translator::AstTranslator,
    attribute::attribute_name::AttributeName,
    correlation::{
        aliased_correlation_name::AliasedCorrelationName, correlation_name::CorrelationName,
    },
    field::{aliased_field_name::AliasedFieldName, field_name::FieldName},
    records::{record::Record, record_schema::RecordSchema, Records},
};

#[derive(Clone, Debug, new)]
pub(crate) struct InsertCommandAnalyzer {
    command: apllodb_ast::InsertCommand,
}

impl InsertCommandAnalyzer {
    pub(super) fn table_name_to_insert(&self) -> ApllodbResult<TableName> {
        AstTranslator::table_name(self.command.table_name.clone())
    }

    fn column_names_to_insert(&self) -> ApllodbResult<Vec<ColumnName>> {
        let ast_column_names = self.command.column_names.clone().into_vec();
        ast_column_names
            .into_iter()
            .map(AstTranslator::column_name)
            .collect()
    }

    fn naive_afn_to_insert(&self) -> ApllodbResult<HashSet<AliasedFieldName>> {
        self.column_names_to_insert()?
            .into_iter()
            .map(|cn| {
                let attr_name = AttributeName::ColumnNameVariant(cn);
                let corr_name = CorrelationName::TableNameVariant(self.table_name_to_insert()?);
                let aliased_corr_name = AliasedCorrelationName::new(corr_name, None);
                let field_name = FieldName::new(aliased_corr_name, attr_name);
                let aliased_field_name = AliasedFieldName::new(field_name, None);
                Ok(aliased_field_name)
            })
            .collect()
    }

    fn schema_to_insert(&self) -> ApllodbResult<RecordSchema> {
        Ok(RecordSchema::from(self.naive_afn_to_insert()?))
    }

    /// InsertNode takes its input as Records.
    /// Here creates Records from VALUES.
    pub(super) fn records_to_insert(&self) -> ApllodbResult<Records> {
        let schema = Arc::new(self.schema_to_insert()?);

        let records: Vec<Record> = self
            .command
            .values
            .clone()
            .into_vec()
            .into_iter()
            .map(|insert_value| {
                let ast_expressions = insert_value.expressions.as_vec();

                if schema.len() != ast_expressions.len() {
                    ApllodbError::feature_not_supported(
                        "VALUES expressions and column names must have same length currently",
                    );
                }

                // prepare enough length vec first.
                let mut constant_values: Vec<SqlValue> = vec![];
                for _ in 0..schema.len() {
                    constant_values.push(SqlValue::Null);
                }
                // insert SqlValue from VALUES following column names order.
                for (cn, ast_expr) in self
                    .command
                    .column_names
                    .as_vec()
                    .iter()
                    .zip(ast_expressions)
                {
                    let expr = AstTranslator::expression_in_non_select(
                        ast_expr.clone(),
                        vec![self.table_name_to_insert()?],
                    )?;
                    let sql_value = expr.to_sql_value_for_expr_without_index()?;

                    let (pos, _) = schema.index(&SchemaIndex::from(cn.0 .0.as_str()))?;
                    constant_values[pos.to_usize()] = sql_value;
                }

                let row = Row::new(constant_values);
                Ok(Record::new(schema.clone(), row))
            })
            .collect::<ApllodbResult<_>>()?;

        Ok(Records::new(schema, records))
    }
}
