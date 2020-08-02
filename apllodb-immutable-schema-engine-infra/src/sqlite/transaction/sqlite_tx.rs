mod dao;
mod repository;
mod sqlite_statement;

pub(in crate::sqlite) use dao::VTableDao;
pub(in crate::sqlite::transaction::sqlite_tx) use sqlite_statement::SqliteStatement;

use super::tx_id::TxId;
use crate::sqlite::{
    database::SqliteDatabase, sqlite_error::map_sqlite_err, sqlite_rowid::SqliteRowid,
    to_sql_string::ToSqlString,
};
use apllodb_immutable_schema_engine_domain::transaction::ImmutableSchemaTx;
use apllodb_shared_components::{
    data_structure::DatabaseName,
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use log::debug;
use repository::{VTableRepositoryImpl, VersionRepositoryImpl};
use std::cmp::Ordering;

/// Many transactions share 1 SQLite connection in `Database`.
#[derive(Debug)]
pub struct SqliteTx<'db> {
    id: TxId,
    database_name: DatabaseName,
    rusqlite_tx: rusqlite::Transaction<'db>,
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
            rusqlite_tx: tx,
        })
    }

    /// # Failures
    ///
    /// If any of the following error is returned, transaction has already been aborted.
    ///
    /// - [IoError](error/enum.ApllodbErrorKind.html#variant.IoError) when:
    ///   - rusqlite raises an error.
    fn commit(self) -> ApllodbResult<()> {
        self.rusqlite_tx.commit().map_err(|e| {
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
        self.rusqlite_tx.rollback().map_err(|e| {
            map_sqlite_err(e, "backend sqlite3 raised an error on aborting transaction")
        })?;
        Ok(())
    }

    fn database_name(&self) -> &DatabaseName {
        &self.database_name
    }

    fn vtable_repo(&'tx self) -> Self::VTRepo {
        use apllodb_immutable_schema_engine_domain::traits::VTableRepository;
        VTableRepositoryImpl::new(&self)
    }

    fn version_repo(&'tx self) -> Self::VRepo {
        use apllodb_immutable_schema_engine_domain::traits::VersionRepository;
        VersionRepositoryImpl::new(&self)
    }
}

impl<'db> SqliteTx<'db> {
    pub(in crate::sqlite::transaction::sqlite_tx) fn prepare<S: Into<String>>(
        &self,
        sql: S,
    ) -> ApllodbResult<SqliteStatement<'_, '_>> {
        let sql = sql.into();
        debug!("SqliteTx::prepare(): {}", sql);

        let raw_stmt = self
            .rusqlite_tx
            .prepare(&sql)
            .map_err(|e| map_sqlite_err(e, "SQLite raised an error on prepare"))?;
        Ok(SqliteStatement::new(&self, raw_stmt))
    }

    pub(in crate::sqlite::transaction::sqlite_tx) fn execute_named<S: Into<String>>(
        &self,
        sql: S,
        params: &[(&str, &dyn ToSqlString)],
    ) -> ApllodbResult<()> {
        // TODO return ChangedRows(usize)

        let sql = sql.into();
        debug!("SqliteTx::execute_named(): {}", sql);

        let params = params
            .into_iter()
            .map(|(pname, v)| (*pname, v.to_sql_string()))
            .collect::<Vec<(&str, String)>>();

        let msg = |prefix: &str| {
            format!(
                "{} while execute_named() with the following command: {}",
                prefix, sql
            )
        };

        self.rusqlite_tx
            .execute_named(
                &sql,
                params
                    .iter()
                    .map(|(pname, s)| (*pname, s as &dyn rusqlite::ToSql))
                    .collect::<Vec<_>>()
                    .as_slice(),
            )
            .map_err(|e| match e {
                rusqlite::Error::SqliteFailure(
                    libsqlite3_sys::Error {
                        code: libsqlite3_sys::ErrorCode::DatabaseBusy,
                        ..
                    },
                    _,
                ) => ApllodbError::new(
                    ApllodbErrorKind::DeadlockDetected,
                    msg("deadlock detected"),
                    Some(Box::new(e)),
                ),

                rusqlite::Error::SqliteFailure(
                    libsqlite3_sys::Error {
                        extended_code: rusqlite::ffi::SQLITE_CONSTRAINT_PRIMARYKEY,
                        ..
                    },
                    _,
                ) => ApllodbError::new(
                    ApllodbErrorKind::UniqueViolation,
                    msg("duplicate value on primary key"),
                    Some(Box::new(e)),
                ),

                _ => map_sqlite_err(e, msg("unexpected error")),
            })?;

        Ok(())
    }

    pub(in crate::sqlite::transaction::sqlite_tx) fn last_insert_rowid(&self) -> SqliteRowid {
        SqliteRowid(self.rusqlite_tx.last_insert_rowid())
    }
}

#[cfg(test)]
mod tests {
    use super::SqliteTx;
    use crate::{sqlite::database::SqliteDatabase, test_support::setup};

    use apllodb_immutable_schema_engine_domain::{apparent_pk, pk_column_name};
    use apllodb_immutable_schema_engine_interface_adapter::TransactionController;
    use apllodb_shared_components::{
        column_constraints, column_definition, column_definitions, column_name, const_expr,
        data_structure::{AlterTableAction, DataType, DataTypeKind, SqlValue},
        data_type,
        error::{ApllodbErrorKind, ApllodbResult},
        hmap, t_pk, table_constraints, table_name,
    };
    use apllodb_storage_engine_interface::Transaction;

    #[test]
    fn test_create_table_failure_duplicate_table() -> ApllodbResult<()> {
        setup();

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
        setup();

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
        setup();

        use apllodb_storage_engine_interface::Row;

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
                    pk_column_name!("country_code"),
                    SqlValue::pack(&DataType::new(DataTypeKind::SmallInt, false) , &100i16)?,
                ),
                (
                    pk_column_name!("postal_code"),
                    SqlValue::pack(&DataType::new(DataTypeKind::Integer, false) , &1000001i32)?,
                ),
            ]
            , "although `country_code` is not specified in SELECT projection, it's available since it's a part of PK");
        }

        tx.commit()?;

        Ok(())
    }
}
