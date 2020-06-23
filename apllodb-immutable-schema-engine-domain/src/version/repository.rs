use super::ActiveVersions;
use crate::{ActiveVersion, ImmutableSchemaTx, VTableId, VersionId, VersionRowIter};
use apllodb_shared_components::{
    data_structure::{ColumnName, Expression},
    error::ApllodbResult,
};
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

    /// Scan version.
    ///
    /// - Resolves each column's ColumnDataType from active versions.
    /// - Issue SELECT to `version` and get rows.
    ///
    /// # Failures
    ///
    /// - [UndefinedColumn](error/enum.ApllodbErrorKind.html#variant.UndefinedColumn) when:
    ///   - At least one `column_names` are not included in this `version`.
    fn full_scan(
        &self,
        version_id: &VersionId,
        column_names: &[ColumnName],
    ) -> ApllodbResult<Self::VerRowIter>;

    fn insert(
        &self,
        version_id: &VersionId,
        column_values: &HashMap<ColumnName, Expression>,
    ) -> ApllodbResult<()>;

    fn active_versions(&self, vtable_id: &VTableId) -> ApllodbResult<ActiveVersions>;
}
