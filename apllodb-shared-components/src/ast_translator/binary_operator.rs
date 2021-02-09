use apllodb_sql_parser::apllodb_ast;

use crate::{ast_translator::AstTranslator, data_structure::expression::operator::BinaryOperator};

impl AstTranslator {
    pub(crate) fn binary_operator(
        ast_binary_operator: apllodb_ast::BinaryOperator,
    ) -> BinaryOperator {
        match ast_binary_operator {
            apllodb_ast::BinaryOperator::Equal => BinaryOperator::Equal,
        }
    }
}
