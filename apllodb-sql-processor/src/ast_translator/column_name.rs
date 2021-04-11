use apllodb_shared_components::ApllodbResult;
use apllodb_sql_parser::apllodb_ast;

use crate::ast_translator::AstTranslator;

impl AstTranslator {
    pub fn column_name(ast_column_name: apllodb_ast::ColumnName) -> ApllodbResult<ColumnName> {
        ColumnName::new(ast_column_name.0 .0)
    }
}
