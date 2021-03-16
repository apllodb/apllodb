use crate::{ApllodbResult, NNSqlValue, SqlValue};
use apllodb_sql_parser::apllodb_ast;

use crate::ast_translator::AstTranslator;

impl AstTranslator {
    pub(crate) fn string_constant(
        ast_string_constant: apllodb_ast::StringConstant,
    ) -> ApllodbResult<SqlValue> {
        Ok(SqlValue::NotNull(NNSqlValue::Text(ast_string_constant.0)))
    }
}