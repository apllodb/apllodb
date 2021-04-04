use apllodb_sql_parser::apllodb_ast;

use apllodb_shared_components::Ordering;

use super::AstTranslator;

impl AstTranslator {
    pub fn ordering(ast_ordering: Option<apllodb_ast::Ordering>) -> Ordering {
        match ast_ordering {
            None | Some(apllodb_ast::Ordering::AscVariant) => Ordering::Asc,
            Some(apllodb_ast::Ordering::DescVariant) => Ordering::Desc,
        }
    }
}
