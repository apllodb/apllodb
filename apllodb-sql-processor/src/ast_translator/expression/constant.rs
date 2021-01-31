pub(crate) mod numeric_constant;

use apllodb_shared_components::{ApllodbResult, SqlValue};
use apllodb_sql_parser::apllodb_ast;

use crate::ast_translator::AstTranslator;

impl AstTranslator {
    pub(crate) fn constant(ast_constant: apllodb_ast::Constant) -> ApllodbResult<SqlValue> {
        let sql_value: SqlValue = match ast_constant {
            apllodb_ast::Constant::NumericConstantVariant(nc) => Self::numeric_constant(nc)?,
        };
        Ok(sql_value)
    }
}
