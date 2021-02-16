use crate::{ApllodbResult, Expression};
use apllodb_sql_parser::apllodb_ast;

use crate::ast_translator::AstTranslator;

impl AstTranslator {
    pub fn condition(ast_condition: apllodb_ast::Condition) -> ApllodbResult<Expression> {
        Self::expression(ast_condition.expression)
    }
}
