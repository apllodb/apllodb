use crate::row::pk::apparent_pk::ApparentPrimaryKey;

use super::{active_version::ActiveVersion, id::VersionId};
use apllodb_shared_components::{ApllodbResult, ColumnName, SqlValue};
use apllodb_storage_engine_interface::StorageEngine;
use async_trait::async_trait;
use std::collections::HashMap;

#[async_trait(?Send)]
pub trait VersionRepository {
    /// Create a version.
    async fn create(&self, version: &ActiveVersion) -> ApllodbResult<()>;

    /// Deactivate a version.
    async fn deactivate(&self, version_id: &VersionId) -> ApllodbResult<()>;

    // TODO ここに version scan が現れ、 VerRowIter の型パラメータが入るのが自然

    /// # Failures
    ///
    /// - [UniqueViolation](apllodb_shared_components::ApllodbErrorKind::UniqueViolation) when:
    ///   - record with the same `apparent_pk` already exists.
    async fn insert(
        &self,
        version_id: &VersionId,
        apparent_pk: ApparentPrimaryKey,
        column_values: &HashMap<ColumnName, SqlValue>,
    ) -> ApllodbResult<()>;
}
