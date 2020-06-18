use crate::{ActiveVersion, ImmutableSchemaTx, VersionId, VersionRowIter};
use apllodb_shared_components::{data_structure::ColumnName, error::ApllodbResult};

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
}
