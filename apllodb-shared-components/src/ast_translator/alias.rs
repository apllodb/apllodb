use crate::{AliasName, ApllodbResult};
use apllodb_sql_parser::apllodb_ast;

use crate::ast_translator::AstTranslator;

impl AstTranslator {
    pub fn alias(ast_alias: apllodb_ast::Alias) -> ApllodbResult<AliasName> {
        AliasName::new(ast_alias.0 .0)
    }
}
