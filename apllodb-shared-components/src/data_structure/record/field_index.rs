use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    traits::correlation::Correlation, ApllodbError, ApllodbErrorKind, ApllodbResult,
    CorrelationName, FieldName, FieldReference, FullFieldReference, SelectFieldReference,
};

/// Matcher to [FullFieldReference](crate::FullFieldReference).
/// Used to get a value from a record.
///
/// # Example
///
/// ```
/// # use apllodb_shared_components::FieldIndex;
///
/// let _ = FieldIndex::from("c");    // column name or alias name "c"
/// let _ = FieldIndex::from("t.c");  // column name or alias name "c"; inside table / table alias / subquery alias named "t".
///
/// assert_eq!(FieldIndex::from("c"), FieldIndex::from("  c "));
/// assert_ne!(FieldIndex::from("c"), FieldIndex::from("C"));
/// assert_eq!(FieldIndex::from("t.c"), FieldIndex::from("  t  .  c "));
/// ```
///
/// # Panics
///
/// When constructed from invalid-formed string.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct FieldIndex {
    correlation_name: Option<CorrelationName>,
    field_name: FieldName,
}

impl FieldIndex {
    /// Find a FullFieldReference index which matches this FieldIndex.
    ///
    /// # Failures
    ///
    /// - [InvalidName](crate::ApllodbErrorKind::InvalidName) when:
    ///   - none of `full_field_references` matches to this FieldIndex.
    ///   - this FieldIndex has correlation while SELECT SQL does not include FROM item.
    /// - [AmbiguousColumn](crate::ApllodbErrorKind::AmbiguousColumn) when:
    ///   - more than 1 of `full_field_references` match to this FieldIndex.
    pub(crate) fn peek(
        &self,
        full_field_references: &[FullFieldReference],
    ) -> ApllodbResult<(usize, FullFieldReference)> {
        let mut match_fn: Box<dyn FnMut(&FullFieldReference) -> bool> = match &self.correlation_name
        {
            None => {
                Box::new(move |ffr| Self::matches_without_index_corr(self.field_name.clone(), ffr))
            }
            Some(index_corr) => Box::new(move |ffr| {
                Self::matches_with_index_corr(index_corr.clone(), self.field_name.clone(), ffr)
            }),
        };

        let mut matches: Vec<(usize, FullFieldReference)> = full_field_references
            .iter()
            .enumerate()
            .filter_map(|(idx, ffr)| {
                if match_fn(ffr) {
                    Some((idx, ffr.clone()))
                } else {
                    None
                }
            })
            .collect();

        match matches.len() {
            0 => Err(ApllodbError::new(
                ApllodbErrorKind::InvalidName,
                format!(
                    "field index `{}` does not match any of full_field_references",
                    self
                ),
                None,
            )),
            1 => Ok(matches.pop().unwrap()),
            _ => Err(ApllodbError::new(
                ApllodbErrorKind::AmbiguousColumn,
                format!(
                    "field index `{}` match to more than 1 of full_field_references",
                    self
                ),
                None,
            )),
        }
    }

    fn matches_with_index_corr(
        self_corr: CorrelationName,
        self_field: FieldName,
        full_field_reference: &FullFieldReference,
    ) -> bool {
        match (
            full_field_reference.as_correlation_reference(),
            full_field_reference.as_field_reference(),
        ) {
            (None, _) => false,
            (Some(ffr_corr), ffr_field) => {
                ffr_corr.is_named(&self_corr) && ffr_field.is_named(&self_field)
            }
        }
    }

    fn matches_without_index_corr(
        self_field: FieldName,
        full_field_reference: &FullFieldReference,
    ) -> bool {
        full_field_reference
            .as_field_reference()
            .is_named(&self_field)
    }
}

impl Display for FieldIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pre = if let Some(corr) = &self.correlation_name {
            format!("{}.", corr)
        } else {
            "".to_string()
        };
        write!(f, "{}{}", pre, self.field_name)
    }
}

impl<S: Into<String>> From<S> for FieldIndex {
    /// This method is called from client code so panic!() would not be a big problem.
    fn from(s: S) -> Self {
        let s: String = s.into();
        let parts: Vec<&str> = s.split('.').collect();

        debug_assert!(!parts.is_empty());
        assert!(parts.len() <= 2, "too many dots (.) !");

        parts.iter().for_each(|part| {
            assert!(
                !part.is_empty(),
                "correlation name nor field name must not be empty string"
            )
        });

        let first = parts
            .get(0)
            .expect("must have at least 1 part")
            .trim()
            .to_string();
        let second = parts.get(1).map(|s| s.trim().to_string());

        if let Some(second) = second {
            Self {
                correlation_name: Some(CorrelationName::new(first).unwrap()),
                field_name: FieldName::new(second).unwrap(),
            }
        } else {
            Self {
                correlation_name: None,
                field_name: FieldName::new(first).unwrap(),
            }
        }
    }
}

impl From<SelectFieldReference> for FieldIndex {
    fn from(unresolved_field_reference: SelectFieldReference) -> Self {
        match (
            unresolved_field_reference.as_correlation_name(),
            unresolved_field_reference.as_field_reference(),
        ) {
            (None, FieldReference::ColumnNameVariant(column_name))
            | (None, FieldReference::ColumnAliasVariant { column_name, .. }) => {
                Self::from(column_name.as_str())
            }

            (Some(corr), FieldReference::ColumnNameVariant(column_name))
            | (Some(corr), FieldReference::ColumnAliasVariant { column_name, .. }) => {
                Self::from(format!("{}.{}", corr.as_str(), column_name.as_str()))
            }
        }
    }
}

impl From<FullFieldReference> for FieldIndex {
    fn from(full_field_reference: FullFieldReference) -> Self {
        match (
            full_field_reference.as_correlation_reference(),
            full_field_reference.as_field_reference(),
        ) {
            (None, FieldReference::ColumnNameVariant(column_name))
            | (None, FieldReference::ColumnAliasVariant { column_name, .. }) => {
                Self::from(column_name.as_str())
            }

            (Some(corr), FieldReference::ColumnNameVariant(column_name))
            | (Some(corr), FieldReference::ColumnAliasVariant { column_name, .. }) => {
                Self::from(format!("{}.{}", corr.to_string(), column_name.as_str()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ApllodbErrorKind, ApllodbResult, FieldIndex, FullFieldReference, SelectFieldReference,
    };

    #[test]
    fn test_from_success() {
        let from_to_data: Vec<(&str, &str)> = vec![
            ("c", "c"),
            ("  c ", "c"),
            ("t.c", "t.c"),
            ("   t  .  c    ", "t.c"),
        ];

        for (from, to) in from_to_data {
            assert_eq!(FieldIndex::from(from).to_string(), to);
        }
    }

    #[test]
    #[should_panic]
    fn test_from_panic1() {
        FieldIndex::from("");
    }

    #[test]
    #[should_panic]
    fn test_from_panic2() {
        FieldIndex::from(".c");
    }

    #[test]
    #[should_panic]
    fn test_from_panic3() {
        FieldIndex::from("t.");
    }

    #[test]
    #[should_panic]
    fn test_from_panic4() {
        FieldIndex::from("a.b.c");
    }

    #[test]
    fn test_peek() -> ApllodbResult<()> {
        struct TestDatum {
            field_index: &'static str,
            sfrs: Vec<SelectFieldReference>,
            expected_result: Result<
                usize, // index to matching one from `full_field_references`,
                ApllodbErrorKind,
            >,
        }

        let test_data: Vec<TestDatum> = vec![
            TestDatum {
                field_index: "c",
                sfrs: vec![],
                expected_result: Err(ApllodbErrorKind::InvalidName),
            },
            TestDatum {
                field_index: "c",
                sfrs: vec![SelectFieldReference::factory_cn("c")],
                expected_result: Ok(0),
            },
            TestDatum {
                field_index: "xxx",
                sfrs: vec![SelectFieldReference::factory_cn("c")],
                expected_result: Err(ApllodbErrorKind::InvalidName),
            },
            TestDatum {
                field_index: "c",
                sfrs: vec![SelectFieldReference::factory_corr_cn("t", "c")],
                expected_result: Ok(0),
            },
            TestDatum {
                field_index: "xxx",
                sfrs: vec![SelectFieldReference::factory_corr_cn("t", "c")],
                expected_result: Err(ApllodbErrorKind::InvalidName),
            },
            TestDatum {
                field_index: "c",
                sfrs: vec![SelectFieldReference::factory_corr_cn("t", "c").with_field_alias("ca")],
                expected_result: Ok(0),
            },
            TestDatum {
                field_index: "ca",
                sfrs: vec![SelectFieldReference::factory_corr_cn("t", "c").with_field_alias("ca")],
                expected_result: Ok(0),
            },
            TestDatum {
                field_index: "t.ca",
                sfrs: vec![SelectFieldReference::factory_corr_cn("t", "c").with_field_alias("ca")],
                expected_result: Ok(0),
            },
            TestDatum {
                field_index: "xxx",
                sfrs: vec![SelectFieldReference::factory_corr_cn("t", "c").with_field_alias("ca")],
                expected_result: Err(ApllodbErrorKind::InvalidName),
            },
            TestDatum {
                field_index: "t.c",
                sfrs: vec![SelectFieldReference::factory_corr_cn("t", "c")],
                expected_result: Ok(0),
            },
            TestDatum {
                field_index: "xxx.c",
                sfrs: vec![SelectFieldReference::factory_corr_cn("t", "c")],
                expected_result: Err(ApllodbErrorKind::InvalidName),
            },
            TestDatum {
                field_index: "c2",
                sfrs: vec![
                    SelectFieldReference::factory_corr_cn("t1", "c1"),
                    SelectFieldReference::factory_corr_cn("t2", "c2"),
                ],
                expected_result: Ok(1),
            },
            TestDatum {
                field_index: "t1.c1",
                sfrs: vec![
                    SelectFieldReference::factory_corr_cn("t1", "c1"),
                    SelectFieldReference::factory_corr_cn("t2", "c2"),
                ],
                expected_result: Ok(0),
            },
            TestDatum {
                field_index: "t1.c",
                sfrs: vec![
                    SelectFieldReference::factory_corr_cn("t1", "c"),
                    SelectFieldReference::factory_corr_cn("t2", "c"),
                ],
                expected_result: Ok(0),
            },
            TestDatum {
                field_index: "c",
                sfrs: vec![
                    SelectFieldReference::factory_corr_cn("t1", "c"),
                    SelectFieldReference::factory_corr_cn("t2", "c"),
                ],
                expected_result: Err(ApllodbErrorKind::AmbiguousColumn),
            },
            TestDatum {
                field_index: "ta.c",
                sfrs: vec![SelectFieldReference::factory_corr_cn("ta", "c")],
                expected_result: Ok(0),
            },
        ];

        for test_datum in test_data {
            let field_index = FieldIndex::from(test_datum.field_index);

            let ffrs: Vec<FullFieldReference> = test_datum
                .sfrs
                .iter()
                .map(|sfr| sfr.clone().resolve_naive())
                .collect();

            match field_index.peek(&ffrs) {
                Ok((idx, ffr)) => {
                    let expected_idx = test_datum
                        .expected_result
                        .expect("succeeded in peeking, should expect Ok()");
                    assert_eq!(idx, expected_idx);
                    assert_eq!(
                        ffr,
                        test_datum.sfrs.get(idx).unwrap().clone().resolve_naive()
                    );
                }
                Err(e) => {
                    assert_eq!(e.kind(), &test_datum.expected_result.unwrap_err());
                }
            }
        }
        Ok(())
    }
}
