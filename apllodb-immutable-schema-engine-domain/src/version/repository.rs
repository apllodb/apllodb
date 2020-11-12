use super::{active_version::ActiveVersion, id::VersionId};
use crate::{
    abstract_types::ImmutableSchemaAbstractTypes,
    row::{
        column::non_pk_column::column_name::NonPKColumnName, pk::apparent_pk::ApparentPrimaryKey,
    },
};
use apllodb_shared_components::{data_structure::Expression, error::ApllodbResult};
use apllodb_storage_engine_interface::StorageEngine;
use std::collections::HashMap;

pub trait VersionRepository<
    'tx,
    'db: 'tx,
    Engine: StorageEngine,
    Types: ImmutableSchemaAbstractTypes<'tx, 'db, Engine>,
>
{
    fn new(tx: &'tx Engine::Tx) -> Self;

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
        column_values: &HashMap<NonPKColumnName, Expression>,
    ) -> ApllodbResult<()>;
}
