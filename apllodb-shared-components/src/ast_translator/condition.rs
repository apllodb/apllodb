use crate::{ApllodbResult, Expression};
use apllodb_sql_parser::apllodb_ast;

use crate::ast_translator::AstTranslator;

impl AstTranslator {
    pub fn condition_in_select(
        ast_condition: apllodb_ast::Condition,
        ast_table_names: Vec<(apllodb_ast::TableName, Option<apllodb_ast::Alias>)>,
    ) -> ApllodbResult<Expression> {
        Self::expression_in_select(ast_condition.expression, ast_table_names)
    }
}
