use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    traits::correlation::Correlation, AliasName, ApllodbError, ApllodbErrorKind, ApllodbResult,
    ColumnName, CorrelationName, CorrelationReference, FromItem, FullFieldReference,
    TableWithAlias,
};

use super::field_reference::FieldReference;

/// Unresolved field reference is in a "(correlation.)?field" form, appears in SELECT field.
///
/// It's correlation may be omitted in SQL.
/// E.g. `SELECT c FROM t  -- t is omitted`
///
/// Omitted correlation can be *resolved* by FromItem.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize, new)]
pub struct SelectFieldReference {
    correlation_name: Option<CorrelationName>,
    field_reference: FieldReference,
}

impl Display for SelectFieldReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            if let Some(corr) = self.as_correlation_name() {
                format!("{}.", corr)
            } else {
                "".to_string()
            },
            self.field_reference
        )
    }
}

impl SelectFieldReference {
    /// Get ref of CorrelationName
    pub fn as_correlation_name(&self) -> Option<&CorrelationName> {
        self.correlation_name.as_ref()
    }

    /// Get ref of FieldReference
    pub fn as_field_reference(&self) -> &FieldReference {
        &self.field_reference
    }

    /// Get ref of ColumnName
    pub fn as_column_name(&self) -> &ColumnName {
        match &self.field_reference {
            FieldReference::ColumnNameVariant(cn) => cn,
            FieldReference::ColumnAliasVariant { column_name, .. } => column_name,
        }
    }

    /// Set field reference
    pub fn set_field_alias(&mut self, field_alias: AliasName) {
        let cur_column_name = self.as_column_name();
        self.field_reference = FieldReference::ColumnAliasVariant {
            alias_name: field_alias,
            column_name: cur_column_name.clone(),
        };
    }

    /// into FullFieldReference w/ FromItem.
    ///
    /// `ast_from_item` is None only when SELECT statement does not have FROM clause.
    ///
    /// # Failures
    ///
    /// - [InvalidColumnReference](crate::ApllodbErrorKind::InvalidColumnReference) when:
    ///   - `ast_from_item` does not have matching correlation.
    ///   - `ast_from_item` does not have matching field.
    pub fn resolve(self, from_item: Option<FromItem>) -> ApllodbResult<FullFieldReference> {
        match from_item {
            None => {
                if let Some(_corr) = self.as_correlation_name() {
                    Err(ApllodbError::new(
                        ApllodbErrorKind::InvalidColumnReference,
                        format!(
                            "field `{}` specified but the correlation is not in FROM clause",
                            self.as_field_reference()
                        ),
                        None,
                    ))
                } else {
                    Ok(FullFieldReference::new(None, self.field_reference))
                }
            }
            Some(from_item) => {
                let tables: Vec<TableWithAlias> = (&from_item).into();
                assert!(!tables.is_empty());

                if tables.len() > 1 {
                    unimplemented!("needs catalog info")
                }

                let table = tables.first().unwrap().clone();
                self.resolve_with_table(table)
            }
        }
    }

    fn resolve_with_table(
        self,
        table_with_alias: TableWithAlias,
    ) -> ApllodbResult<FullFieldReference> {
        if let Some(corr) = self.as_correlation_name() {
            Self::resolve_with_table_with_prefix(corr, self.as_field_reference(), &table_with_alias)
        } else {
            Self::resolve_with_table_without_prefix(self.as_field_reference(), &table_with_alias)
        }
    }

    fn resolve_with_table_with_prefix(
        prefix: &CorrelationName,
        field: &FieldReference,
        from: &TableWithAlias,
    ) -> ApllodbResult<FullFieldReference> {
        // SELECT ta.C FROM T (AS a)?;

        if from.is_named(prefix) {
            Ok(FullFieldReference::new(
                Some(CorrelationReference::from(from.clone())),
                field.clone(),
            ))
        } else {
            Err(ApllodbError::new(
                ApllodbErrorKind::InvalidColumnReference,
                format!(
                    "field `{:?}` is not the same as FROM item `{:?}`",
                    field, from
                ),
                None,
            ))
        }
    }

    fn resolve_with_table_without_prefix(
        field_reference: &FieldReference,
        table_with_alias: &TableWithAlias,
    ) -> ApllodbResult<FullFieldReference> {
        // SELECT C FROM T (AS a)?;
        // -> C is from T
        //
        // FIXME: it's wrong to eagerly determine "C is from T" when TableWithAlias are more than one.
        // Need to check catalog.

        let correlation_reference = CorrelationReference::from(table_with_alias.clone());

        Ok(FullFieldReference::new(
            Some(correlation_reference),
            field_reference.clone(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ApllodbErrorKind, ApllodbResult, FromItem, FullFieldReference, SelectFieldReference,
    };
    use pretty_assertions::assert_eq;

    #[derive(Debug, new)]
    struct TestDatum {
        sfr: SelectFieldReference,
        from_item: Option<FromItem>,
        expected_result: Result<FullFieldReference, ApllodbErrorKind>,
    }

    #[test]
    fn test_resolve() -> ApllodbResult<()> {
        let test_data: Vec<TestDatum> = vec![
            TestDatum::new(
                SelectFieldReference::factory_cn("c"),
                None,
                Ok(SelectFieldReference::factory_cn("c").resolve_naive()),
            ),
            TestDatum::new(
                SelectFieldReference::factory_corr_cn("t", "c"),
                None,
                Err(ApllodbErrorKind::InvalidColumnReference),
            ),
            TestDatum::new(
                SelectFieldReference::factory_cn("c"),
                Some(FromItem::factory("t")),
                Ok(SelectFieldReference::factory_corr_cn("t", "c").resolve_naive()),
            ),
            TestDatum::new(
                SelectFieldReference::factory_corr_cn("t", "c"),
                Some(FromItem::factory("t")),
                Ok(SelectFieldReference::factory_corr_cn("t", "c").resolve_naive()),
            ),
            TestDatum::new(
                SelectFieldReference::factory_corr_cn("t1", "c"),
                Some(FromItem::factory("t2")),
                Err(ApllodbErrorKind::InvalidColumnReference),
            ),
            TestDatum::new(
                SelectFieldReference::factory_corr_cn("x", "c"),
                Some(FromItem::factory_with_corr_alias("t", "a")),
                Err(ApllodbErrorKind::InvalidColumnReference),
            ),
        ];

        for test_datum in test_data {
            log::debug!("{:#?}", test_datum);

            match test_datum.sfr.resolve(test_datum.from_item) {
                Ok(ffr) => {
                    assert_eq!(ffr, test_datum.expected_result.unwrap())
                }
                Err(e) => {
                    assert_eq!(e.kind(), &test_datum.expected_result.unwrap_err())
                }
            }
        }

        Ok(())
    }
}
