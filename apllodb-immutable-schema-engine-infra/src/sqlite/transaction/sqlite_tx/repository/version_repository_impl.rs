use crate::sqlite::{
    transaction::{
        sqlite_tx::dao::{Navi, NaviDao, SqliteMasterDao, VersionDao},
        TxId,
    },
    SqliteRowIterator, SqliteTx,
};
use apllodb_immutable_schema_engine_domain::{
    ActiveVersion, ActiveVersions, ApparentPrimaryKey, Revision, VTableId, VersionId,
    VersionRepository,
};
use apllodb_shared_components::{
    data_structure::{ColumnName, Expression},
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use std::collections::HashMap;

#[derive(Debug)]
pub struct VersionRepositoryImpl<'tx, 'db: 'tx> {
    tx: &'tx SqliteTx<'db>,
}

impl<'tx, 'db: 'tx> VersionRepository<'tx, 'db> for VersionRepositoryImpl<'tx, 'db> {
    type Tx = SqliteTx<'db>;
    type TID = TxId;
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
        self.version_dao().create_table(&version)?;
        Ok(())
    }

    fn deactivate(&self, _version_id: &VersionId) -> ApllodbResult<()> {
        todo!()
    }

    fn full_scan(
        &self,
        version_id: &VersionId,
        column_names: &[ColumnName],
    ) -> ApllodbResult<Self::VerRowIter> {
        let version = self
            .sqlite_master_dao()
            .select_active_version(&version_id)?;
        let version_row_iter = self.version_dao().full_scan(&version, &column_names)?;
        Ok(version_row_iter)
    }

    fn insert(
        &self,
        version_id: &VersionId,
        apparent_pk: ApparentPrimaryKey,
        column_values: &HashMap<ColumnName, Expression>,
    ) -> ApllodbResult<()> {
        let revision = match self
            .navi_dao()
            .probe(version_id.vtable_id(), &apparent_pk)?
        {
            Navi::Exist { .. } => Err(ApllodbError::new(
                ApllodbErrorKind::UniqueViolation,
                format!(
                    "record with the same primary key already exists: {:?}",
                    apparent_pk
                ),
                None,
            )),
            Navi::NotExist => Ok(Revision::initial()),
            Navi::Deleted { revision, .. } => Ok(revision.next()),
        }?;

        let rowid = self.navi_dao().insert(apparent_pk, revision, &version_id)?;

        self.version_dao()
            .insert(&version_id, rowid, &column_values)?;
        Ok(())
    }

    fn active_versions(&self, vtable_id: &VTableId) -> ApllodbResult<ActiveVersions> {
        let active_versions = self.sqlite_master_dao().select_active_versions(vtable_id)?;
        Ok(ActiveVersions::from(active_versions))
    }
}

impl<'tx, 'db: 'tx> VersionRepositoryImpl<'tx, 'db> {
    fn version_dao(&self) -> VersionDao<'tx, 'db> {
        VersionDao::new(&self.tx.rusqlite_tx)
    }

    fn navi_dao(&self) -> NaviDao<'tx, 'db> {
        NaviDao::new(&self.tx.rusqlite_tx)
    }

    fn sqlite_master_dao(&self) -> SqliteMasterDao<'tx, 'db> {
        SqliteMasterDao::new(&self.tx.rusqlite_tx)
    }
}
