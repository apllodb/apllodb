pub(crate) mod numeric_constant;

use apllodb_shared_components::{ApllodbResult, Constant};
use apllodb_sql_parser::apllodb_ast;

use crate::ast_translator::AstTranslator;

impl AstTranslator {
    #[allow(dead_code)]
    pub(crate) fn constant(ast_constant: apllodb_ast::Constant) -> ApllodbResult<Constant> {
        let constant: Constant = match ast_constant {
            apllodb_ast::Constant::NumericConstantVariant(nc) => {
                Constant::NumericConstantVariant(Self::numeric_constant(nc)?)
            }
        };
        Ok(constant)
    }
}
