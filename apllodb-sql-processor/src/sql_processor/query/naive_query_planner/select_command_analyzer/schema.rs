use std::collections::HashSet;

use apllodb_shared_components::{ApllodbError, ApllodbResult, Expression, SchemaIndex};
use apllodb_sql_parser::apllodb_ast;

use crate::attribute::attribute_name::AttributeName;
use crate::{
    ast_translator::AstTranslator,
    correlation::aliased_correlation_name::AliasedCorrelationName,
    field::{aliased_field_name::AliasedFieldName, field_name::FieldName},
    records::record_schema::RecordSchema,
};
use apllodb_shared_components::{ApllodbErrorKind, SchemaName};
use apllodb_storage_engine_interface::ColumnName;

use super::SelectCommandAnalyzer;

impl SelectCommandAnalyzer {
    /// including all fields used during a SELECT execution
    pub(in super::super) fn widest_schema(&self) -> ApllodbResult<RecordSchema> {
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

    pub(in super::super) fn aliased_field_names_in_projection(
        &self,
    ) -> ApllodbResult<Vec<AliasedFieldName>> {
        let from_item_correlations = self.from_item_correlations()?;
        let ast_select_fields = self.select_command.select_fields.as_vec().clone();

        ast_select_fields
            .iter()
            .map(|select_field| {
                Self::select_field_into_aliased_field_name(select_field, &from_item_correlations)
            })
            .collect::<ApllodbResult<_>>()
    }
    fn select_field_into_aliased_field_name(
        ast_select_field: &apllodb_ast::SelectField,
        from_item_correlations: &[AliasedCorrelationName],
    ) -> ApllodbResult<AliasedFieldName> {
        let expression = AstTranslator::expression_in_select(
            ast_select_field.expression.clone(),
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
                let afn = if let Some(a) = &ast_select_field.alias {
                    AliasedFieldName::new(field_name, Some(AstTranslator::field_alias(a.clone())?))
                } else {
                    AliasedFieldName::new(field_name, None)
                };
                Ok(afn)
            }
        }
    }

    fn join_indexes(&self) -> ApllodbResult<Vec<SchemaIndex>> {
        self.from_item_indexes()
            .map(|idxs| idxs.into_iter().collect())
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
        let idx_orderings = self.sort_index_orderings()?;
        Ok(idx_orderings.into_iter().map(|(idx, _)| idx).collect())
    }

    /// TODO may need Catalog value when:
    /// - index does not have prefix part and
    /// - from_item_correlations are more than 1
    /// because this function has to determine which of `from1` or `from2` `field1` is from.
    ///
    /// # Failures
    ///
    /// - [InvalidColumnReference](apllodb_shared_components::ApllodbErrorKind::InvalidColumnReference) when:
    ///   - `from_item_correlations` is empty.
    /// - [UndefinedColumn](apllodb_shared_components::ApllodbErrorKind::UndefinedColumn) when:
    ///   - none of `from_item_correlations` has field named `ast_column_reference.column_name`
    ///   - `ast_column_reference` has a correlation but it is not any of `from_item_correlations`.
    pub(super) fn field_name(
        index: &SchemaIndex,
        from_item_correlations: &[AliasedCorrelationName],
    ) -> ApllodbResult<FieldName> {
        if from_item_correlations.is_empty() {
            Err(ApllodbError::new(
                ApllodbErrorKind::InvalidColumnReference,
                format!(
                    "no FROM item. cannot detect where `{}` field is from",
                    index
                ),
                None,
            ))
        } else if let Some(corr) = index.prefix() {
            Self::field_name_with_prefix(corr, index.attr(), from_item_correlations)
        } else {
            Self::field_name_without_prefix(index.attr(), from_item_correlations)
        }
    }

    /// # Failures
    ///
    /// - [UndefinedColumn](apllodb_shared_components::ApllodbErrorKind::UndefinedColumn) when:
    ///   - `ast_correlation` does not match any of `from_item_correlations`.
    fn field_name_with_prefix(
        prefix: &str,
        attr: &str,
        from_item_correlations: &[AliasedCorrelationName],
    ) -> ApllodbResult<FieldName> {
        assert!(!from_item_correlations.is_empty());

        let attr = AttributeName::ColumnNameVariant(ColumnName::new(attr)?);
        let index = SchemaIndex::from(format!("{}.{}", prefix, attr).as_str());

        // SELECT T.C FROM ...;
        from_item_correlations
            .iter()
            .find_map(|from_item_corr| {
                // creates AliasedFieldName to use .matches()
                let field_name_candidate = AliasedFieldName::new(
                    FieldName::new(from_item_corr.clone(), attr.clone()),
                    None,
                );
                field_name_candidate
                    .matches(&index)
                    .then(|| field_name_candidate.field_name)
            })
            .ok_or_else(|| {
                ApllodbError::new(
                    ApllodbErrorKind::UndefinedColumn,
                    format!(
                        "`{}` does not match any of FROM items: {:?}",
                        index, from_item_correlations
                    ),
                    None,
                )
            })
    }

    fn field_name_without_prefix(
        attr: &str,
        from_item_correlations: &[AliasedCorrelationName],
    ) -> ApllodbResult<FieldName> {
        assert!(!from_item_correlations.is_empty());
        if from_item_correlations.len() > 1 {
            return Err(ApllodbError::feature_not_supported(format!(
                "needs catalog info to detect which table has the column `{:?}`",
                attr
            )));
        }

        // SELECT C FROM T (AS a)?;
        // -> C is from T
        let from_item_correlation = from_item_correlations[0].clone();
        let attr = AttributeName::ColumnNameVariant(ColumnName::new(attr)?);
        Ok(FieldName::new(from_item_correlation, attr))
    }
}

#[cfg(test)]
mod tests {
    use super::super::SelectCommandAnalyzer;
    use crate::{
        correlation::aliased_correlation_name::AliasedCorrelationName, field::field_name::FieldName,
    };
    use apllodb_shared_components::{ApllodbErrorKind, SchemaIndex};
    use pretty_assertions::assert_eq;

    #[derive(new)]
    struct TestDatum {
        index: SchemaIndex,
        from_item_correlations: Vec<AliasedCorrelationName>,
        expected_result: Result<FieldName, ApllodbErrorKind>,
    }

    #[test]
    fn test_column_reference() {
        let test_data: Vec<TestDatum> = vec![
            TestDatum::new(
                SchemaIndex::from("c"),
                vec![],
                Err(ApllodbErrorKind::InvalidColumnReference),
            ),
            TestDatum::new(
                SchemaIndex::from("t.c"),
                vec![],
                Err(ApllodbErrorKind::InvalidColumnReference),
            ),
            TestDatum::new(
                SchemaIndex::from("c"),
                vec![AliasedCorrelationName::factory_tn("t")],
                Ok(FieldName::factory("t", "c")),
            ),
            TestDatum::new(
                SchemaIndex::from("t.c"),
                vec![AliasedCorrelationName::factory_tn("t")],
                Ok(FieldName::factory("t", "c")),
            ),
            TestDatum::new(
                SchemaIndex::from("t1.c"),
                vec![AliasedCorrelationName::factory_tn("t2")],
                Err(ApllodbErrorKind::UndefinedColumn),
            ),
            TestDatum::new(
                SchemaIndex::from("c"),
                vec![AliasedCorrelationName::factory_tn("t").with_alias("a")],
                Ok(FieldName::factory("t", "c").with_corr_alias("a")),
            ),
            TestDatum::new(
                SchemaIndex::from("t.c"),
                vec![AliasedCorrelationName::factory_tn("t").with_alias("a")],
                Ok(FieldName::factory("t", "c").with_corr_alias("a")),
            ),
            TestDatum::new(
                SchemaIndex::from("a.c"),
                vec![AliasedCorrelationName::factory_tn("t").with_alias("a")],
                Ok(FieldName::factory("t", "c").with_corr_alias("a")),
            ),
            TestDatum::new(
                SchemaIndex::from("x.c"),
                vec![AliasedCorrelationName::factory_tn("t").with_alias("a")],
                Err(ApllodbErrorKind::UndefinedColumn),
            ),
        ];

        for test_datum in test_data {
            match SelectCommandAnalyzer::field_name(
                &test_datum.index,
                &test_datum.from_item_correlations,
            ) {
                Ok(field_name) => {
                    assert_eq!(field_name, test_datum.expected_result.unwrap())
                }
                Err(e) => {
                    assert_eq!(e.kind(), &test_datum.expected_result.unwrap_err())
                }
            }
        }
    }
}
