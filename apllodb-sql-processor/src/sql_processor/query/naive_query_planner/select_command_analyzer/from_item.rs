use apllodb_shared_components::{ApllodbResult, AstTranslator, CorrelationReference};
use apllodb_sql_parser::apllodb_ast;

use super::SelectCommandAnalyzer;

impl SelectCommandAnalyzer {
    pub(in super::super) fn from_item_correlation_references(
        &self,
    ) -> ApllodbResult<Vec<CorrelationReference>> {
        let ast_from_item = self.select_command.from_item.as_ref();

        if let Some(ast_from_item) = ast_from_item {
            Self::ast_from_item_into_correlation_references(ast_from_item)
        } else {
            Ok(vec![])
        }
    }

    fn ast_from_item_into_correlation_references(
        ast_from_item: &apllodb_ast::FromItem,
    ) -> ApllodbResult<Vec<CorrelationReference>> {
        match ast_from_item {
            apllodb_ast::FromItem::TableNameVariant { table_name, alias } => {
                let table_name = AstTranslator::table_name(table_name.clone())?;
                let corr_ref = match alias {
                    None => CorrelationReference::TableNameVariant(table_name),
                    Some(alias) => CorrelationReference::TableAliasVariant {
                        table_name,
                        alias_name: AstTranslator::alias(alias.clone())?,
                    },
                };
                Ok(vec![corr_ref])
            }
            apllodb_ast::FromItem::JoinVariant { left, right, .. } => {
                let mut left_corr_ref = Self::ast_from_item_into_correlation_references(left)?;
                let mut right_corr_ref = Self::ast_from_item_into_correlation_references(right)?;
                left_corr_ref.append(&mut right_corr_ref);
                Ok(left_corr_ref)
            }
        }
    }
}
