use crate::{ApllodbResult, CorrelationReference, TableName};
use apllodb_sql_parser::apllodb_ast;

use crate::ast_translator::AstTranslator;

impl AstTranslator {
    pub fn correlation(
        ast_correlation: apllodb_ast::Correlation,
    ) -> ApllodbResult<CorrelationReference> {
        Ok(CorrelationReference::TableNameVariant(TableName::new(
            ast_correlation.0 .0,
        )?))
    }
}
