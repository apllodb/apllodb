use apllodb_shared_components::{ApllodbResult, TableName};
use apllodb_sql_parser::apllodb_ast;

use crate::ast_translator::AstTranslator;

impl AstTranslator {
    pub fn table_name(ast_table_name: apllodb_ast::TableName) -> ApllodbResult<TableName> {
        TableName::new(ast_table_name.0 .0)
    }
}
