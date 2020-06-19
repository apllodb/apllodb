mod dao;
mod id;
mod repository;

pub(in crate::sqlite) use dao::VTableDao;

use crate::sqlite::SqliteDatabase;
use apllodb_immutable_schema_engine_domain::ImmutableSchemaTx;
use apllodb_shared_components::{
    data_structure::DatabaseName,
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use id::SqliteTxId;
use repository::{VTableRepositoryImpl, VersionRepositoryImpl};
use std::cmp::Ordering;

/// Many transactions share 1 SQLite connection in `Database`.
#[derive(Debug)]
pub struct SqliteTx<'db> {
    id: SqliteTxId,
    database_name: DatabaseName,
    pub(in crate::sqlite) sqlite_tx: rusqlite::Transaction<'db>, // TODO private
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

impl<'tx, 'db: 'tx> ImmutableSchemaTx<'tx, 'db> for SqliteTx<'db> {
    type Db = SqliteDatabase;

    type VTRepo = VTableRepositoryImpl<'tx, 'db>;
    type VRepo = VersionRepositoryImpl<'tx, 'db>;

    /// # Failures
    ///
    /// - [IoError](error/enum.ApllodbErrorKind.html#variant.IoError) when:
    ///   - rusqlite raises an error.
    fn begin(db: &'db mut Self::Db) -> ApllodbResult<Self> {
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

    fn vtable_repo(&'tx self) -> Self::VTRepo {
        use apllodb_immutable_schema_engine_domain::VTableRepository;
        VTableRepositoryImpl::new(&self)
    }

    fn version_repo(&'tx self) -> Self::VRepo {
        use apllodb_immutable_schema_engine_domain::VersionRepository;
        VersionRepositoryImpl::new(&self)
    }
}

#[cfg(test)]
mod tests {
    use super::SqliteTx;
    use crate::sqlite::{SqliteDatabase, SqliteRowIterator};
    use apllodb_immutable_schema_engine_interface_adapter::TransactionController;
    use apllodb_shared_components::{
        column_constraints, column_definition, column_definitions, column_name, const_expr,
        data_structure::{AlterTableAction, DataTypeKind},
        data_type,
        error::{ApllodbErrorKind, ApllodbResult},
        hmap, table_constraints, table_name,
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

        let tx1 = TransactionController::<SqliteTx<'_>, SqliteRowIterator<'_>>::begin(&mut db1)?;
        let tx2 = TransactionController::<SqliteTx<'_>, SqliteRowIterator<'_>>::begin(&mut db2)?;

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

        let tx = TransactionController::<SqliteTx<'_>, SqliteRowIterator<'_>>::begin(&mut db)?;

        tx.create_table(&tn, &tc, &coldefs)?;
        match tx.create_table(&tn, &tc, &coldefs) {
            // Internally, new record is trying to be INSERTed but it is made wait by tx2.
            // (Since SQLite's transaction is neither OCC nor MVCC, tx1 is made wait here before transaction commit.)
            Err(e) => assert_eq!(*e.kind(), ApllodbErrorKind::DuplicateTable),
            Ok(_) => panic!("should rollback"),
        }
        Ok(())
    }

    #[test]
    fn test_success_select_all_from_2_versions() -> ApllodbResult<()> {
        let mut db = SqliteDatabase::new_for_test()?;
        let tx = TransactionController::<SqliteTx<'_>, SqliteRowIterator<'_>>::begin(&mut db)?;

        let tn = &table_name!("t");
        let tc = table_constraints!();
        let coldefs = column_definitions!(
            column_definition!(
                "id",
                data_type!(DataTypeKind::Integer, false),
                column_constraints!()
            ),
            column_definition!(
                "c",
                data_type!(DataTypeKind::Integer, false),
                column_constraints!()
            ),
        );

        tx.create_table(&tn, &tc, &coldefs)?;

        // tx.insert(
        //     &tn,
        //     hmap! { column_name!("id") => const_expr!(1), column_name!("c") => const_expr!(1) },
        // )?;

        tx.alter_table(
            &tn,
            &AlterTableAction::DropColumn {
                column_name: column_name!("c"),
            },
        )?;

        // tx.insert(&tn, hmap! { column_name!("id") => const_expr!(2) })?;

        // Selects both v1's record (id=1) and v2's record (id=2),
        // although v2 does not have column "c".
        // let records = tx.select(&tn, &vec![column_name!("id"), column_name!("c")])?;

        // for rec_res in records {
        //     let r = rec_res?;
        //     let id: i64 = r.get(&column_name!("id"))?;
        //     match id {
        //         1 => assert_eq!(r.get::<i64>(&column_name!("c"))?, 1),
        //         2 => {
        //             match r.get::<i64>(&column_name!("c")) {
        //                 Err(e) => assert_eq!(*e.kind(), ApllodbErrorKind::DatatypeMismatch),
        //                 _ => unreachable!(),
        //             };
        //             assert_eq!(r.get::<Option<i64>>(&column_name!("c"))?, None);
        //         }
        //         _ => unreachable!(),
        //     }
        // }

        Ok(())
    }
}
