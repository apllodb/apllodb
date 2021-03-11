use apllodb_sql_parser::apllodb_ast;

use crate::{ApllodbResult, AstTranslator, CorrelationReference};

impl AstTranslator {
    pub fn from_item(
        ast_from_item: apllodb_ast::FromItem,
    ) -> ApllodbResult<Vec<CorrelationReference>> {
        let table_name = Self::table_name(ast_from_item.table_name)?;
        let corr_ref = match ast_from_item.alias {
            None => CorrelationReference::TableNameVariant(table_name),
            Some(alias) => CorrelationReference::TableAliasVariant {
                table_name,
                alias_name: Self::alias(alias)?,
            },
        };
        Ok(vec![corr_ref])
    }
}
