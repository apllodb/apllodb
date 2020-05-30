mod id;
pub(crate) mod lock_manager;
mod objects;
mod simple_storage;

use crate::{latch::Latch, Database, Table};
use apllodb_shared_components::{
    data_structure::TableName,
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use apllodb_storage_manager_interface::TxCtxLike;
use id::SimpleTxId;
use lock_manager::{LockManager, TableRwToken};
use objects::TableObj;
use simple_storage::SimpleStorage;
use std::{cmp::Ordering, sync::Arc};

/// Simple (Naive) ACID transaction implementation for Serializable isolation level.
///
/// Used with not only DML but also DDL.
///
/// - **Concurrency control** : SS2PL (Strong Strict Two Phase Lock).
/// - **Lock target** : table.
/// - **Dead lock prevention** : No-Wait.
#[derive(Debug)]
pub(crate) struct SimpleTx {
    id: SimpleTxId,

    loaded_tables: Vec<TableObj>,

    // Singleton (at most 1 per database) instance of LockManager.
    // Arc since many SimpleTx instances share the same LockManager instance.
    // Latch since a SimpleTx instance should exclusively borrow mutable reference to LockManager at a time to lock/unlock.
    lock_manager: Arc<Latch<LockManager>>,
}

impl PartialEq for SimpleTx {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for SimpleTx {}

impl Ord for SimpleTx {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}
impl PartialOrd for SimpleTx {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl TxCtxLike for SimpleTx {
    type DbCtx = Database;

    fn begin(db: &Self::DbCtx) -> ApllodbResult<Self>
    where
        Self: std::marker::Sized,
    {
        Ok(Self {
            id: SimpleTxId::new(),
            loaded_tables: vec![],
            lock_manager: db.lock_manager.clone(),
        })
    }

    /// # Failures
    ///
    /// - Errors from [Storage::make_durable](foobar.html).
    fn commit(&mut self) -> ApllodbResult<()> {
        SimpleStorage::flush_objects_atomically(self.loaded_tables.drain(..).collect())?;

        self.unlock_all();

        todo!()
    }

    fn abort(&mut self) -> ApllodbResult<()> {
        self.unlock_all();

        todo!()
    }
}

impl SimpleTx {
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
    fn lock_or_abort(&mut self, table_name: &TableName) -> ApllodbResult<TableRwToken> {
        self.lock_manager
            .with_lock(|lm| lm.reentrant_try_lock(table_name, self.id))
            .ok_or_else(|| {
                self.safe_abort();
                ApllodbError::new(
                    ApllodbErrorKind::TransactionRollback,
                    format!("failed to try-lock table {}", table_name),
                    None,
                )
            })
    }

    fn unlock_all(&mut self) {
        self.lock_manager.with_lock(|lm| lm.unlock_all(self.id))
    }

    /// # Failures
    ///
    /// - [TransactionRollback](error/enum.ApllodbErrorKind.html#variant.TransactionRollback) when:
    ///   - Lock for `table_name` is already acquired by another transaction.
    pub(crate) fn read_table(&mut self, table_name: &TableName) -> ApllodbResult<Table> {
        let token = self.lock_or_abort(table_name)?;
        let table_obj = SimpleStorage::load_table(&token)?;
        Ok(table_obj.as_table().clone())
    }

    /// # Failures
    ///
    /// - [TransactionRollback](error/enum.ApllodbErrorKind.html#variant.TransactionRollback) when:
    ///   - Lock for `table` is already acquired by another transaction.
    pub(crate) fn write_table(&mut self, table: Table) -> ApllodbResult<()> {
        let token = self.lock_or_abort(table.name())?;
        let mut table_obj = SimpleStorage::load_table(&token)?;
        table_obj.update_by(table);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::SimpleTx;
    use crate::{
        column_constraints, column_definition, column_definitions, table_constraints, table_name,
        AccessMethods, Database,
    };
    use apllodb_shared_components::error::{ApllodbErrorKind, ApllodbResult};
    use apllodb_storage_manager_interface::{AccessMethodsDdl, TxCtxLike};

    #[test]
    fn test_no_wait() -> ApllodbResult<()> {
        let db = Database::new();

        let tn = &table_name!("t");
        let tc = table_constraints!();
        let coldefs = column_definitions!(column_definition!("c1", column_constraints!()));

        let mut tx1 = SimpleTx::begin(&db)?;
        let mut tx2 = SimpleTx::begin(&db)?;

        AccessMethods::create_table(&mut tx1, &tn, &tc, &coldefs)?;

        match AccessMethods::create_table(&mut tx2, &tn, &tc, &coldefs) {
            Err(e) => assert_eq!(*e.kind(), ApllodbErrorKind::TransactionRollback),
            Ok(_) => panic!("should rollback"),
        }

        Ok(())
    }

    // reentrant

    // acid
    // iは無理。no-waitなので
    // aはむずいけど試したい
    // c...?
}
