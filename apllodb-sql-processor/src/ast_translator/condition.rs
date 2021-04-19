use apllodb_shared_components::{ApllodbResult, Expression};
use apllodb_sql_parser::apllodb_ast;

use crate::{
    ast_translator::AstTranslator, correlation::aliased_correlation_name::AliasedCorrelationName,
};

impl AstTranslator {
    pub fn condition_in_select(
        ast_condition: apllodb_ast::Condition,
        from_item_correlations: &[AliasedCorrelationName],
    ) -> ApllodbResult<Expression> {
        Self::expression_in_select(ast_condition.expression, from_item_correlations)
    }
}
