use apllodb_shared_components::{
    ApllodbError, ApllodbResult, ColumnName, CorrelationReference, FieldReference,
    FullFieldReference, Row, RecordFieldRefSchema, Records, SqlValue, SqlValues, TableName,
};
use apllodb_sql_parser::apllodb_ast;

use crate::ast_translator::AstTranslator;

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

    fn ffrs_to_insert(&self) -> ApllodbResult<Vec<FullFieldReference>> {
        self.column_names_to_insert()?
            .into_iter()
            .map(|cn| {
                Ok(FullFieldReference::new(
                    CorrelationReference::TableNameVariant(self.table_name_to_insert()?),
                    FieldReference::ColumnNameVariant(cn),
                ))
            })
            .collect()
    }

    fn schema_to_insert(&self) -> ApllodbResult<RecordFieldRefSchema> {
        Ok(RecordFieldRefSchema::new(self.ffrs_to_insert()?))
    }

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
