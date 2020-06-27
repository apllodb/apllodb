use crate::sqlite::{transaction::{sqlite_tx::dao::NaviDao, VTableDao}, SqliteTx};
use apllodb_immutable_schema_engine_domain::{VTable, VTableId, VTableRepository};
use apllodb_shared_components::error::ApllodbResult;

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
    fn update(&self, vtable: &VTable) -> ApllodbResult<()> {
        // TODO update VTable on TableWideConstraints change.
        Ok(())
    }
}

impl<'tx, 'db: 'tx> VTableRepositoryImpl<'tx, 'db> {
    fn vtable_dao(&self) -> VTableDao<'tx, 'db> {
        VTableDao::new(&self.tx.sqlite_tx)
    }

    fn navi_dao(&self) -> NaviDao<'tx, 'db> {
        NaviDao::new(&self.tx.sqlite_tx)
    }
}
