use crate::row::pk::apparent_pk::ApparentPrimaryKey;

use super::{active_version::ActiveVersion, id::VersionId};
use apllodb_shared_components::{ApllodbResult, ColumnName, SqlValue};
use apllodb_storage_engine_interface::StorageEngine;
use std::collections::HashMap;

pub trait VersionRepository<'sess, Engine: StorageEngine<'sess>> {
    /// Create a version.
    fn create(&self, version: &ActiveVersion) -> ApllodbResult<()>;

    /// Deactivate a version.
    fn deactivate(&self, version_id: &VersionId) -> ApllodbResult<()>;

    /// # Failures
    ///
    /// - [UniqueViolation](apllodb_shared_components::ApllodbErrorKind::UniqueViolation) when:
    ///   - record with the same `apparent_pk` already exists.
    fn insert(
        &self,
        version_id: &VersionId,
        apparent_pk: ApparentPrimaryKey,
        column_values: &HashMap<ColumnName, SqlValue>,
    ) -> ApllodbResult<()>;
}
