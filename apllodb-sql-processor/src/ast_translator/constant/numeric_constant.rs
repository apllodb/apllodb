pub(crate) mod integer_constant;

use apllodb_shared_components::{ApllodbResult, NumericConstant};
use apllodb_sql_parser::apllodb_ast;

use crate::ast_translator::AstTranslator;

impl AstTranslator {
    pub(crate) fn numeric_constant(
        ast_numeric_constant: apllodb_ast::NumericConstant,
    ) -> ApllodbResult<NumericConstant> {
        let numeric_constant: NumericConstant = match ast_numeric_constant {
            apllodb_ast::NumericConstant::IntegerConstantVariant(ic) => {
                NumericConstant::IntegerConstantVariant(Self::integer_constant(ic)?)
            }
        };
        Ok(numeric_constant)
    }
}
