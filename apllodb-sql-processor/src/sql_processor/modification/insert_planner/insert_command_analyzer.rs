use apllodb_shared_components::{ApllodbError, ApllodbResult, SqlValue};
use apllodb_sql_parser::apllodb_ast;
use apllodb_storage_engine_interface::{ColumnName, Row, TableName};

use crate::{ast_translator::AstTranslator, attribute::attribute_name::AttributeName, correlation::{
        aliased_correlation_name::AliasedCorrelationName, correlation_name::CorrelationName,
    }, field::{aliased_field_name::AliasedFieldName, field_name::FieldName}, records::{Records, record_schema::RecordSchema}};

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

    fn naive_afn_to_insert(&self) -> ApllodbResult<Vec<AliasedFieldName>> {
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
        let schema = self.schema_to_insert()?;

        let records: Vec<Row> = self
            .command
            .values
            .clone()
            .into_vec()
            .into_iter()
            .map(|insert_value| {
                let expressions = insert_value.expressions.into_vec();

                if schema.len() != expressions.len() {
                    ApllodbError::feature_not_supported(
                        "VALUES expressions and column names must have same length currently",
                    );
                }

                let constant_values: Vec<SqlValue> = expressions
                    .into_iter()
                    .map(|ast_expression| {
                        let expression = AstTranslator::expression_in_non_select(
                            ast_expression,
                            vec![self.table_name_to_insert()?],
                        )?;
                        expression.to_sql_value(None)
                    })
                    .collect::<ApllodbResult<_>>()?;

                let values = SqlValues::new(constant_values);
                Ok(Row::new(values))
            })
            .collect::<ApllodbResult<_>>()?;

        Ok(Records::new(schema, records))
    }
}
