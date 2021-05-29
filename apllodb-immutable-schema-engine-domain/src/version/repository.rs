use crate::row::pk::apparent_pk::ApparentPrimaryKey;

use super::{active_version::ActiveVersion, id::VersionId};
use apllodb_shared_components::{ApllodbResult, SqlValue};
use apllodb_storage_engine_interface::ColumnName;
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait(?Send)]
pub trait VersionRepository {
    /// Create a version.
    async fn create(&self, version: &ActiveVersion) -> ApllodbResult<()>;

    /// Deactivate a version.
    async fn deactivate(&self, version_id: &VersionId) -> ApllodbResult<()>;

    /// # Failures
    ///
    /// - [IntegrityConstraintUniqueViolation](apllodb_shared_components::SqlState::IntegrityConstraintUniqueViolation) when:
    ///   - record with the same `apparent_pk` already exists.
    async fn insert(
        &self,
        version_id: &VersionId,
        apparent_pk: ApparentPrimaryKey,
        column_values: &HashMap<ColumnName, SqlValue>,
    ) -> ApllodbResult<()>;
}
