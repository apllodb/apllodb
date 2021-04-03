use apllodb_shared_components::{ApllodbResult, DatabaseName};
use apllodb_sql_parser::apllodb_ast;

use crate::ast_translator::AstTranslator;

impl AstTranslator {
    pub fn database_name(
        ast_database_name: apllodb_ast::DatabaseName,
    ) -> ApllodbResult<DatabaseName> {
        DatabaseName::new(ast_database_name.0 .0)
    }
}
