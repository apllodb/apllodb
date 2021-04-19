mod from_item;
mod schema;

use apllodb_shared_components::{ApllodbError, ApllodbResult, Expression, SchemaIndex};
use apllodb_sql_parser::apllodb_ast;

use crate::{aliaser::Aliaser, ast_translator::AstTranslator, select::ordering::Ordering};

#[derive(Clone, Debug, new)]
pub(crate) struct SelectCommandAnalyzer {
    select_command: apllodb_ast::SelectCommand,
}

impl SelectCommandAnalyzer {
    pub(super) fn aliaser(&self) -> ApllodbResult<Aliaser> {
        let afns = self.aliased_field_names_in_projection()?;
        Ok(Aliaser::from(afns))
    }

    pub(super) fn selection_condition(&self) -> ApllodbResult<Option<Expression>> {
        if let Some(ast_condition) = &self.select_command.where_condition {
            let from_correlations = self.from_item_correlations()?;
            let expr =
                AstTranslator::condition_in_select(ast_condition.clone(), &from_correlations)?;
            Ok(Some(expr))
        } else {
            Ok(None)
        }
    }

    pub(super) fn sort_index_orderings(&self) -> ApllodbResult<Vec<(SchemaIndex, Ordering)>> {
        if let Some(ast_order_bys) = &self.select_command.order_bys {
            let from_correlations = self.from_item_correlations()?;
            let ast_order_bys = ast_order_bys.clone().into_vec();

            let index_orderings: Vec<(SchemaIndex, Ordering)> = ast_order_bys
                .into_iter()
                .map(|ast_order_by| {
                    let expression = AstTranslator::expression_in_select(
                        ast_order_by.expression,
                        &from_correlations,
                    )?;
                    let index = if let Expression::SchemaIndexVariant(idx) = expression {
                        Ok(idx)
                    } else {
                        Err(ApllodbError::feature_not_supported(
                            "ORDER BY's expression is supposed to be a SchemaIndex currently",
                        ))
                    }?;
                    let ordering = AstTranslator::ordering(ast_order_by.ordering);
                    Ok((index, ordering))
                })
                .collect::<ApllodbResult<_>>()?;

            Ok(index_orderings)
        } else {
            Ok(vec![])
        }
    }
}
