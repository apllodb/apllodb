pub(crate) mod integer_constant;

use crate::{ApllodbResult, SqlValue};
use apllodb_sql_parser::apllodb_ast;

use crate::ast_translator::AstTranslator;

impl AstTranslator {
    pub(crate) fn numeric_constant(
        ast_numeric_constant: apllodb_ast::NumericConstant,
    ) -> ApllodbResult<SqlValue> {
        let sql_value: SqlValue = match ast_numeric_constant {
            apllodb_ast::NumericConstant::IntegerConstantVariant(ic) => Self::integer_constant(ic)?,
        };
        Ok(sql_value)
    }
}
