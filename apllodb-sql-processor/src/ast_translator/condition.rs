use apllodb_shared_components::{ApllodbResult, CorrelationReference, Expression};
use apllodb_sql_parser::apllodb_ast;

use crate::ast_translator::AstTranslator;

impl AstTranslator {
    pub fn condition_in_select(
        ast_condition: apllodb_ast::Condition,
        correlations: &[CorrelationReference],
    ) -> ApllodbResult<Expression> {
        Self::expression_in_select(ast_condition.expression, correlations)
    }
}
