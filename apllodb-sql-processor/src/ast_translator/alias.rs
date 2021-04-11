use apllodb_shared_components::ApllodbResult;
use apllodb_sql_parser::apllodb_ast;

use crate::{correlation::correlation_alias::CorrelationAlias, field::field_alias::FieldAlias};

use super::AstTranslator;

impl AstTranslator {
    pub fn correlation_alias(ast_alias: apllodb_ast::Alias) -> ApllodbResult<CorrelationAlias> {
        CorrelationAlias::new(ast_alias.0 .0)
    }

    pub fn field_alias(ast_alias: apllodb_ast::Alias) -> ApllodbResult<FieldAlias> {
        FieldAlias::new(ast_alias.0 .0)
    }
}
