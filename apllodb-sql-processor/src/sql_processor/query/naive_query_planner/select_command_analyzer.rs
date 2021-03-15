use std::collections::HashSet;

use apllodb_shared_components::{
    ApllodbResult, AstTranslator, CorrelationReference, Expression, FullFieldReference, Ordering,
    RecordFieldRefSchema,
};
use apllodb_sql_parser::apllodb_ast;

#[derive(Clone, Debug, new)]
pub(crate) struct SelectCommandAnalyzer {
    select_command: apllodb_ast::SelectCommand,
}

impl SelectCommandAnalyzer {
    /// including all fields used during a SELECT execution
    pub(super) fn widest_schema(&self) -> ApllodbResult<RecordFieldRefSchema> {
        let mut widest_ffrs = HashSet::<FullFieldReference>::new();

        for ffr in self.ffrs_in_selection()? {
            widest_ffrs.insert(ffr);
        }
        for ffr in self.ffrs_in_sort()? {
            widest_ffrs.insert(ffr);
        }
        for ffr in self.projection_ffrs()? {
            widest_ffrs.insert(ffr);
        }

        Ok(RecordFieldRefSchema::new(widest_ffrs.into_iter().collect()))
    }

    pub(super) fn from_item_correlation_references(
        &self,
    ) -> ApllodbResult<Vec<CorrelationReference>> {
        let ast_from_item = self
            .select_command
            .from_item
            .as_ref()
            .expect("currently SELECT w/o FROM is unimplemented")
            .clone();
        AstTranslator::from_item(ast_from_item)
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
                    let ffr = if let Expression::FullFieldReferenceVariant(ffr) = expression {
                        ffr
                    } else {
                        unimplemented!(
                            "ORDER BY's expression is supposed to be a field name currently"
                        );
                    };
                    let ordering = AstTranslator::ordering(ast_order_by.ordering);
                    Ok((ffr, ordering))
                })
                .collect::<ApllodbResult<_>>()?;

            Ok(ffr_orderings)
        } else {
            Ok(vec![])
        }
    }

    pub(super) fn projection_ffrs(&self) -> ApllodbResult<Vec<FullFieldReference>> {
        let from_correlations = self.from_item_correlation_references()?;
        let ast_select_fields = self.select_command.select_fields.as_vec().clone();

        ast_select_fields
            .iter()
            .map(|select_field| Self::select_field_into_ffr(select_field, &from_correlations))
            .collect::<ApllodbResult<_>>()
    }

    fn ffrs_in_selection(&self) -> ApllodbResult<HashSet<FullFieldReference>> {
        if let Some(ast_condition) = &self.select_command.where_condition {
            let from_correlations = self.from_item_correlation_references()?;
            let expression =
                AstTranslator::condition_in_select(ast_condition.clone(), &from_correlations)?;
            let ffrs = expression.to_full_field_references();
            Ok(ffrs.into_iter().collect())
        } else {
            Ok(HashSet::new())
        }
    }
    fn ffrs_in_sort(&self) -> ApllodbResult<HashSet<FullFieldReference>> {
        let ffr_orderings = self.sort_ffr_orderings()?;
        Ok(ffr_orderings.into_iter().map(|(ffr, _)| ffr).collect())
    }

    fn select_field_into_ffr(
        ast_select_field: &apllodb_ast::SelectField,
        from_correlations: &[CorrelationReference],
    ) -> ApllodbResult<FullFieldReference> {
        match &ast_select_field.expression {
            apllodb_ast::Expression::ConstantVariant(_) => {
                unimplemented!();
            }
            apllodb_ast::Expression::ColumnReferenceVariant(ast_colref) => {
                AstTranslator::select_field_column_reference(
                    ast_colref.clone(),
                    ast_select_field.alias.clone(),
                    from_correlations,
                )
            }
            apllodb_ast::Expression::UnaryOperatorVariant(_, _)
            | apllodb_ast::Expression::BinaryOperatorVariant(_, _, _) => {
                // TODO このレイヤーで計算しちゃいたい
                unimplemented!();
            }
        }
    }
}
