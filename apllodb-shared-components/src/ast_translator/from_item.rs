use apllodb_sql_parser::apllodb_ast;

use crate::{ApllodbResult, AstTranslator, CorrelationReference};

impl AstTranslator {
    pub fn from_item(
        ast_from_item: apllodb_ast::FromItem,
    ) -> ApllodbResult<Vec<CorrelationReference>> {
        match ast_from_item {
            apllodb_ast::FromItem::TableNameVariant { table_name, alias } => {
                let table_name = Self::table_name(table_name)?;
                let corr_ref = match alias {
                    None => CorrelationReference::TableNameVariant(table_name),
                    Some(alias) => CorrelationReference::TableAliasVariant {
                        table_name,
                        alias_name: Self::alias(alias)?,
                    },
                };
                Ok(vec![corr_ref])
            }
            apllodb_ast::FromItem::JoinVariant { left, right, .. } => {
                let mut left_corr_ref = Self::from_item(*left)?;
                let mut right_corr_ref = Self::from_item(*left)?;
                left_corr_ref.append(&mut right_corr_ref);
                Ok(left_corr_ref)
            }
        }
    }
}
