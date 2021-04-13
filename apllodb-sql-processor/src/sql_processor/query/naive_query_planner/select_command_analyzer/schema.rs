use crate::{
    attribute::attribute_name::AttributeName,
    correlation::aliased_correlation_name::AliasedCorrelationName,
    field::{aliased_field_name::AliasedFieldName, field_name::FieldName},
};
use apllodb_shared_components::{
    ApllodbError, ApllodbErrorKind, ApllodbResult, SchemaIndex, SchemaName,
};
use apllodb_storage_engine_interface::ColumnName;

use super::SelectCommandAnalyzer;

impl SelectCommandAnalyzer {
    /// TODO may need Catalog value when:
    /// - index does not have prefix part and
    /// - from_item_correlations are more than 1
    /// because this function has to determine which of `from1` or `from2` `field1` is from.
    ///
    /// # Failures
    ///
    /// - [InvalidColumnReference](crate::ApllodbErrorKind::InvalidColumnReference) when:
    ///   - `from_item_correlations` is empty.
    /// - [UndefinedColumn](crate::ApllodbErrorKind::UndefinedColumn) when:
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
    /// - [UndefinedColumn](crate::ApllodbErrorKind::UndefinedColumn) when:
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
                if field_name_candidate.matches(&index) {
                    Some(field_name_candidate.field_name)
                } else {
                    None
                }
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
    use apllodb_sql_parser::apllodb_ast::{ColumnReference, Correlation};
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
