use super::active_version::ActiveVersion;
use crate::row::column::non_pk_column::column_name::NonPKColumnName;
use apllodb_shared_components::{
    data_structure::Expression,
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use std::collections::HashMap;

/// Collection of [ActiveVersion](x.html) sorted from latest to oldest.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct ActiveVersions(Vec<ActiveVersion>);

impl<I: IntoIterator<Item = ActiveVersion>> From<I> for ActiveVersions {
    /// Construct sorted collection.
    /// `i` need not to be sorted.
    fn from(i: I) -> Self {
        let mut v: Vec<ActiveVersion> = i.into_iter().collect();
        v.sort_by(|a, b| b.cmp(a));
        Self(v)
    }
}

impl ActiveVersions {
    pub fn as_sorted_slice(&self) -> &[ActiveVersion] {
        &self.0
    }

    /// # Failures
    ///
    /// - [UndefinedTable](error/enum.ApllodbErrorKind.html#variant.UndefinedTable) when:
    ///   - No version is active (table must be already DROPped).
    pub fn current_version(&self) -> ApllodbResult<&ActiveVersion> {
        self.0.first().ok_or_else(|| {
            ApllodbError::new(
                ApllodbErrorKind::UndefinedTable,
                "no active version found",
                None,
            )
        })
    }

    /// Returns the versions to select from.
    pub fn versions_to_select(&self) -> ApllodbResult<&[ActiveVersion]> {
        Ok(&self.0)
    }

    /// Returns the biggest version that can accept `column_values`.
    ///
    /// # Failures
    ///
    /// - [IntegrityConstraintViolation](error/enum.ApllodbErrorKind.html#variant.IntegrityConstraintViolation) when:
    ///   - No active version can accept the column value.
    pub fn version_to_insert(
        &self,
        non_pk_column_values: &HashMap<NonPKColumnName, Expression>,
    ) -> ApllodbResult<&ActiveVersion> {
        if self.0.is_empty() {
            return Err(ApllodbError::new(
                ApllodbErrorKind::UndefinedTable,
                "no active version found",
                None,
            ));
        }

        // FIXME use `map_while` after it is stabilized: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map_while
        let mut errors_per_versions: Vec<(&ActiveVersion, ApllodbError)> = Vec::new();
        for version in &self.0 {
            if let Err(e) = version.check_version_constraint(non_pk_column_values) {
                errors_per_versions.push((version, e));
            } else {
                return Ok(version);
            }
        }

        // summarize errors

        // none version has a specified column.
        if errors_per_versions.iter().map(|(_, e)| e.kind()).all(|k| {
            if let ApllodbErrorKind::UndefinedColumn = k {
                true
            } else {
                false
            }
        }) {
            Err(ApllodbError::new(
                ApllodbErrorKind::UndefinedColumn,
                format!(
                    "at least 1 column does not exist in any version: {:?}",
                    errors_per_versions,
                ),
                None,
            ))
        } else {
            Err(ApllodbError::new(
                ApllodbErrorKind::IntegrityConstraintViolation,
                format!(
                    "all versions reject INSERTing {:?}: {:?}",
                    non_pk_column_values, errors_per_versions,
                ),
                None,
            ))
        }
    }
}
