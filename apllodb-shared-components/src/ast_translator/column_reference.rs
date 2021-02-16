use crate::{
    data_structure::reference::{
        correlation_reference::CorrelationReference, field_reference::FieldReference,
    },
    AliasName, ApllodbError, ApllodbErrorKind, ApllodbResult, ColumnName, FullFieldReference,
    TableName,
};
use apllodb_sql_parser::apllodb_ast::{self};

use crate::ast_translator::AstTranslator;

impl AstTranslator {
    /// TODO may need Catalog value when:
    /// - ast_column_reference does not have correlation and
    /// - ast_table_names are more than 1
    /// because this function has to determine which of `from1` or `from2` `field1` is from.
    ///
    /// # Failures
    ///
    /// - [InvalidColumnReference](crate::ApllodbErrorKind::InvalidColumnReference) when:
    ///   - `ast_table_names` is empty.
    /// - [UndefinedColumn](crate::ApllodbErrorKind::UndefinedColumn) when:
    ///   - none of `ast_from_item` has field named `ast_column_reference.column_name`
    /// - [UndefinedObject](crate::ApllodbErrorKind::UndefinedObject) when:
    ///   - `ast_column_reference` has a correlation but it is not any of `ast_table_names`.
    /// - [AmbiguousColumn](crate::ApllodbErrorKind::AmbiguousColumn) when:
    ///   - ここ、FromItemを一旦domainのものにして、そこにおいてDuplicateObjectにしたほうがいい
    pub fn column_reference(
        ast_column_reference: apllodb_ast::ColumnReference,
        ast_table_names: Vec<(apllodb_ast::TableName, Option<apllodb_ast::Alias>)>,
    ) -> ApllodbResult<FullFieldReference> {
        if ast_table_names.is_empty() {
            Err(ApllodbError::new(
                ApllodbErrorKind::InvalidColumnReference,
                format!(
                    "no FROM item. cannot detect where `{:?}` field is from",
                    ast_column_reference
                ),
                None,
            ))
        } else {
            if let Some(corr) = ast_column_reference.correlation {
                Self::column_reference_with_corr(
                    corr,
                    ast_column_reference.column_name,
                    ast_table_names,
                )
            } else {
                Self::column_reference_without_corr(
                    ast_column_reference.column_name,
                    ast_table_names,
                )
            }
        }

        // match ast_table_names.len() {
        //     0 => {
        //         // SELECT (ta.)?C;
        //         // -> InvalidColumnReference
        //         Err(ApllodbError::new(
        //             ApllodbErrorKind::InvalidColumnReference,
        //             format!(
        //                 "no FROM item. cannot detect where `{:?}` field is from",
        //                 ast_column_reference
        //             ),
        //             None,
        //         ))
        //     }
        //     1 => {
        //         // SELECT (ta.)?C FROM T (AS a)?;
        //         let (ast_table_name, opt_ast_alias) = &ast_table_names[0];

        //         let correlation_reference: CorrelationReference = match &ast_column_reference
        //             .correlation
        //         {
        //             None => {
        //                 // SELECT C FROM T (AS a)?;
        //                 // -> C is from T
        //                 if let Some(apllodb_ast::Alias(apllodb_ast::Identifier(alias))) =
        //                     &opt_ast_alias
        //                 {
        //                     Ok(CorrelationReference::TableAliasVariant {
        //                         table_name: TableName::new(ast_table_name.0 .0.clone())?,
        //                         alias_name: AliasName::new(alias)?,
        //                     })
        //                 } else {
        //                     Ok(CorrelationReference::TableNameVariant(TableName::new(
        //                         ast_table_name.0 .0.clone(),
        //                     )?))
        //                 }
        //             }
        //             Some(apllodb_ast::Correlation(apllodb_ast::Identifier(colref_corr))) => {
        //                 // SELECT ta.C FROM T (AS a)?;
        //                 if colref_corr == &ast_table_name.0 .0 {
        //                     // SELECT T.C FROM T (AS a)?;
        //                     // -> C is from T
        //                     if let Some(apllodb_ast::Alias(apllodb_ast::Identifier(alias))) =
        //                         &opt_ast_alias
        //                     {
        //                         Ok(CorrelationReference::TableAliasVariant {
        //                             table_name: TableName::new(colref_corr)?,
        //                             alias_name: AliasName::new(alias)?,
        //                         })
        //                     } else {
        //                         Ok(CorrelationReference::TableNameVariant(TableName::new(
        //                             colref_corr,
        //                         )?))
        //                     }
        //                 } else {
        //                     // SELECT a1.C FROM T (AS a2)?;
        //                     match &opt_ast_alias {
        //                         None => {
        //                             // SELECT a_not_T.C FROM T;
        //                             // -> UndefinedColumn
        //                             Err(ApllodbError::new(
        //                                 ApllodbErrorKind::UndefinedColumn,
        //                                 format!(
        //                                     "correlation of column reference `{:?}` is not the same as FROM item `{:?}`",
        //                                     ast_column_reference, ast_table_name
        //                                 ),
        //                                 None,
        //                             ))
        //                         }
        //                         Some(apllodb_ast::Alias(apllodb_ast::Identifier(
        //                             from_item_alias,
        //                         ))) => {
        //                             // SELECT a_not_T.C FROM T AS A;
        //                             if colref_corr == from_item_alias {
        //                                 // SELECT A.C FROM T AS A;
        //                                 // -> C is FROM T aliased as A
        //                                 Ok(CorrelationReference::TableAliasVariant {
        //                                     alias_name: AliasName::new(colref_corr)?,
        //                                     table_name: TableName::new(
        //                                         ast_table_name.0 .0.clone(),
        //                                     )?,
        //                                 })
        //                             } else {
        //                                 // SELECT not_a_t.C FROM T AS A;
        //                                 // -> UndefinedColumn
        //                                 Err(ApllodbError::new(
        //                                     ApllodbErrorKind::UndefinedColumn,
        //                                     format!(
        //                                         "correlation of column reference `{:?}` is not the same as FROM item `{:?}`",
        //                                         ast_column_reference, ast_table_name
        //                                     ),
        //                                     None,
        //                                 ))
        //                             }
        //                         }
        //                     }
        //                 }
        //             }
        //         }?;

        //         let field_reference = FieldReference::ColumnNameVariant(ColumnName::new(
        //             ast_column_reference.column_name.0 .0,
        //         )?);

        //         Ok(FullFieldReference::new(
        //             correlation_reference,
        //             field_reference,
        //         ))
        //     }
        //     _ => unimplemented!(),
        // }
    }

    fn column_reference_with_corr(
        ast_correlation: apllodb_ast::Correlation,
        ast_column_name: apllodb_ast::ColumnName,
        ast_table_names: Vec<(apllodb_ast::TableName, Option<apllodb_ast::Alias>)>,
    ) -> ApllodbResult<FullFieldReference> {
        assert!(!ast_table_names.is_empty());
    }

    fn column_reference_without_corr(
        ast_column_name: apllodb_ast::ColumnName,
        ast_table_names: Vec<(apllodb_ast::TableName, Option<apllodb_ast::Alias>)>,
    ) -> ApllodbResult<FullFieldReference> {
        assert!(!ast_table_names.is_empty());
        if ast_table_names.len() > 1 {
            unimplemented!(
                "needs catalog info to detect which table has the column `{:?}`",
                ast_column_name
            )
        }

        let (ast_table_name, opt_ast_alias) = &ast_table_names[0];

        // SELECT C FROM T (AS a)?;
        // -> C is from T
        let correlation_reference: CorrelationReference =
            if let Some(apllodb_ast::Alias(apllodb_ast::Identifier(alias))) = &opt_ast_alias {
                CorrelationReference::TableAliasVariant {
                    table_name: TableName::new(ast_table_name.0 .0.clone())?,
                    alias_name: AliasName::new(alias)?,
                }
            } else {
                CorrelationReference::TableNameVariant(TableName::new(ast_table_name.0 .0.clone())?)
            };

        let field_reference =
            FieldReference::ColumnNameVariant(ColumnName::new(ast_column_name.0 .0)?);

        Ok(FullFieldReference::new(
            correlation_reference,
            field_reference,
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{ApllodbErrorKind, ApllodbResult, AstTranslator, FullFieldReference};
    use apllodb_sql_parser::apllodb_ast::{ColumnReference, Correlation, FromItem};
    use pretty_assertions::assert_eq;

    #[derive(new)]
    struct TestDatum {
        ast_column_reference: ColumnReference,
        ast_table_names: Vec<FromItem>,
        expected_result: Result<FullFieldReference, ApllodbErrorKind>,
    }

    #[test]
    fn test_column_reference() -> ApllodbResult<()> {
        let test_data: Vec<TestDatum> = vec![
            TestDatum::new(
                ColumnReference::factory(None, "c"),
                vec![],
                Err(ApllodbErrorKind::InvalidColumnReference),
            ),
            TestDatum::new(
                ColumnReference::factory(Some(Correlation::factory("t")), "c"),
                vec![],
                Err(ApllodbErrorKind::InvalidColumnReference),
            ),
            TestDatum::new(
                ColumnReference::factory(None, "c"),
                vec![FromItem::factory_tn("t", None)],
                Ok(FullFieldReference::factory("t", "c")),
            ),
            TestDatum::new(
                ColumnReference::factory(Some(Correlation::factory("t")), "c"),
                vec![FromItem::factory_tn("t", None)],
                Ok(FullFieldReference::factory("t", "c")),
            ),
            TestDatum::new(
                ColumnReference::factory(Some(Correlation::factory("t1")), "c"),
                vec![FromItem::factory_tn("t2", None)],
                Err(ApllodbErrorKind::UndefinedColumn),
            ),
            TestDatum::new(
                ColumnReference::factory(None, "c"),
                vec![FromItem::factory_tn("t", Some("a"))],
                Ok(FullFieldReference::factory("t", "c").with_corr_alias("a")),
            ),
            TestDatum::new(
                ColumnReference::factory(Some(Correlation::factory("t")), "c"),
                vec![FromItem::factory_tn("t", Some("a"))],
                Ok(FullFieldReference::factory("t", "c").with_corr_alias("a")),
            ),
            TestDatum::new(
                ColumnReference::factory(Some(Correlation::factory("a")), "c"),
                vec![FromItem::factory_tn("t", Some("a"))],
                Ok(FullFieldReference::factory("t", "c").with_corr_alias("a")),
            ),
            TestDatum::new(
                ColumnReference::factory(Some(Correlation::factory("x")), "c"),
                vec![FromItem::factory_tn("t", Some("a"))],
                Err(ApllodbErrorKind::UndefinedColumn),
            ),
        ];

        for test_datum in test_data {
            match AstTranslator::column_reference(
                test_datum.ast_column_reference,
                test_datum
                    .ast_table_names
                    .into_iter()
                    .map(|from_item| {
                        if let FromItem::TableNameVariant { table_name, alias } = from_item {
                            (table_name, alias)
                        } else {
                            unreachable!("only TableNameVariant should be tested here")
                        }
                    })
                    .collect(),
            ) {
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
