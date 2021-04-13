mod from_item;
mod schema;

use std::collections::HashSet;

use apllodb_shared_components::{ApllodbError, ApllodbResult, Expression};
use apllodb_sql_parser::apllodb_ast;

use crate::{
    ast_translator::AstTranslator,
    correlation::aliased_correlation_name::AliasedCorrelationName,
    field::{aliased_field_name::AliasedFieldName, field_alias::FieldAlias},
    select::ordering::Ordering,
};

#[derive(Clone, Debug, new)]
pub(crate) struct SelectCommandAnalyzer {
    select_command: apllodb_ast::SelectCommand,
}

impl SelectCommandAnalyzer {
    /// including all fields used during a SELECT execution
    pub(super) fn widest_schema(&self) -> ApllodbResult<RecordFieldRefSchema> {
        let mut widest_ffrs = HashSet::<FullFieldReference>::new();

        for ffr in self.ffrs_in_join()? {
            widest_ffrs.insert(ffr);
        }
        for ffr in self.ffrs_in_selection()? {
            widest_ffrs.insert(ffr);
        }
        for ffr in self.ffrs_in_sort()? {
            widest_ffrs.insert(ffr);
        }
        for ffr in self.aliased_field_names_in_projection()? {
            widest_ffrs.insert(ffr);
        }

        Ok(RecordFieldRefSchema::new(widest_ffrs.into_iter().collect()))
    }

    pub(super) fn selection_condition(&self) -> ApllodbResult<Option<Expression>> {
        if let Some(ast_condition) = &self.select_command.where_condition {
            let from_correlations = self.from_item_correlation_references()?;
            let expr =
                AstTranslator::condition_in_select(ast_condition.clone(), &from_correlations)?;
            Ok(Some(expr))
        } else {
            Ok(None)
        }
    }

    pub(super) fn sort_ffr_orderings(&self) -> ApllodbResult<Vec<(FullFieldReference, Ordering)>> {
        if let Some(ast_order_bys) = &self.select_command.order_bys {
            let from_correlations = self.from_item_correlation_references()?;
            let ast_order_bys = ast_order_bys.clone().into_vec();

            let ffr_orderings: Vec<(FullFieldReference, Ordering)> = ast_order_bys
                .into_iter()
                .map(|ast_order_by| {
                    let expression = AstTranslator::expression_in_select(
                        ast_order_by.expression,
                        &from_correlations,
                    )?;
                    let ffr = if let Expression::SchemaIndexVariant(ffr) = expression {
                        Ok(ffr)
                    } else {
                        Err(ApllodbError::feature_not_supported(
                            "ORDER BY's expression is supposed to be a field name currently",
                        ))
                    }?;
                    let ordering = AstTranslator::ordering(ast_order_by.ordering);
                    Ok((ffr, ordering))
                })
                .collect::<ApllodbResult<_>>()?;

            Ok(ffr_orderings)
        } else {
            Ok(vec![])
        }
    }

    pub(super) fn aliased_field_names_in_projection(&self) -> ApllodbResult<Vec<AliasedFieldName>> {
        let from_item_correlations = self.from_item_correlation_references()?;
        let ast_select_fields = self.select_command.select_fields.as_vec().clone();

        ast_select_fields
            .iter()
            .map(|select_field| {
                Self::select_field_into_aliased_field_name(select_field, &from_item_correlations)
            })
            .collect::<ApllodbResult<_>>()
    }

    fn ffrs_in_join(&self) -> ApllodbResult<Vec<FullFieldReference>> {
        self.from_item_full_field_references()
            .map(|ffrs| ffrs.into_iter().collect())
    }
    fn ffrs_in_selection(&self) -> ApllodbResult<Vec<FullFieldReference>> {
        if let Some(ast_condition) = &self.select_command.where_condition {
            let from_correlations = self.from_item_correlation_references()?;
            let expression =
                AstTranslator::condition_in_select(ast_condition.clone(), &from_correlations)?;
            let ffrs = expression.to_schema_indexes();
            Ok(ffrs.into_iter().collect())
        } else {
            Ok(vec![])
        }
    }
    fn ffrs_in_sort(&self) -> ApllodbResult<Vec<FullFieldReference>> {
        let ffr_orderings = self.sort_ffr_orderings()?;
        Ok(ffr_orderings.into_iter().map(|(ffr, _)| ffr).collect())
    }

    fn select_field_into_aliased_field_name(
        ast_select_field: &apllodb_ast::SelectField,
        from_item_correlations: &[AliasedCorrelationName],
    ) -> ApllodbResult<AliasedFieldName> {
        let expression = AstTranslator::expression_in_select(
            ast_select_field.expression,
            from_item_correlations,
        )?;

        match expression {
            Expression::ConstantVariant(_) => Err(ApllodbError::feature_not_supported(
                "constant in select field is not supported currently",
            )),
            Expression::UnaryOperatorVariant(_, _) | Expression::BooleanExpressionVariant(_) => {
                // TODO このレイヤーで計算しちゃいたい
                Err(ApllodbError::feature_not_supported(
                    "unary/binary operation in select field is not supported currently",
                ))
            }
            Expression::SchemaIndexVariant(index) => {
                let field_name = Self::field_name(&index, from_item_correlations)?;
                let afn = if let Some(a) = ast_select_field.alias {
                    AliasedFieldName::new(field_name, Some(FieldAlias::new(a.0 .0)?))
                } else {
                    AliasedFieldName::new(field_name, None)
                };
                Ok(afn)
            }
        }
    }
}
