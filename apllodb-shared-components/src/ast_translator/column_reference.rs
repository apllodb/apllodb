use crate::{
    data_structure::reference::{
        correlation_reference::CorrelationReference, field_reference::FieldReference,
    },
    ApllodbResult, FullFieldReference,
};
use apllodb_sql_parser::apllodb_ast::{self};

use crate::ast_translator::AstTranslator;

impl AstTranslator {
    pub fn column_reference_with_table_name(
        ast_table_name: apllodb_ast::TableName,
        ast_column_name: apllodb_ast::ColumnName,
    ) -> ApllodbResult<FullFieldReference> {
        let table_name = Self::table_name(ast_table_name)?;
        let column_name = Self::column_name(ast_column_name)?;

        let correlation_reference = CorrelationReference::TableNameVariant(table_name);
        let field_reference = FieldReference::ColumnNameVariant(column_name);

        Ok(FullFieldReference::new(
            correlation_reference,
            field_reference,
        ))
    }

    // TODO column_reference_with_table_alias, ...
}
