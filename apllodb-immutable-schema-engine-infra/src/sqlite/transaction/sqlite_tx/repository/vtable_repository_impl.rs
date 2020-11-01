use crate::sqlite::{
    sqlite_rowid::SqliteRowid,
    transaction::{
        sqlite_tx::{
            dao::{NaviDao, SqliteMasterDao, VersionDao},
            SqliteTx,
        },
        VTableDao,
    },
};
use apllodb_immutable_schema_engine_domain::{
    row::column::non_pk_column::column_name::NonPKColumnName,
    row_iter::ImmutableSchemaRowIterator,
    traits::{VTableRepository, VersionRepository},
    transaction::ImmutableSchemaTx,
    version::{active_versions::ActiveVersions, id::VersionId},
    vtable::{id::VTableId, VTable},
};
use apllodb_shared_components::error::ApllodbResult;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct VTableRepositoryImpl<'tx, 'db: 'tx> {
    tx: &'tx SqliteTx<'db>,
}

impl<'tx, 'db: 'tx> VTableRepository<'tx, 'db> for VTableRepositoryImpl<'tx, 'db> {
    type Tx = SqliteTx<'db>;

    fn new(tx: &'tx Self::Tx) -> Self {
        Self { tx }
    }

    /// # Failures
    ///
    /// - [DuplicateTable](error/enum.ApllodbErrorKind.html#variant.DuplicateTable) when:
    ///   - Table `table_name` is already visible to this transaction.
    /// - Errors from [TableDao::create()](foobar.html).
    fn create(&self, vtable: &VTable) -> ApllodbResult<()> {
        self.vtable_dao().insert(&vtable)?;
        self.navi_dao().create_table(&vtable)?;
        Ok(())
    }

    /// # Failures
    ///
    /// - [IoError](error/enum.ApllodbErrorKind.html#variant.IoError) when:
    ///   - rusqlite raises an error.
    /// - [UndefinedTable](error/enum.ApllodbErrorKind.html#variant.UndefinedTable) when:
    ///   - Table `table_name` is not visible to this transaction.
    fn read(&self, vtable_id: &VTableId) -> ApllodbResult<VTable> {
        self.vtable_dao().select(&vtable_id)
    }

    /// # Failures
    ///
    /// - [UndefinedTable](error/enum.ApllodbErrorKind.html#variant.UndefinedTable) when:
    ///   - Table `table_name` is not visible to this transaction.
    /// - [IoError](error/enum.ApllodbErrorKind.html#variant.IoError) when:
    ///   - rusqlite raises an error.
    fn update(&self, _vtable: &VTable) -> ApllodbResult<()> {
        // TODO update VTable on TableWideConstraints change.
        Ok(())
    }

    fn full_scan(
        &self,
        vtable_id: &VTableId,
        projection: &[NonPKColumnName],
    ) -> ApllodbResult<
        ImmutableSchemaRowIterator<
            <<Self::Tx as ImmutableSchemaTx<'tx, 'db>>::VRepo as VersionRepository<'tx, 'db>>::VerRowIter,
        >,
>{
        let mut ver_row_iters: VecDeque<<<Self::Tx as ImmutableSchemaTx<'tx, 'db>>::VRepo as VersionRepository<'tx, 'db>>::VerRowIter> = VecDeque::new();

        let vtable = self.vtable_dao().select(vtable_id)?;

        let navi_collection = self.navi_dao().full_scan_latest_revision(&vtable)?;

        for (version_number, navi_collection) in navi_collection.group_by_version_number()? {
            let version_id = VersionId::new(&vtable_id, &version_number);
            let version = self
                .sqlite_master_dao()
                .select_active_version(&vtable, &version_id)?;

            let ver_row_iter = self.version_dao().join_with_navi(
                &vtable,
                &version,
                &navi_collection
                    .map(|navi| navi.map(|n| n.rowid))
                    .collect::<ApllodbResult<Vec<SqliteRowid>>>()?,
                projection,
            )?;
            ver_row_iters.push_back(ver_row_iter);
        }

        Ok(ImmutableSchemaRowIterator::chain(ver_row_iters))
    }

    fn delete_all(&self, vtable: &VTable) -> ApllodbResult<()> {
        self.navi_dao().insert_deleted_records_all(&vtable)?;
        Ok(())
    }

    fn active_versions(&self, vtable: &VTable) -> ApllodbResult<ActiveVersions> {
        let active_versions = self.sqlite_master_dao().select_active_versions(vtable)?;
        Ok(ActiveVersions::from(active_versions))
    }
}

impl<'tx, 'db: 'tx> VTableRepositoryImpl<'tx, 'db> {
    fn vtable_dao(&self) -> VTableDao<'tx, 'db> {
        VTableDao::new(&self.tx)
    }

    fn version_dao(&self) -> VersionDao<'tx, 'db> {
        VersionDao::new(&self.tx)
    }

    fn navi_dao(&self) -> NaviDao<'tx, 'db> {
        NaviDao::new(&self.tx)
    }

    fn sqlite_master_dao(&self) -> SqliteMasterDao<'tx, 'db> {
        SqliteMasterDao::new(&self.tx)
    }
}
