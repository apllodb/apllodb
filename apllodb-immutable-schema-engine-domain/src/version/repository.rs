use super::ActiveVersions;
use crate::{ActiveVersion, ImmutableSchemaTx, VTableId, VersionId, VersionRowIter};
use apllodb_shared_components::{
    data_structure::ColumnName,
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};

pub trait VersionRepository<'tx, 'db: 'tx> {
    type Tx: ImmutableSchemaTx<'tx, 'db>;

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
        version: &VersionId,
        column_names: &[ColumnName],
    ) -> ApllodbResult<Self::VerRowIter>;

    fn active_versions(&self, vtable_id: &VTableId) -> ApllodbResult<ActiveVersions>;

    /// # Failures
    ///
    /// - [UndefinedTable](error/enum.ApllodbErrorKind.html#variant.UndefinedTable) when:
    ///   - No version is active (table must be already DROPped).
    fn current_version(&self, vtable_id: &VTableId) -> ApllodbResult<ActiveVersion> {
        self.active_versions(vtable_id)?
            .as_sorted_slice()
            .first()
            .ok_or_else(|| {
                ApllodbError::new(
                    ApllodbErrorKind::UndefinedTable,
                    "no active version found",
                    None,
                )
            })
            .map(|v| v.clone())
    }
}
