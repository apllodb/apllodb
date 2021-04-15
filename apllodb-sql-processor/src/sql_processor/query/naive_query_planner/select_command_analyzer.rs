mod from_item;
mod schema;

use std::collections::HashSet;

use apllodb_shared_components::{ApllodbError, ApllodbResult, Expression, SchemaIndex};
use apllodb_sql_parser::apllodb_ast;

use crate::{
    ast_translator::AstTranslator,
    correlation::aliased_correlation_name::AliasedCorrelationName,
    field::{aliased_field_name::AliasedFieldName, field_alias::FieldAlias, field_name::FieldName},
    records::record_schema::RecordSchema,
    select::ordering::Ordering,
};

#[derive(Clone, Debug, new)]
pub(crate) struct SelectCommandAnalyzer {
    select_command: apllodb_ast::SelectCommand,
}

impl SelectCommandAnalyzer {
    /// including all fields used during a SELECT execution
    pub(super) fn widest_schema(&self) -> ApllodbResult<RecordSchema> {
        let mut indexes = HashSet::<SchemaIndex>::new();

        for idx in self.join_indexes()? {
            indexes.insert(idx);
        }
        for idx in self.selection_indexes()? {
            indexes.insert(idx);
        }
        for idx in self.sort_indexes()? {
            indexes.insert(idx);
        }

        let mut widest_afns = Vec::<AliasedFieldName>::new();
        // insert AliasedFieldNames first.
        for afn in self.aliased_field_names_in_projection()? {
            widest_afns.push(afn);
        }
        // insert unaliased FieldNames if widest_afns do not contain aliased version.
        let from_item_correlations = self.from_item_correlations()?;
        for idx in indexes {
            let field_name = Self::field_name(&idx, &from_item_correlations)?;
            if !widest_afns.iter().any(|afn| afn.field_name == field_name) {
                let afn = AliasedFieldName::new(field_name, None);
                widest_afns.push(afn);
            }
        }

        Ok(RecordSchema::from(
            widest_afns.into_iter().collect::<HashSet<_>>(),
        ))
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

    pub(super) fn sort_ffr_orderings(&self) -> ApllodbResult<Vec<(FullFieldReference, Ordering)>> {
        if let Some(ast_order_bys) = &self.select_command.order_bys {
            let from_correlations = self.from_item_correlations()?;
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
        let from_item_correlations = self.from_item_correlations()?;
        let ast_select_fields = self.select_command.select_fields.as_vec().clone();

        ast_select_fields
            .iter()
            .map(|select_field| {
                Self::select_field_into_aliased_field_name(select_field, &from_item_correlations)
            })
            .collect::<ApllodbResult<_>>()
    }

    fn join_indexes(&self) -> ApllodbResult<Vec<SchemaIndex>> {
        self.from_item_indexes()
            .map(|ffrs| ffrs.into_iter().collect())
    }
    fn selection_indexes(&self) -> ApllodbResult<Vec<SchemaIndex>> {
        if let Some(ast_condition) = &self.select_command.where_condition {
            let from_correlations = self.from_item_correlations()?;
            let expression =
                AstTranslator::condition_in_select(ast_condition.clone(), &from_correlations)?;
            let indexes = expression.to_schema_indexes();
            Ok(indexes)
        } else {
            Ok(vec![])
        }
    }
    fn sort_indexes(&self) -> ApllodbResult<Vec<SchemaIndex>> {
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
