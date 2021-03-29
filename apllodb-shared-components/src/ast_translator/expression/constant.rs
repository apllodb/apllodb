pub(crate) mod numeric_constant;
pub(crate) mod string_constant;

use crate::{ApllodbResult, SqlValue};
use apllodb_sql_parser::apllodb_ast;

use crate::ast_translator::AstTranslator;

impl AstTranslator {
    pub(crate) fn constant(ast_constant: apllodb_ast::Constant) -> ApllodbResult<SqlValue> {
        let sql_value: SqlValue = match ast_constant {
            apllodb_ast::Constant::NullVariant => SqlValue::Null,
            apllodb_ast::Constant::NumericConstantVariant(nc) => Self::numeric_constant(nc)?,
            apllodb_ast::Constant::StringConstantVariant(sc) => Self::string_constant(sc),
        };
        Ok(sql_value)
    }
}
