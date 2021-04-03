use apllodb_sql_parser::apllodb_ast;

use apllodb_shared_components::BinaryOperator;

use super::AstTranslator;

impl AstTranslator {
    pub(crate) fn binary_operator(
        ast_binary_operator: apllodb_ast::BinaryOperator,
    ) -> BinaryOperator {
        match ast_binary_operator {
            apllodb_ast::BinaryOperator::Equal => BinaryOperator::Equal,
        }
    }
}
