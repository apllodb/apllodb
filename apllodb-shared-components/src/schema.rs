pub(crate) mod r_pos;
pub(crate) mod schema_index;
pub(crate) mod schema_name;

use crate::{ApllodbError, ApllodbErrorKind, ApllodbResult, RPos};

use self::{schema_index::SchemaIndex, schema_name::SchemaName};

/// Schema defines structure of records / rows.
///
/// Main purpose of schema is to resolve fields' / columns' position in records / rows to extract values from them.
///
/// While rows, used in storage-engine, consist of tables' column values,
/// records have higher level of fields like field references, constants, and operations.
///
/// So a schema for rows consist of sequence of "table_name.column_name" but a schema for records may include unnamed field.
pub trait Schema {
    /// Field's / Column's full name.
    type Name: SchemaName;

    /// Default constructor
    fn new(names_with_pos: Vec<(RPos, Option<Self::Name>)>) -> Self
    where
        Self: Sized;

    /// Finds a pair of (RPos, Name) of a field/column specified by Index.
    ///
    /// # Failures
    ///
    /// - [InvalidName](crate::ApllodbErrorKind::InvalidName) when:
    ///   - no field matches to this Index.
    /// - [AmbiguousColumn](crate::ApllodbErrorKind::AmbiguousColumn) when:
    ///   - more than 1 of fields match to this Index.
    fn index(&self, idx: &SchemaIndex) -> ApllodbResult<(RPos, Self::Name)> {
        let matching_pair: Vec<(RPos, Self::Name)> = self
            .names_with_pos()
            .iter()
            .filter_map(|(pos, opt_name)| {
                opt_name
                    .as_ref()
                    .map(|name| {
                        if name.matches(idx) {
                            Some((*pos, name.clone()))
                        } else {
                            None
                        }
                    })
                    .flatten()
            })
            .collect();

        if matching_pair.len() == 1 {
            matching_pair.first().cloned().ok_or_else(|| unreachable!())
        } else if matching_pair.is_empty() {
            Err(ApllodbError::new(
                ApllodbErrorKind::InvalidName,
                format!("no field matches to: {}", idx),
                None,
            ))
        } else {
            Err(ApllodbError::new(
                ApllodbErrorKind::AmbiguousColumn,
                format!("more than 1 fields match to: {}", idx),
                None,
            ))
        }
    }

    /// Filter specified fields
    fn projection(&self, indexes: &[SchemaIndex]) -> ApllodbResult<Self>
    where
        Self: Sized,
    {
        let new_inner: Vec<(RPos, Option<Self::Name>)> = indexes
            .iter()
            .map(|index| {
                let (pos, name) = self.index(index)?;
                Ok((pos, Some(name)))
            })
            .collect::<ApllodbResult<_>>()?;
        Ok(Self::new(new_inner))
    }

    /// Pairs of (RPos, Name).
    fn names_with_pos(&self) -> Vec<(RPos, Option<Self::Name>)>;
}