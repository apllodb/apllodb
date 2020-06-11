mod dao;
mod database;
mod id;
mod record_iterator;
mod sqlite_table_name;
mod to_sql_string;

pub(crate) use database::Database;
pub(crate) use record_iterator::SqliteRecordIterator;

pub(self) use to_sql_string::ToSqlString;

use super::ImmutableSchemaTx;
use apllodb_shared_components::{
    data_structure::TableName,
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use apllodb_storage_manager_interface::TxCtxLike;
use dao::{TableDao, VersionDao};
use id::SqliteTxId;
use std::cmp::Ordering;

/// Many transactions share 1 SQLite connection in `Database`.
#[derive(Debug)]
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
        self.sqlite_tx.rollback().map_err(|e| {
            ApllodbError::new(
                ApllodbErrorKind::IoError,
                format!("backend sqlite3 raised an error on committing transaction"),
                Some(Box::new(e)),
            )
        })?;
        Ok(())
    }
}

impl<'db> ImmutableSchemaTx for SqliteTx<'db> {
    type Tbl = crate::Table<'db, SqliteTx<'db>>;

    /// # Failures
    ///
    /// - [IoError](error/enum.ApllodbErrorKind.html#variant.IoError) when:
    ///   - rusqlite raises an error.
    /// - [UndefinedTable](error/enum.ApllodbErrorKind.html#variant.UndefinedTable) when:
    ///   - Table `table_name` is not visible to this transaction.
    fn get_table(&self, _table_name: &TableName) -> ApllodbResult<Self::Tbl> {
        todo!()
    }

    /// **This operation does not satisfies atomicity and isolation** because
    /// SQLite's DDL commands are internally issued.
    ///
    /// # Failures
    ///
    /// - Errors from [TableDao::create()](foobar.html).
    fn create_table(&self, table: &Self::Tbl) -> ApllodbResult<()> {
        let v1 = table.version_repo().current_version()?;

        self.table_dao().create(&table)?;
        self.version_dao(table.name().clone()).create(&v1)?;

        Ok(())
    }

    /// **This operation does not satisfies atomicity and isolation** because
    /// SQLite's DDL commands are internally issued.
    ///
    /// # Failures
    ///
    /// - [IoError](error/enum.ApllodbErrorKind.html#variant.IoError) when:
    ///   - rusqlite raises an error.
    fn alter_table(&self, _table: &Self::Tbl) -> ApllodbResult<()> {
        // insert table metadata
        // create v1
        todo!()
    }
}

impl<'tx, 'db: 'tx> SqliteTx<'db> {
    #[allow(dead_code)]
    /// # Failures
    ///
    /// - [IoError](error/enum.ApllodbErrorKind.html#variant.IoError) when:
    ///   - rusqlite raises an error.
    pub fn begin(db: &'db mut Database) -> ApllodbResult<Self>
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

    fn table_dao(&self) -> TableDao<'_> {
        TableDao::new(&self.sqlite_tx)
    }

    fn version_dao(&self, table_name: TableName) -> VersionDao<'_> {
        VersionDao::new(&self.sqlite_tx, table_name)
    }
}

#[cfg(test)]
mod tests {
    use super::{database::Database, SqliteTx};
    use crate::{
        column_constraints, column_definition, column_definitions, data_type, table_constraints,
        table_name, AccessMethods,
    };
    use apllodb_shared_components::{
        data_structure::DataTypeKind,
        error::{ApllodbErrorKind, ApllodbResult},
    };
    use apllodb_storage_manager_interface::AccessMethodsDdl;
    use apllodb_storage_manager_interface::TxCtxLike;

    #[test]
    fn test_wait_lock() -> ApllodbResult<()> {
        let mut db1 = Database::new_for_test()?;
        let mut db2 = db1.dup()?;

        let tn = &table_name!("t");
        let tc = table_constraints!();
        let coldefs = column_definitions!(column_definition!(
            "c1",
            data_type!(DataTypeKind::Integer, false),
            column_constraints!()
        ));

        let mut tx1 = SqliteTx::begin(&mut db1)?;
        let mut tx2 = SqliteTx::begin(&mut db2)?;

        // tx1 is created earlier than tx2 but tx2 issues CREATE TABLE command in prior to tx1.
        // In this case, tx1 is blocked by tx2, and tx1 gets an error indicating table duplication.
        AccessMethods::create_table(&mut tx2, &tn, &tc, &coldefs)?;
        match AccessMethods::create_table(&mut tx1, &tn, &tc, &coldefs) {
            // Internally, new record is trying to be INSERTed but it is made wait by tx2.
            // (Since SQLite's transaction is neither OCC nor MVCC, tx1 is made wait here before transaction commit.)
            Err(e) => assert_eq!(*e.kind(), ApllodbErrorKind::DeadlockDetected),
            Ok(_) => panic!("should rollback"),
        }

        tx1.commit()?; // it's ok to commit tx1 although it already aborted by error.
        tx2.commit()?;

        Ok(())
    }

    // reentrant

    // 自分自身からは途中状態が見える

    // acid
    // iは無理。no-waitなので
    // aはむずいけど試したい
    // cは、トランザクション途中では制約満たしてなくても、commit時に満たすのを試したい

    #[test]
    fn test_create_table_failure_duplicate_table() -> ApllodbResult<()> {
        let mut db = Database::new_for_test()?;

        let tn = &table_name!("t");
        let tc = table_constraints!();
        let coldefs = column_definitions!(column_definition!(
            "c1",
            data_type!(DataTypeKind::Integer, false),
            column_constraints!()
        ));

        let mut tx = SqliteTx::begin(&mut db)?;

        AccessMethods::create_table(&mut tx, &tn, &tc, &coldefs)?;
        match AccessMethods::create_table(&mut tx, &tn, &tc, &coldefs) {
            // Internally, new record is trying to be INSERTed but it is made wait by tx2.
            // (Since SQLite's transaction is neither OCC nor MVCC, tx1 is made wait here before transaction commit.)
            Err(e) => assert_eq!(*e.kind(), ApllodbErrorKind::DuplicateTable),
            Ok(_) => panic!("should rollback"),
        }
        Ok(())
    }
}
