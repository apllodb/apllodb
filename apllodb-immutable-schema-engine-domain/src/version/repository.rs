use crate::row::pk::apparent_pk::ApparentPrimaryKey;

use super::{active_version::ActiveVersion, id::VersionId};
use apllodb_shared_components::{ApllodbResult, ColumnName, Expression};
use apllodb_storage_engine_interface::StorageEngine;
use std::collections::HashMap;

pub trait VersionRepository<'repo, 'db: 'repo, Engine: StorageEngine<'repo, 'db>> {
    fn new(tx: &'repo Engine::Tx) -> Self;

    /// Create a version.
    fn create(&self, version: &ActiveVersion) -> ApllodbResult<()>;

    /// Deactivate a version.
    fn deactivate(&self, version_id: &VersionId) -> ApllodbResult<()>;

    // TODO ここに version scan が現れ、 VerRowIter の型パラメータが入るのが自然

    /// # Failures
    ///
    /// - [UniqueViolation](error/enum.ApllodbErrorKind.html#variant.UniqueViolation) when:
    ///   - record with the same `apparent_pk` already exists.
    fn insert(
        &self,
        version_id: &VersionId,
        apparent_pk: ApparentPrimaryKey,
        column_values: &HashMap<ColumnName, Expression>,
    ) -> ApllodbResult<()>;
}
