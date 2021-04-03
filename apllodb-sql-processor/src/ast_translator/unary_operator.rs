use apllodb_shared_components::UnaryOperator;
use apllodb_sql_parser::apllodb_ast;

use crate::ast_translator::AstTranslator;

impl AstTranslator {
    pub(crate) fn unary_operator(ast_unary_operator: apllodb_ast::UnaryOperator) -> UnaryOperator {
        match ast_unary_operator {
            apllodb_ast::UnaryOperator::Minus => UnaryOperator::Minus,
        }
    }
}
