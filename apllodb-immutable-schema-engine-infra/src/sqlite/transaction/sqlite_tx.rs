mod dao;
mod id;

use crate::sqlite::{SqliteDatabase, SqliteRowIterator};
use apllodb_immutable_schema_engine_domain::{
    ActiveVersion, ImmutableSchemaTx, VTable, VersionNumber,
};
use apllodb_immutable_schema_engine_interface_adapter::TransactionController;
use apllodb_shared_components::{
    data_structure::{ColumnName, DatabaseName, TableName},
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use apllodb_storage_engine_interface::Transaction;
use dao::{VTableDao, VersionDao};
use id::SqliteTxId;
use std::cmp::Ordering;

/// Many transactions share 1 SQLite connection in `Database`.
#[derive(Debug)]
pub struct SqliteTx<'conn> {
    id: SqliteTxId,
    database_name: DatabaseName,
    pub(in crate::sqlite) sqlite_tx: rusqlite::Transaction<'conn>, // TODO private
}

impl PartialEq for SqliteTx<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for SqliteTx<'_> {}

impl PartialOrd for SqliteTx<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for SqliteTx<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl<'conn> ImmutableSchemaTx<'conn> for SqliteTx<'conn> {
    type Db = SqliteDatabase;
    type VerRowIter = SqliteRowIterator<'conn>;

    /// # Failures
    ///
    /// - [IoError](error/enum.ApllodbErrorKind.html#variant.IoError) when:
    ///   - rusqlite raises an error.
    fn begin(db: &'conn mut Self::Db) -> ApllodbResult<Self> {
        use apllodb_shared_components::traits::Database;

        let database_name = { db.name().clone() };

        let tx = db.sqlite_conn().transaction().map_err(|e| {
            ApllodbError::new(
                ApllodbErrorKind::IoError,
                format!("backend sqlite3 raised an error on beginning transaction"),
                Some(Box::new(e)),
            )
        })?;

        Ok(Self {
            id: SqliteTxId::new(),
            database_name: database_name,
            sqlite_tx: tx,
        })
    }

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

    fn database_name(&self) -> &DatabaseName {
        &self.database_name
    }

    /// **This operation does not satisfies atomicity and isolation** because
    /// SQLite's DDL commands are internally issued.
    ///
    /// # Failures
    ///
    /// - [DuplicateTable](error/enum.ApllodbErrorKind.html#variant.DuplicateTable) when:
    ///   - Table `table_name` is already visible to this transaction.
    /// - Errors from [TableDao::create()](foobar.html).
    fn create_vtable(&self, table: &VTable) -> ApllodbResult<()> {
        self.vtable_dao().create(&table)?;
        Ok(())
    }

    /// # Failures
    ///
    /// - [IoError](error/enum.ApllodbErrorKind.html#variant.IoError) when:
    ///   - rusqlite raises an error.
    /// - [UndefinedTable](error/enum.ApllodbErrorKind.html#variant.UndefinedTable) when:
    ///   - Table `table_name` is not visible to this transaction.
    fn read_vtable(&self, table_name: &TableName) -> ApllodbResult<VTable> {
        todo!()
    }

    /// **This operation does not satisfies atomicity and isolation** because
    /// SQLite's DDL commands are internally issued.
    ///
    /// # Failures
    ///
    /// - [UndefinedTable](error/enum.ApllodbErrorKind.html#variant.UndefinedTable) when:
    ///   - Table `table_name` is not visible to this transaction.
    /// - [IoError](error/enum.ApllodbErrorKind.html#variant.IoError) when:
    ///   - rusqlite raises an error.
    fn update_vtable(&self, table: &VTable) -> ApllodbResult<()> {
        todo!()
    }

    fn create_version(&self, version: &ActiveVersion) -> ApllodbResult<()> {
        // self.version_dao(table.name().clone()).create(version)?;
        // Ok(())

        todo!()
    }

    fn deactivate_version(
        &self,
        table: &VTable,
        version_number: &VersionNumber,
    ) -> ApllodbResult<()> {
        todo!()
    }

    fn full_scan(
        &self,
        _version: &ActiveVersion,
        _column_names: &[ColumnName],
    ) -> ApllodbResult<Self::VerRowIter> {
        todo!()
    }
}

impl<'conn> SqliteTx<'conn> {
    fn vtable_dao(&self) -> VTableDao<'_> {
        VTableDao::new(&self.sqlite_tx)
    }

    fn version_dao(&self) -> VersionDao<'_> {
        VersionDao::new(&self.sqlite_tx)
    }
}

#[cfg(test)]
mod tests {
    use super::SqliteTx;
    use crate::sqlite::{SqliteDatabase, SqliteRowIterator};
    use apllodb_immutable_schema_engine_interface_adapter::TransactionController;
    use apllodb_shared_components::{
        column_constraints, column_definition, column_definitions,
        data_structure::DataTypeKind,
        data_type,
        error::{ApllodbErrorKind, ApllodbResult},
        table_constraints, table_name,
    };
    use apllodb_storage_engine_interface::Transaction;

    #[test]
    fn test_wait_lock() -> ApllodbResult<()> {
        let mut db1 = SqliteDatabase::new_for_test()?;
        let mut db2 = db1.dup()?;

        let tn = &table_name!("t");
        let tc = table_constraints!();
        let coldefs = column_definitions!(column_definition!(
            "c1",
            data_type!(DataTypeKind::Integer, false),
            column_constraints!()
        ));

        let mut tx1 =
            TransactionController::<SqliteTx<'_>, SqliteRowIterator<'_>>::begin(&mut db1)?;
        let mut tx2 =
            TransactionController::<SqliteTx<'_>, SqliteRowIterator<'_>>::begin(&mut db2)?;

        // tx1 is created earlier than tx2 but tx2 issues CREATE TABLE command in prior to tx1.
        // In this case, tx1 is blocked by tx2, and tx1 gets an error indicating table duplication.
        tx2.create_table(&tn, &tc, &coldefs)?;
        match tx1.create_table(&tn, &tc, &coldefs) {
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
        let mut db = SqliteDatabase::new_for_test()?;

        let tn = &table_name!("t");
        let tc = table_constraints!();
        let coldefs = column_definitions!(column_definition!(
            "c1",
            data_type!(DataTypeKind::Integer, false),
            column_constraints!()
        ));

        let mut tx = TransactionController::<SqliteTx<'_>, SqliteRowIterator<'_>>::begin(&mut db)?;

        tx.create_table(&tn, &tc, &coldefs)?;
        match tx.create_table(&tn, &tc, &coldefs) {
            // Internally, new record is trying to be INSERTed but it is made wait by tx2.
            // (Since SQLite's transaction is neither OCC nor MVCC, tx1 is made wait here before transaction commit.)
            Err(e) => assert_eq!(*e.kind(), ApllodbErrorKind::DuplicateTable),
            Ok(_) => panic!("should rollback"),
        }
        Ok(())
    }
}
