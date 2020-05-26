use crate::{storage::Storage, Table};
use apllodb_shared_components::{data_structure::TableName, error::ApllodbResult};
use apllodb_storage_manager_interface::TxCtxLike;

/// Simple (Naive) ACID transaction implementation for Serializable isolation level.
///
/// Used with not only DML but also DDL.
///
/// - **Concurrency control** : SSPL (Strong Strict Two Phase Lock).
/// - **Lock target** : table.
/// - **Dead lock prevention** : No-Wait.
#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) struct SimpleTx<S: Storage> {
    storage: S,
}

impl<S: Storage> TxCtxLike for SimpleTx<S> {
    fn begin() -> ApllodbResult<Self>
    where
        Self: Sized,
    {
        todo!()
    }

    /// # Failures
    ///
    /// - Errors from [Storage::make_durable](foobar.html).
    fn commit(mut self) -> ApllodbResult<()> {
        self.storage.make_durable()?;

        self.unlock_all();

        todo!()
    }

    fn abort(mut self) -> ApllodbResult<()> {
        self.unlock_all();

        todo!()
    }
}

impl<S: Storage> SimpleTx<S> {
    /// # Failures
    ///
    /// - [TransactionRollback](error/enum.ApllodbErrorKind.html#variant.TransactionRollback) when:
    ///   - Lock for `table_name` is already acquired by another transaction.
    fn lock_or_abort(&mut self, _table_name: &TableName) -> ApllodbResult<()> {
        todo!()
    }

    fn unlock_all(&mut self) {
        todo!()
    }
}

impl<S: Storage> SimpleTx<S> {
    /// # Failures
    ///
    /// - [TransactionRollback](error/enum.ApllodbErrorKind.html#variant.TransactionRollback) when:
    ///   - Lock for `table_name` is already acquired by another transaction.
    pub(crate) fn read_table(&mut self, table_name: &TableName) -> ApllodbResult<Table> {
        self.lock_or_abort(table_name)?;
        todo!()
    }

    /// # Failures
    ///
    /// - [TransactionRollback](error/enum.ApllodbErrorKind.html#variant.TransactionRollback) when:
    ///   - Lock for `table` is already acquired by another transaction.
    pub(crate) fn write_table(&mut self, table: Table) -> ApllodbResult<()> {
        self.lock_or_abort(table.name())?;
        todo!()
    }
}
