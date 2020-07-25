use crate::{
    row::column::non_pk_column::NonPKColumnName, ActiveVersion, ApparentPrimaryKey,
    ImmutableSchemaTx, VersionId, VersionRowIter,
};
use apllodb_shared_components::{data_structure::Expression, error::ApllodbResult};
use apllodb_storage_engine_interface::TransactionId;
use std::collections::HashMap;

pub trait VersionRepository<'tx, 'db: 'tx> {
    type Tx: ImmutableSchemaTx<'tx, 'db>;
    type TID: TransactionId;

    /// Row iterator from a single version.
    type VerRowIter: VersionRowIter;

    fn new(tx: &'tx Self::Tx) -> Self;

    /// Create a version.
    fn create(&self, version: &ActiveVersion) -> ApllodbResult<()>;

    /// Deactivate a version.
    fn deactivate(&self, version_id: &VersionId) -> ApllodbResult<()>;

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
