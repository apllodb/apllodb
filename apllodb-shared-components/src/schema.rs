pub(crate) mod r_pos;
pub(crate) mod schema_index;
pub(crate) mod schema_name;

use std::collections::HashSet;

use crate::{ApllodbError, ApllodbResult, RPos};

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
    fn new(names: HashSet<Self::Name>, unnamed_fields_len: usize) -> Self
    where
        Self: Sized;

    /// Finds a pair of (RPos, Name) of a field/column specified by Index.
    ///
    /// # Failures
    ///
    /// - [NameErrorNotFound](crate::SqlState::NameErrorNotFound) when:
    ///   - no field matches to this Index.
    /// - [AmbiguousColumn](crate::SqlState::AmbiguousColumn) when:
    ///   - more than 1 of fields match to this Index.
    fn index(&self, idx: &SchemaIndex) -> ApllodbResult<(RPos, Self::Name)> {
        let matching_pair: Vec<(RPos, Self::Name)> = self
            .names_with_pos()
            .iter()
            .filter_map(|(pos, opt_name)| {
                opt_name
                    .as_ref()
                    .map(|name| name.matches(idx).then(|| (*pos, name.clone())))
                    .flatten()
            })
            .collect();

        if matching_pair.len() == 1 {
            matching_pair.first().cloned().ok_or_else(|| unreachable!())
        } else if matching_pair.is_empty() {
            Err(ApllodbError::new(
                SqlState::NameErrorNotFound,
                format!("no field matches to: {}", idx),
                None,
            ))
        } else {
            Err(ApllodbError::new(
                SqlState::AmbiguousColumn,
                format!("more than 1 fields match to: {}", idx),
                None,
            ))
        }
    }

    /// Length
    fn len(&self) -> usize;

    /// is empty?
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Filter specified fields
    fn projection(&self, indexes: &HashSet<SchemaIndex>) -> ApllodbResult<Self>
    where
        Self: Sized,
    {
        let new_inner: HashSet<Self::Name> = indexes
            .iter()
            .map(|index| {
                let (_, name) = self.index(index)?;
                Ok(name)
            })
            .collect::<ApllodbResult<_>>()?;
        Ok(Self::new(new_inner, 0))
    }

    /// Pairs of (RPos, Name).
    fn names_with_pos(&self) -> Vec<(RPos, Option<Self::Name>)>;
}
