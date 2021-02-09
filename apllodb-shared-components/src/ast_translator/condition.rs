use crate::{ApllodbResult, Expression};
use apllodb_sql_parser::apllodb_ast;

use crate::ast_translator::AstTranslator;

impl AstTranslator {
    pub fn condition_in_select(
        ast_condition: apllodb_ast::Condition,
        ast_from_items: Vec<apllodb_ast::FromItem>,
    ) -> ApllodbResult<Expression> {
        Self::expression_in_select(ast_condition.expression, ast_from_items)
    }
}
