use crate::{
    data_structure::reference::field_reference::FieldReference, ApllodbResult,
    UnresolvedFieldReference,
};
use apllodb_sql_parser::apllodb_ast::{self};

use crate::ast_translator::AstTranslator;

impl AstTranslator {
    pub fn column_reference(
        ast_column_reference: apllodb_ast::ColumnReference,
    ) -> ApllodbResult<UnresolvedFieldReference> {
        let corr = ast_column_reference.correlation.map_or_else(
            || Ok(None),
            |ast_corr| Self::correlation(ast_corr).map(Some),
        )?;
        let field =
            FieldReference::ColumnNameVariant(Self::column_name(ast_column_reference.column_name)?);
        Ok(UnresolvedFieldReference::new(corr, field))
    }
}
