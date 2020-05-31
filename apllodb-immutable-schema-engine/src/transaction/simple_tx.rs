mod id;
pub(crate) mod lock_manager;
mod simple_storage;

use crate::{Database, Table};
use apllodb_shared_components::{
    data_structure::TableName,
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use apllodb_storage_manager_interface::TxCtxLike;
use id::SimpleTxId;
use lock_manager::TableRwToken;
use simple_storage::{SimpleStorage, TableObj};
use std::{cmp::Ordering, collections::HashMap};

/// Simple (Naive) ACID transaction implementation for Serializable isolation level.
///
/// Used with not only DML but also DDL.
///
/// - **Concurrency control** : SS2PL (Strong Strict Two Phase Lock).
/// - **Lock target** : table.
/// - **Dead lock prevention** : No-Wait.
#[derive(Debug)]
pub(crate) struct SimpleTx<'db> {
    id: SimpleTxId,

    loaded_tables: HashMap<TableName, TableObj>,

    db: &'db Database,
}

impl<'db> PartialEq for SimpleTx<'db> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl<'db> Eq for SimpleTx<'db> {}

impl<'db> Ord for SimpleTx<'db> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}
impl<'db> PartialOrd for SimpleTx<'db> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<'db> TxCtxLike<'db> for SimpleTx<'db> {
    type DbCtx = Database;

    fn begin(db: &'db Self::DbCtx) -> ApllodbResult<Self>
    where
        Self: std::marker::Sized,
    {
        Ok(Self {
            id: SimpleTxId::new(),
            loaded_tables: HashMap::new(),
            db,
        })
    }

    /// # Failures
    ///
    /// - Errors from [Storage::make_durable](foobar.html).
    fn commit(&mut self) -> ApllodbResult<()> {
        SimpleStorage::flush_objects_atomically(
            self.db,
            self.loaded_tables.drain().map(|(_, v)| v).collect(),
        )?;

        self.unlock_all();

        todo!()
    }

    fn abort(&mut self) -> ApllodbResult<()> {
        self.unlock_all();

        todo!()
    }
}

impl<'db> SimpleTx<'db> {
    /// Returns table metadata from buffer the transaction instance owns.
    /// If the metadata does not reside in the transaction's buffer, the transaction try-locks to the table
    /// and issues request to load it to `SimpleStorage`.
    ///
    /// # Failures
    ///
    /// - [TransactionRollback](error/enum.ApllodbErrorKind.html#variant.TransactionRollback) when:
    ///   - Lock for `table_name` is already acquired by another transaction.
    /// - Errors from [SimpleStorage::load_table()](foobar.html)
    pub(crate) fn get_table(&mut self, table_name: &TableName) -> ApllodbResult<Table> {
        let token = self.lock_or_abort(table_name)?;
        Ok(self
            .get_latest_or_load_table_obj(&token)?
            .as_table()
            .clone())
    }

    /// Updates table metadata in buffer the transaction instance owns.
    ///
    /// # Failures
    ///
    /// - [TransactionRollback](error/enum.ApllodbErrorKind.html#variant.TransactionRollback) when:
    ///   - Lock for `table` is already acquired by another transaction.
    /// - Errors from [SimpleStorage::load_table()](foobar.html)
    pub(crate) fn put_table(&mut self, table: Table) -> ApllodbResult<()> {
        let token = self.lock_or_abort(table.name())?;

        let table_obj = self.get_latest_or_load_table_obj(&token)?;
        table_obj.update_by(table);

        Ok(())
    }

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
        self.db
            .lock_manager
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
        self.db.lock_manager.with_lock(|lm| lm.unlock_all(self.id))
    }

    fn get_latest_or_load_table_obj(
        &mut self,
        token: &TableRwToken,
    ) -> ApllodbResult<&mut TableObj> {
        Ok(self
            .loaded_tables
            .entry(token.as_table_name().clone())
            .or_insert(SimpleStorage::load_table(self.db, &token)?))
    }
}

#[cfg(test)]
mod tests {
    use super::SimpleTx;
    use crate::{
        column_constraints, column_definition, column_definitions, database_name,
        table_constraints, table_name, AccessMethods, Database,
    };
    use apllodb_shared_components::error::{ApllodbErrorKind, ApllodbResult};
    use apllodb_storage_manager_interface::{AccessMethodsDdl, TxCtxLike};

    #[test]
    fn test_no_wait() -> ApllodbResult<()> {
        let db = Database::new(database_name!("db_foobar"));

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

    // 自分自身からは途中状態が見える

    // acid
    // iは無理。no-waitなので
    // aはむずいけど試したい
    // cは、トランザクション途中では制約満たしてなくても、commit時に満たすのを試したい
}
