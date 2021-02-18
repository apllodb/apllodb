use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    data_structure::reference::correlation_reference::CorrelationReference, AliasName,
    ApllodbResult, ColumnName, FromItem, FullFieldReference, TableName, TableWithAlias,
};

use super::{field_reference::FieldReference, FieldReferenceBase};

/// Unresolved field reference is in a "(correlation.)?field" form.
///
/// It's correlation may be omitted in SQL.
/// E.g. `SELECT c FROM t  -- t is omitted`
///
/// Omitted correlation can be *resolved* by FromItem.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct UnresolvedFieldReference(FieldReferenceBase);

impl Display for UnresolvedFieldReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

impl UnresolvedFieldReference {
    /// Constructor
    pub fn new(
        correlation_reference: Option<CorrelationReference>,
        field_reference: FieldReference,
    ) -> Self {
        let base = FieldReferenceBase::new(correlation_reference, field_reference);
        Self(base)
    }

    /// Get ref of CorrelationReference
    pub fn as_correlation_reference(&self) -> Option<&CorrelationReference> {
        self.0.as_correlation_reference()
    }

    /// Get ref of TableName
    pub fn as_table_name(&self) -> Option<&TableName> {
        self.0.as_table_name()
    }

    /// Get ref of FieldReference
    pub fn as_field_reference(&self) -> &FieldReference {
        self.0.as_field_reference()
    }

    /// Get ref of ColumnName
    pub fn as_column_name(&self) -> &ColumnName {
        self.0.as_column_name()
    }

    /// Set correlation reference
    ///
    /// # Panics
    ///
    /// When correlation does not exist.
    pub fn set_correlation_alias(&mut self, correlation_alias: AliasName) {
        self.0.set_correlation_alias(correlation_alias)
    }

    /// Set field reference
    pub fn set_field_alias(&mut self, field_alias: AliasName) {
        self.0.set_field_alias(field_alias)
    }

    /// into FullFieldReference
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
            None => Ok(FullFieldReference::new(self.0)),
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
        if let Some(corr) = self.as_correlation_reference() {
            Self::resolve_with_table_with_corr(corr, self.as_field_reference(), table_with_alias)
        } else {
            Self::resolve_with_table_without_corr(self.as_field_reference(), table_with_alias)
        }
    }

    fn resolve_with_table_with_corr(
        _correlation_reference: &CorrelationReference,
        _field_reference: &FieldReference,
        _table_with_alias: TableWithAlias,
    ) -> ApllodbResult<FullFieldReference> {
        todo!()
    }

    fn resolve_with_table_without_corr(
        field_reference: &FieldReference,
        table_with_alias: TableWithAlias,
    ) -> ApllodbResult<FullFieldReference> {
        // SELECT C FROM T (AS a)?;
        // -> C is from T
        //
        // FIXME: it's wrong to eagerly determine "C is from T" when TableWithAlias are more than one.
        // Need to check catalog.

        let correlation_reference = CorrelationReference::TableVariant(table_with_alias);

        Ok(FullFieldReference::new(FieldReferenceBase::new(
            Some(correlation_reference),
            field_reference.clone(),
        )))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ApllodbErrorKind, ApllodbResult, FromItem, FullFieldReference, UnresolvedFieldReference,
    };
    use pretty_assertions::assert_eq;

    #[derive(new)]
    struct TestDatum {
        ufr: UnresolvedFieldReference,
        from_item: Option<FromItem>,
        expected_result: Result<FullFieldReference, ApllodbErrorKind>,
    }

    #[test]
    fn test_resolve() -> ApllodbResult<()> {
        let test_data: Vec<TestDatum> = vec![
            TestDatum::new(
                UnresolvedFieldReference::factory_cn("c"),
                None,
                Err(ApllodbErrorKind::InvalidColumnReference),
            ),
            TestDatum::new(
                UnresolvedFieldReference::factory_tn_cn("t", "c"),
                None,
                Err(ApllodbErrorKind::InvalidColumnReference),
            ),
            TestDatum::new(
                UnresolvedFieldReference::factory_cn("c"),
                Some(FromItem::factory("t")),
                Ok(UnresolvedFieldReference::factory_tn_cn("t", "c").resolve_naive()),
            ),
            TestDatum::new(
                UnresolvedFieldReference::factory_tn_cn("t", "c"),
                Some(FromItem::factory("t")),
                Ok(UnresolvedFieldReference::factory_tn_cn("t", "c").resolve_naive()),
            ),
            TestDatum::new(
                UnresolvedFieldReference::factory_tn_cn("t1", "c"),
                Some(FromItem::factory("t2")),
                Err(ApllodbErrorKind::InvalidColumnReference),
            ),
            TestDatum::new(
                UnresolvedFieldReference::factory_cn("c"),
                Some(FromItem::factory_with_corr_alias("t", "a")),
                Ok(UnresolvedFieldReference::factory_tn_cn("t", "c")
                    .with_corr_alias("a")
                    .resolve_naive()),
            ),
            TestDatum::new(
                UnresolvedFieldReference::factory_tn_cn("t", "c"),
                Some(FromItem::factory_with_corr_alias("t", "a")),
                Ok(UnresolvedFieldReference::factory_tn_cn("t", "c")
                    .with_corr_alias("a")
                    .resolve_naive()),
            ),
            TestDatum::new(
                UnresolvedFieldReference::factory_tn_cn("a", "c"),
                Some(FromItem::factory_with_corr_alias("t", "a")),
                Ok(UnresolvedFieldReference::factory_tn_cn("t", "c")
                    .with_corr_alias("a")
                    .resolve_naive()),
            ),
            TestDatum::new(
                UnresolvedFieldReference::factory_tn_cn("x", "c"),
                Some(FromItem::factory_with_corr_alias("t", "a")),
                Err(ApllodbErrorKind::InvalidColumnReference),
            ),
        ];

        for test_datum in test_data {
            match test_datum.ufr.resolve(test_datum.from_item) {
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
