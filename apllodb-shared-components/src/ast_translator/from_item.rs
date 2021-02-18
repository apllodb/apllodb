use crate::{ApllodbResult, FromItem, TableWithAlias};
use apllodb_sql_parser::apllodb_ast;

use crate::ast_translator::AstTranslator;

impl AstTranslator {
    /// # Failures
    ///
    /// - [DuplicateObject](apllodb_shared_components::ApllodbErrorKind::DuplicateObject) when:
    ///   - a table/alias have the same name with another table/alias
    pub fn from_item(ast_from_item: apllodb_ast::FromItem) -> ApllodbResult<FromItem> {
        Ok(FromItem::TableVariant(TableWithAlias::new(
            Self::table_name(ast_from_item.table_name)?,
            ast_from_item
                .alias
                .map_or_else(|| Ok(None), |a| Self::alias(a).map(Some))?,
        )))
    }
}
