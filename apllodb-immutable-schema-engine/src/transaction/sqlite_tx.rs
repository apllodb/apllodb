mod database;
mod id;

use crate::Table;
use apllodb_shared_components::{
    data_structure::TableName,
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use apllodb_storage_manager_interface::TxCtxLike;
use database::Database;
use id::SqliteTxId;
use std::cmp::Ordering;

/// Many transactions share 1 SQLite connection in `Database`.
pub(crate) struct SqliteTx<'db> {
    id: SqliteTxId,
    sqlite_tx: rusqlite::Transaction<'db>,
}

impl<'db> PartialEq for SqliteTx<'db> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl<'db> Eq for SqliteTx<'db> {}

impl<'db> PartialOrd for SqliteTx<'db> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<'db> Ord for SqliteTx<'db> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl<'db> TxCtxLike for SqliteTx<'db> {
    /// # Failures
    ///
    /// If any of the following error is returned, transaction has already been aborted.
    ///
    /// - [IoError](error/enum.ApllodbErrorKind.html#variant.IoError) when:
    ///   - rusqlite raises an error.
    fn commit(self) -> ApllodbResult<()> {
        self.sqlite_tx.commit().map_err(|e| {
            ApllodbError::new(
                ApllodbErrorKind::IoError,
                format!("backend sqlite3 raised an error on committing transaction"),
                Some(Box::new(e)),
            )
        })?;
        Ok(())
    }

    /// # Failures
    ///
    /// - [IoError](error/enum.ApllodbErrorKind.html#variant.IoError) when:
    ///   - rusqlite raises an error.
    fn abort(self) -> ApllodbResult<()> {
        self.sqlite_tx.commit().map_err(|e| {
            ApllodbError::new(
                ApllodbErrorKind::IoError,
                format!("backend sqlite3 raised an error on committing transaction"),
                Some(Box::new(e)),
            )
        })?;
        Ok(())
    }
}

impl<'db> SqliteTx<'db> {
    #[allow(dead_code)]
    /// # Failures
    ///
    /// - [IoError](error/enum.ApllodbErrorKind.html#variant.IoError) when:
    ///   - rusqlite raises an error.
    fn begin(db: &'db mut Database) -> ApllodbResult<Self>
    where
        Self: std::marker::Sized,
    {
        let tx = db.sqlite_conn().transaction().map_err(|e| {
            ApllodbError::new(
                ApllodbErrorKind::IoError,
                format!("backend sqlite3 raised an error on beginning transaction"),
                Some(Box::new(e)),
            )
        })?;

        Ok(Self {
            id: SqliteTxId::new(),
            sqlite_tx: tx,
        })
    }

    /// Returns table metadata from buffer the transaction instance owns.
    ///
    /// # Failures
    ///
    /// - [IoError](error/enum.ApllodbErrorKind.html#variant.IoError) when:
    ///   - rusqlite raises an error.
    /// - [UndefinedTable](error/enum.ApllodbErrorKind.html#variant.UndefinedTable) when:
    ///   - Table `table_name` is not visible to this transaction.
    pub(crate) fn get_table(&mut self, _table_name: &TableName) -> ApllodbResult<Table> {
        todo!()
    }

    /// Updates table metadata in buffer the transaction instance owns.
    ///
    /// # Failures
    ///
    /// - [IoError](error/enum.ApllodbErrorKind.html#variant.IoError) when:
    ///   - rusqlite raises an error.
    pub(crate) fn put_table(&mut self, _table: Table) -> ApllodbResult<()> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::{database::Database, SqliteTx};
    use crate::{
        column_constraints, column_definition, column_definitions, database_name,
        table_constraints, table_name, AccessMethods,
    };
    use apllodb_shared_components::error::{ApllodbErrorKind, ApllodbResult};
    use apllodb_storage_manager_interface::AccessMethodsDdl;
    use apllodb_storage_manager_interface::TxCtxLike;

    #[test]
    fn test_wait() -> ApllodbResult<()> {
        let mut db1 = Database::new(database_name!("db_foobar"))?;
        let mut db2 = Database::new(database_name!("db_foobar"))?;

        let tn = &table_name!("t");
        let tc = table_constraints!();
        let coldefs = column_definitions!(column_definition!("c1", column_constraints!()));

        let mut tx1 = SqliteTx::begin(&mut db1)?;
        let mut tx2 = SqliteTx::begin(&mut db2)?;

        // tx1 is created earlier than tx2 but tx2 issues CREATE TABLE command in prior to tx1.
        // In this case, tx1 is blocked by tx2, and tx1 gets an error indicating table duplication.
        AccessMethods::create_table(&mut tx2, &tn, &tc, &coldefs)?;
        AccessMethods::create_table(&mut tx1, &tn, &tc, &coldefs)?;

        match tx1.commit() {
            // blocked and got error
            Err(e) => {
                println!("{}", e);
                assert_eq!(*e.kind(), ApllodbErrorKind::IoError)
            }
            Ok(_) => panic!("should rollback"),
        }
        tx2.commit()?;

        Ok(())
    }

    // reentrant

    // 自分自身からは途中状態が見える

    // acid
    // iは無理。no-waitなので
    // aはむずいけど試したい
    // cは、トランザクション途中では制約満たしてなくても、commit時に満たすのを試したい
}
