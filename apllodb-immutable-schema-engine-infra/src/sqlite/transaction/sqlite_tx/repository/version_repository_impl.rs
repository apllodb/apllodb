use crate::sqlite::{transaction::sqlite_tx::dao::VersionDao, SqliteRowIterator, SqliteTx};
use apllodb_immutable_schema_engine_domain::{
    ActiveVersion, ActiveVersions, VTableId, VersionId, VersionRepository,
};
use apllodb_shared_components::{
    data_structure::{ColumnName, Expression},
    error::ApllodbResult,
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct VersionRepositoryImpl<'tx, 'db: 'tx> {
    tx: &'tx SqliteTx<'db>,
}

impl<'tx, 'db: 'tx> VersionRepository<'tx, 'db> for VersionRepositoryImpl<'tx, 'db> {
    type Tx = SqliteTx<'db>;
    type VerRowIter = SqliteRowIterator;

    fn new(tx: &'tx Self::Tx) -> Self {
        Self { tx }
    }

    /// # Failures
    ///
    /// - [DuplicateTable](error/enum.ApllodbErrorKind.html#variant.DuplicateTable) when:
    ///   - Table `table_name` is already visible to this transaction.
    /// - Errors from [TableDao::create()](foobar.html).
    fn create(&self, version: &ActiveVersion) -> ApllodbResult<()> {
        self.version_dao().create(&version)?;
        Ok(())
    }

    fn deactivate(&self, version_id: &VersionId) -> ApllodbResult<()> {
        todo!()
    }

    fn full_scan(
        &self,
        version_id: &VersionId,
        column_names: &[ColumnName],
    ) -> ApllodbResult<Self::VerRowIter> {
        let version_row_iter = self.version_dao().full_scan(&version_id, &column_names)?;
        Ok(version_row_iter)
    }

    fn insert(
        &self,
        version_id: &VersionId,
        column_values: &HashMap<ColumnName, Expression>,
    ) -> ApllodbResult<()> {
        self.version_dao().insert(&version_id, &column_values)?;
        Ok(())
    }

    fn active_versions(&self, vtable_id: &VTableId) -> ApllodbResult<ActiveVersions> {
        let active_versions = self.version_dao().select_active_versions(vtable_id)?;
        Ok(ActiveVersions::from(active_versions))
    }
}

impl<'tx, 'db: 'tx> VersionRepositoryImpl<'tx, 'db> {
    fn version_dao(&self) -> VersionDao<'tx, 'db> {
        VersionDao::new(&self.tx.sqlite_tx)
    }
}
