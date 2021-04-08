use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::{
    data_structure::reference::correlation_reference::correlation_index::CorrelationIndex,
    ApllodbError, ApllodbErrorKind, ApllodbResult, CorrelationReference, FieldReference,
    FullFieldReference,
};

use super::record_pos::RecordPos;

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
    correlation_index: Option<CorrelationIndex>,
    field_name: String,
}

impl FieldIndex {
    /// Find a FullFieldReference which matches this FieldIndex.
    ///
    /// # Failures
    ///
    /// - [InvalidName](crate::ApllodbErrorKind::InvalidName) when:
    ///   - none of `full_field_references` matches to this FieldIndex.
    /// - [AmbiguousColumn](crate::ApllodbErrorKind::AmbiguousColumn) when:
    ///   - more than 1 of `full_field_references` match to this FieldIndex.
    pub fn peek<'a>(
        &self,
        full_field_references: impl IntoIterator<Item = &'a FullFieldReference>,
    ) -> ApllodbResult<(RecordPos, &'a FullFieldReference)> {
        let mut ret_pos = RecordPos::new(0);
        let mut ret_ffr: Option<&'a FullFieldReference> = None;

        for ffr in full_field_references {
            if self.matches(ffr) {
                if ret_ffr.is_some() {
                    return Err(ApllodbError::new(
                        ApllodbErrorKind::AmbiguousColumn,
                        format!(
                            "field index `{}` match to more than 1 of full_field_references",
                            self
                        ),
                        None,
                    ));
                } else {
                    ret_ffr = Some(ffr);
                }
            }
            if ret_ffr.is_none() {
                ret_pos.inc();
            }
        }

        if ret_ffr.is_none() {
            return Err(ApllodbError::new(
                ApllodbErrorKind::InvalidName,
                format!(
                    "field index `{}` does not match any of full_field_references",
                    self
                ),
                None,
            ));
        }

        Ok((ret_pos, ret_ffr.unwrap()))
    }

    fn matches(&self, full_field_reference: &FullFieldReference) -> bool {
        match (
            &self.correlation_index,
            full_field_reference.as_correlation_reference(),
            full_field_reference.as_field_reference(),
        ) {
            (
                // index: c
                // ffr: (T (AS TA)?).C
                None,
                _,
                FieldReference::ColumnNameVariant(cn),
            ) => self.field_name == cn.as_str(),

            (
                // index: c
                // ffr: (T (AS TA)?).C AS CA
                None,
                _,
                FieldReference::ColumnAliasVariant {
                    alias_name,
                    column_name,
                },
            ) => self.field_name == column_name.as_str() || self.field_name == alias_name.as_str(),

            (
                // index: t.c
                // ffr: (T (AS TA)?).C
                Some(self_correlation_index),
                corr_ref,
                FieldReference::ColumnNameVariant(column_name),
            ) => {
                self_correlation_index.matches(corr_ref) && self.field_name == column_name.as_str()
            }
            (
                // index: t.c
                // ffr: (T (AS TA)?).C AS CA
                Some(self_correlation_name),
                corr_ref,
                FieldReference::ColumnAliasVariant {
                    column_name,
                    alias_name,
                },
            ) => {
                self_correlation_name.matches(corr_ref)
                    && (self.field_name == column_name.as_str()
                        || self.field_name == alias_name.as_str())
            }
        }
    }
}

impl Display for FieldIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pre = if let Some(corr) = &self.correlation_index {
            format!("{}.", corr)
        } else {
            "".to_string()
        };
        write!(f, "{}{}", pre, self.field_name)
    }
}

impl<S: Into<String>> From<S> for FieldIndex {
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
                correlation_index: Some(CorrelationIndex::from(first)),
                field_name: second,
            }
        } else {
            Self {
                correlation_index: None,
                field_name: first,
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
            (
                CorrelationReference::TableNameVariant(table_name),
                FieldReference::ColumnNameVariant(column_name),
            )
            | (
                CorrelationReference::TableNameVariant(table_name),
                FieldReference::ColumnAliasVariant { column_name, .. },
            )
            | (
                CorrelationReference::TableAliasVariant { table_name, .. },
                FieldReference::ColumnNameVariant(column_name),
            )
            | (
                CorrelationReference::TableAliasVariant { table_name, .. },
                FieldReference::ColumnAliasVariant { column_name, .. },
            ) => Self::from(format!("{}.{}", table_name.as_str(), column_name.as_str())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ApllodbErrorKind, FieldIndex, FullFieldReference, RecordPos};

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
    fn test_peek() {
        struct TestDatum {
            field_index: &'static str,
            full_field_references: Vec<FullFieldReference>,
            expected_result: Result<
                usize, // index to matching one from `full_field_references`,
                ApllodbErrorKind,
            >,
        }

        let test_data: Vec<TestDatum> = vec![
            TestDatum {
                field_index: "c",
                full_field_references: vec![],
                expected_result: Err(ApllodbErrorKind::InvalidName),
            },
            TestDatum {
                field_index: "c",
                full_field_references: vec![FullFieldReference::factory("t", "c")],
                expected_result: Ok(0),
            },
            TestDatum {
                field_index: "xxx",
                full_field_references: vec![FullFieldReference::factory("t", "c")],
                expected_result: Err(ApllodbErrorKind::InvalidName),
            },
            TestDatum {
                field_index: "c",
                full_field_references: vec![
                    FullFieldReference::factory("t", "c").with_field_alias("ca")
                ],
                expected_result: Ok(0),
            },
            TestDatum {
                field_index: "ca",
                full_field_references: vec![
                    FullFieldReference::factory("t", "c").with_field_alias("ca")
                ],
                expected_result: Ok(0),
            },
            TestDatum {
                field_index: "t.ca",
                full_field_references: vec![
                    FullFieldReference::factory("t", "c").with_field_alias("ca")
                ],
                expected_result: Ok(0),
            },
            TestDatum {
                field_index: "xxx",
                full_field_references: vec![
                    FullFieldReference::factory("t", "c").with_field_alias("ca")
                ],
                expected_result: Err(ApllodbErrorKind::InvalidName),
            },
            TestDatum {
                field_index: "t.c",
                full_field_references: vec![FullFieldReference::factory("t", "c")],
                expected_result: Ok(0),
            },
            TestDatum {
                field_index: "xxx.c",
                full_field_references: vec![FullFieldReference::factory("t", "c")],
                expected_result: Err(ApllodbErrorKind::InvalidName),
            },
            TestDatum {
                field_index: "c",
                full_field_references: vec![
                    FullFieldReference::factory("t", "c").with_corr_alias("ta")
                ],
                expected_result: Ok(0),
            },
            TestDatum {
                field_index: "t.c",
                full_field_references: vec![
                    FullFieldReference::factory("t", "c").with_corr_alias("ta")
                ],
                expected_result: Ok(0),
            },
            TestDatum {
                field_index: "ta.c",
                full_field_references: vec![
                    FullFieldReference::factory("t", "c").with_corr_alias("ta")
                ],
                expected_result: Ok(0),
            },
            TestDatum {
                field_index: "c",
                full_field_references: vec![FullFieldReference::factory("t", "c")
                    .with_corr_alias("ta")
                    .with_field_alias("ca")],
                expected_result: Ok(0),
            },
            TestDatum {
                field_index: "ca",
                full_field_references: vec![FullFieldReference::factory("t", "c")
                    .with_corr_alias("ta")
                    .with_field_alias("ca")],
                expected_result: Ok(0),
            },
            TestDatum {
                field_index: "t.c",
                full_field_references: vec![FullFieldReference::factory("t", "c")
                    .with_corr_alias("ta")
                    .with_field_alias("ca")],
                expected_result: Ok(0),
            },
            TestDatum {
                field_index: "ta.c",
                full_field_references: vec![FullFieldReference::factory("t", "c")
                    .with_corr_alias("ta")
                    .with_field_alias("ca")],
                expected_result: Ok(0),
            },
            TestDatum {
                field_index: "t.ca",
                full_field_references: vec![FullFieldReference::factory("t", "c")
                    .with_corr_alias("ta")
                    .with_field_alias("ca")],
                expected_result: Ok(0),
            },
            TestDatum {
                field_index: "ta.ca",
                full_field_references: vec![FullFieldReference::factory("t", "c")
                    .with_corr_alias("ta")
                    .with_field_alias("ca")],
                expected_result: Ok(0),
            },
            TestDatum {
                field_index: "c2",
                full_field_references: vec![
                    FullFieldReference::factory("t1", "c1"),
                    FullFieldReference::factory("t2", "c2"),
                ],
                expected_result: Ok(1),
            },
            TestDatum {
                field_index: "t1.c1",
                full_field_references: vec![
                    FullFieldReference::factory("t1", "c1"),
                    FullFieldReference::factory("t2", "c2"),
                ],
                expected_result: Ok(0),
            },
            TestDatum {
                field_index: "t1.c",
                full_field_references: vec![
                    FullFieldReference::factory("t1", "c"),
                    FullFieldReference::factory("t2", "c"),
                ],
                expected_result: Ok(0),
            },
            TestDatum {
                field_index: "c",
                full_field_references: vec![
                    FullFieldReference::factory("t1", "c"),
                    FullFieldReference::factory("t2", "c"),
                ],
                expected_result: Err(ApllodbErrorKind::AmbiguousColumn),
            },
        ];

        for test_datum in test_data {
            let field_index = FieldIndex::from(test_datum.field_index);
            match field_index.peek(test_datum.full_field_references.iter().as_ref()) {
                Ok((pos, ffr)) => {
                    let expected_pos = test_datum
                        .expected_result
                        .expect("succeeded in peeking, should expect Ok()");
                    assert_eq!(pos, RecordPos::new(expected_pos));
                    assert_eq!(
                        ffr,
                        test_datum
                            .full_field_references
                            .get(pos.to_usize())
                            .unwrap()
                    );
                }
                Err(e) => {
                    assert_eq!(e.kind(), &test_datum.expected_result.unwrap_err());
                }
            }
        }
    }
}
