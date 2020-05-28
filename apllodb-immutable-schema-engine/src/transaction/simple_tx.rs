mod simple_storage;

use crate::Table;
use apllodb_shared_components::{
    data_structure::TableName,
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use apllodb_storage_manager_interface::TxCtxLike;
use parking_lot::ReentrantMutexGuard;
use simple_storage::{SimpleStorage, TableRwToken};

/// Simple (Naive) ACID transaction implementation for Serializable isolation level.
///
/// Used with not only DML but also DDL.
///
/// - **Concurrency control** : SSPL (Strong Strict Two Phase Lock).
/// - **Lock target** : table.
/// - **Dead lock prevention** : No-Wait.
#[derive(Debug)]
pub(crate) struct SimpleTx<'st> {
    storage: &'st SimpleStorage,
}

impl<'st> TxCtxLike<'st> for SimpleTx<'st> {
    type Storage = SimpleStorage;

    fn begin(storage: &'st Self::Storage) -> ApllodbResult<Self>
    where
        Self: Sized,
    {
        Ok(Self { storage })
    }

    /// # Failures
    ///
    /// - Errors from [Storage::make_durable](foobar.html).
    fn commit(&mut self) -> ApllodbResult<()> {
        self.storage.flush_objects_atomically()?;

        self.unlock_all();

        todo!()
    }

    fn abort(&mut self) -> ApllodbResult<()> {
        self.unlock_all();

        todo!()
    }
}

impl<'st> SimpleTx<'st> {
    fn safe_abort(&mut self) {
        self.abort()
            .expect("SimpleTx::abort() is expected to always succeed.")
    }

    /// Try-lock to table `table_name`.
    ///
    /// This method is reentrant in that it succeeds if lock is already acquired by myself.
    ///
    /// # Failures
    ///
    /// - [TransactionRollback](error/enum.ApllodbErrorKind.html#variant.TransactionRollback) when:
    ///   - Lock for `table_name` is already acquired by another transaction.
    fn lock_or_abort(
        &mut self,
        table_name: &TableName,
    ) -> ApllodbResult<ReentrantMutexGuard<TableRwToken>> {
        match self.storage.try_lock(table_name) {
            Some(token_in_guard) => Ok(token_in_guard),
            None => {
                self.safe_abort();
                Err(ApllodbError::new(
                    ApllodbErrorKind::TransactionRollback,
                    format!("failed to try-lock table {}", table_name),
                    None,
                ))
            }
        }
    }

    fn unlock_all(&mut self) {
        todo!()
    }
}

impl<'st> SimpleTx<'st> {
    /// # Failures
    ///
    /// - [TransactionRollback](error/enum.ApllodbErrorKind.html#variant.TransactionRollback) when:
    ///   - Lock for `table_name` is already acquired by another transaction.
    pub(crate) fn read_table(&mut self, table_name: &TableName) -> ApllodbResult<Table> {
        let storage = self.storage; // freezing technique

        let token_in_guard = self.lock_or_abort(table_name)?;
        let table_obj = storage.load_table(&*token_in_guard)?;
        Ok(table_obj.as_table().clone())
    }

    /// # Failures
    ///
    /// - [TransactionRollback](error/enum.ApllodbErrorKind.html#variant.TransactionRollback) when:
    ///   - Lock for `table` is already acquired by another transaction.
    pub(crate) fn write_table(&mut self, table: Table) -> ApllodbResult<()> {
        let storage = self.storage; // freezing technique

        let token_in_guard = self.lock_or_abort(table.name())?;
        let table_obj = storage.load_table(&*token_in_guard)?;
        table_obj.update_by(table);
        Ok(())
    }
}
