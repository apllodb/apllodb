use crate::FromItem;
use apllodb_sql_parser::apllodb_ast;

use crate::ast_translator::AstTranslator;

impl AstTranslator {
    pub fn from_item(ast_from_item: apllodb_ast::FromItem) -> ApllodbResult<FromItem> {
        match ast_from_item {
            apllodb_ast::FromItem::TableNameVariant { table_name, alias } => {
                Ok(FromItem::TableNameVariant {
                    table_name: Self::table_name(table_name)?,
                    alias: alias.map_or_else(|| Ok(None), |a| Self::alias(a).map(Some))?,
                })
            }
            apllodb_ast::FromItem::JoinVariant {
                join_type,
                left,
                right,
                on,
            } => Ok(FromItem::JoinVariant {
                join_type: Self::join_type(join_type),
                left: Self::from_item(*left),
                right: Self::from_item(*right),
                on: Self::expression_in_select(ast_expression, ast_table_names),
            }),
        }
    }
}
