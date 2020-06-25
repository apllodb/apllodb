mod dao;
mod repository;

pub(in crate::sqlite) use dao::VTableDao;

use super::TxId;
use crate::sqlite::{sqlite_error::map_sqlite_err, SqliteDatabase};
use apllodb_immutable_schema_engine_domain::ImmutableSchemaTx;
use apllodb_shared_components::{data_structure::DatabaseName, error::ApllodbResult};
use repository::{VTableRepositoryImpl, VersionRepositoryImpl};
use std::cmp::Ordering;

/// Many transactions share 1 SQLite connection in `Database`.
#[derive(Debug)]
pub struct SqliteTx<'db> {
    id: TxId,
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
    type TID = TxId;

    type VTRepo = VTableRepositoryImpl<'tx, 'db>;
    type VRepo = VersionRepositoryImpl<'tx, 'db>;

    fn id(&self) -> &Self::TID {
        &self.id
    }

    /// # Failures
    ///
    /// - [IoError](error/enum.ApllodbErrorKind.html#variant.IoError) when:
    ///   - rusqlite raises an error.
    fn begin(db: &'db mut Self::Db) -> ApllodbResult<Self> {
        use apllodb_shared_components::traits::Database;

        let database_name = { db.name().clone() };

        let tx = db.sqlite_conn().transaction().map_err(|e| {
            map_sqlite_err(
                e,
                format!("backend sqlite3 raised an error on beginning transaction"),
            )
        })?;

        Ok(Self {
            id: TxId::new(),
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
            map_sqlite_err(
                e,
                "backend sqlite3 raised an error on committing transaction",
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
            map_sqlite_err(e, "backend sqlite3 raised an error on aborting transaction")
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
    use crate::sqlite::SqliteDatabase;
    use apllodb_immutable_schema_engine_domain::{apparent_pk, ApparentPrimaryKey};
    use apllodb_immutable_schema_engine_interface_adapter::TransactionController;
    use apllodb_shared_components::{
        column_constraints, column_definition, column_definitions, column_name, const_expr,
        data_structure::{AlterTableAction, DataType, DataTypeKind},
        data_type,
        error::{ApllodbErrorKind, ApllodbResult},
        hmap, t_pk, table_constraints, table_name,
    };
    use apllodb_storage_engine_interface::Row;
    use apllodb_storage_engine_interface::Transaction;

    #[test]
    fn test_wait_lock() -> ApllodbResult<()> {
        let mut db1 = SqliteDatabase::new_for_test()?;
        let mut db2 = db1.dup()?;

        let tn = &table_name!("t");
        let coldefs = column_definitions!(column_definition!(
            "c1",
            data_type!(DataTypeKind::Integer, false),
            column_constraints!()
        ));
        let tc = table_constraints!(t_pk!("c1"));

        let tx1 = TransactionController::<SqliteTx<'_>>::begin(&mut db1)?;
        let tx2 = TransactionController::<SqliteTx<'_>>::begin(&mut db2)?;

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
    fn test_tx_id_order() -> ApllodbResult<()> {
        let mut db1 = SqliteDatabase::new_for_test()?;
        let mut db2 = db1.dup()?;

        let tx1 = TransactionController::<SqliteTx<'_>>::begin(&mut db1)?;
        let tx2 = TransactionController::<SqliteTx<'_>>::begin(&mut db2)?;

        assert!(tx1.id() < tx2.id());

        Ok(())
    }

    #[test]
    fn test_create_table_failure_duplicate_table() -> ApllodbResult<()> {
        let mut db = SqliteDatabase::new_for_test()?;

        let tn = &table_name!("t");
        let coldefs = column_definitions!(column_definition!(
            "c1",
            data_type!(DataTypeKind::Integer, false),
            column_constraints!()
        ));
        let tc = table_constraints!(t_pk!("c1"));

        let tx = TransactionController::<SqliteTx<'_>>::begin(&mut db)?;

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
        use apllodb_storage_engine_interface::Row;

        let mut db = SqliteDatabase::new_for_test()?;
        let tx = TransactionController::<SqliteTx<'_>>::begin(&mut db)?;

        let tn = &table_name!("t");
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
        let tc = table_constraints!(t_pk!("id"));

        tx.create_table(&tn, &tc, &coldefs)?;

        tx.insert(
            &tn,
            hmap! { column_name!("id") => const_expr!(1), column_name!("c") => const_expr!(1) },
        )?;

        tx.alter_table(
            &tn,
            &AlterTableAction::DropColumn {
                column_name: column_name!("c"),
            },
        )?;

        tx.insert(&tn, hmap! { column_name!("id") => const_expr!(2) })?;

        // Selects both v1's record (id=1) and v2's record (id=2),
        // although v2 does not have column "c".
        let rows = tx.select(&tn, &vec![column_name!("id"), column_name!("c")])?;

        for row_res in rows {
            let row = row_res?;
            let id: i32 = row.get(&column_name!("id"))?;
            match id {
                1 => assert_eq!(row.get::<i32>(&column_name!("c"))?, 1),
                2 => {
                    // Cannot fetch column `c` from v2. Note that v2's `c` is different from NULL,
                    // although it is treated similarly to NULL in GROUP BY, ORDER BY operations.
                    match row.get::<i32>(&column_name!("c")) {
                        Err(e) => assert_eq!(*e.kind(), ApllodbErrorKind::UndefinedColumn),
                        _ => unreachable!(),
                    };
                }
                _ => unreachable!(),
            }
        }

        tx.commit()?;

        Ok(())
    }

    #[test]
    fn test_compound_pk() -> ApllodbResult<()> {
        let mut db = SqliteDatabase::new_for_test()?;
        let tx = TransactionController::<SqliteTx<'_>>::begin(&mut db)?;

        let tn = &table_name!("address");
        let coldefs = column_definitions!(
            column_definition!(
                "country_code",
                data_type!(DataTypeKind::SmallInt, false),
                column_constraints!()
            ),
            column_definition!(
                "postal_code",
                data_type!(DataTypeKind::Integer, false),
                column_constraints!()
            ),
        );
        let tc = table_constraints!(t_pk!("country_code", "postal_code"));

        tx.create_table(&tn, &tc, &coldefs)?;

        tx.insert(
            &tn,
            hmap! { column_name!("country_code") => const_expr!(100i16), column_name!("postal_code") => const_expr!(1000001i32) },
        )?;

        let row_iter = tx.select(&tn, &vec![column_name!("postal_code")])?;
        for row_res in row_iter {
            let row = row_res?;
            assert_eq!(row.pk(), &apparent_pk![
                (
                    column_name!("country_code"),
                    SqlValue::pack(&DataType::new(DataTypeKind::SmallInt, false) , &100i16)?,
                ),
                (
                    column_name!("postal_code"),
                    SqlValue::pack(&DataType::new(DataTypeKind::Integer, false) , &1000001i32)?,
                ),
            ]
            , "although `country_code` is not specified in SELECT projection, it's available since it's a part of PK");
        }

        tx.commit()?;

        Ok(())
    }
}
