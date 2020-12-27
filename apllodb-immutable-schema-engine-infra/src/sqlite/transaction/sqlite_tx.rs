mod sqlite_statement;
pub(crate) mod version;
pub(crate) mod version_revision_resolver;
pub(crate) mod vtable;

use apllodb_immutable_schema_engine_application::use_case::transaction::{
    alter_table::{AlterTableUseCase, AlterTableUseCaseInput},
    create_table::{CreateTableUseCase, CreateTableUseCaseInput},
    delete_all::{DeleteAllUseCase, DeleteAllUseCaseInput},
    full_scan::FullScanUseCase,
    full_scan::FullScanUseCaseInput,
    insert::{InsertUseCase, InsertUseCaseInput},
    update_all::UpdateAllUseCase,
    update_all::UpdateAllUseCaseInput,
};
use apllodb_immutable_schema_engine_application::use_case::TxUseCase;
use apllodb_storage_engine_interface::{ProjectionQuery, Transaction, TransactionBuilder};
pub(in crate::sqlite::transaction::sqlite_tx) use sqlite_statement::SqliteStatement;

use self::{
    version::repository_impl::VersionRepositoryImpl, vtable::repository_impl::VTableRepositoryImpl,
};

use super::tx_id::TxId;
use crate::{
    external_interface::ApllodbImmutableSchemaEngine,
    immutable_schema_row_iter::ImmutableSchemaRowIter,
    sqlite::{
        database::SqliteDatabase, sqlite_error::map_sqlite_err, sqlite_rowid::SqliteRowid,
        sqlite_types::SqliteTypes, to_sql_string::ToSqlString,
    },
};
use apllodb_shared_components::{
    AlterTableAction, ApllodbError, ApllodbErrorKind, ApllodbResult, ColumnDefinition, ColumnName,
    DatabaseName, Expression, TableConstraints, TableName,
};
use log::debug;
use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, new)]
pub struct SqliteTxBuilder<'db> {
    db: &'db mut SqliteDatabase,
}
impl<'db> TransactionBuilder for SqliteTxBuilder<'db> {}

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

impl<'db> SqliteTx<'db> {
    /// Construct SqliteTx, beginning new transaction at the same time.
    ///
    /// # Failures
    ///
    /// - [IoError](apllodb_shared_components::ApllodbErrorKind::IoError) when:
    ///   - rusqlite raises an error.
    pub fn new(db: &'db mut SqliteDatabase) -> ApllodbResult<Self> {
        use apllodb_shared_components::Database;

        let database_name = { db.name().clone() };

        let tx = db.sqlite_conn().transaction().map_err(|e| {
            map_sqlite_err(
                e,
                "backend sqlite3 raised an error on beginning transaction",
            )
        })?;

        Ok(Self {
            id: TxId::new(),
            database_name,
            rusqlite_tx: tx,
        })
    }

    fn vtable_repo(&self) -> VTableRepositoryImpl<'_, 'db> {
        VTableRepositoryImpl::new(self)
    }

    fn version_repo(&self) -> VersionRepositoryImpl<'_, 'db> {
        VersionRepositoryImpl::new(self)
    }
}

impl<'tx, 'db: 'tx> Transaction<ApllodbImmutableSchemaEngine<'db>> for SqliteTx<'db> {
    fn id(&self) -> &TxId {
        &self.id
    }

    /// # Failures
    ///
    /// - [IoError](apllodb_shared_components::ApllodbErrorKind::IoError) when:
    ///   - rusqlite raises an error.
    fn begin(builder: SqliteTxBuilder<'db>) -> ApllodbResult<Self> {
        use apllodb_shared_components::Database;

        let database_name = { builder.db.name().clone() };

        let tx = builder.db.sqlite_conn().transaction().map_err(|e| {
            map_sqlite_err(
                e,
                "backend sqlite3 raised an error on beginning transaction",
            )
        })?;

        Ok(Self {
            id: TxId::new(),
            database_name,
            rusqlite_tx: tx,
        })
    }

    /// # Failures
    ///
    /// If any of the following error is returned, transaction has already been aborted.
    ///
    /// - [IoError](apllodb_shared_components::ApllodbErrorKind::IoError) when:
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
    /// - [IoError](apllodb_shared_components::ApllodbErrorKind::IoError) when:
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

    fn create_table(
        &self,
        table_name: &TableName,
        table_constraints: &TableConstraints,
        column_definitions: &[ColumnDefinition],
    ) -> ApllodbResult<()> {
        let database_name = self.database_name().clone();
        let input = CreateTableUseCaseInput::new(
            &database_name,
            table_name,
            table_constraints,
            column_definitions,
        );
        let _ = CreateTableUseCase::<'_, '_, ApllodbImmutableSchemaEngine, SqliteTypes>::run(
            &self.vtable_repo(),
            &self.version_repo(),
            input,
        )?;
        Ok(())
    }

    fn alter_table(&self, table_name: &TableName, action: &AlterTableAction) -> ApllodbResult<()> {
        let database_name = self.database_name().clone();
        let input = AlterTableUseCaseInput::new(&database_name, table_name, action);
        let _ = AlterTableUseCase::<'_, '_, ApllodbImmutableSchemaEngine, SqliteTypes>::run(
            &self.vtable_repo(),
            &self.version_repo(),
            input,
        )?;
        Ok(())
    }

    fn drop_table(&self, _table_name: &TableName) -> ApllodbResult<()> {
        todo!()
    }

    fn select(
        &self,
        table_name: &TableName,
        projection: ProjectionQuery,
    ) -> ApllodbResult<ImmutableSchemaRowIter> {
        let database_name = self.database_name().clone();
        let input = FullScanUseCaseInput::new(&database_name, table_name, projection);
        let output = FullScanUseCase::<'_, '_, ApllodbImmutableSchemaEngine, SqliteTypes>::run(
            &self.vtable_repo(),
            &self.version_repo(),
            input,
        )?;

        Ok(output.row_iter)
    }

    fn insert(
        &self,
        table_name: &TableName,
        column_values: HashMap<ColumnName, Expression>,
    ) -> ApllodbResult<()> {
        let database_name = self.database_name().clone();
        let input = InsertUseCaseInput::new(&database_name, table_name, column_values);
        let _ = InsertUseCase::<'_, '_, ApllodbImmutableSchemaEngine, SqliteTypes>::run(
            &self.vtable_repo(),
            &self.version_repo(),
            input,
        )?;
        Ok(())
    }

    fn update(
        &self,
        table_name: &TableName,
        column_values: HashMap<ColumnName, Expression>,
    ) -> ApllodbResult<()> {
        let database_name = self.database_name().clone();
        let input = UpdateAllUseCaseInput::new(&database_name, table_name, column_values);
        let _ = UpdateAllUseCase::<'_, '_, ApllodbImmutableSchemaEngine, SqliteTypes>::run(
            &self.vtable_repo(),
            &self.version_repo(),
            input,
        )?;

        Ok(())
    }

    fn delete(&self, table_name: &TableName) -> ApllodbResult<()> {
        let database_name = self.database_name().clone();
        let input = DeleteAllUseCaseInput::new(&database_name, table_name);
        let _ = DeleteAllUseCase::<'_, '_, ApllodbImmutableSchemaEngine, SqliteTypes>::run(
            &self.vtable_repo(),
            &self.version_repo(),
            input,
        )?;

        Ok(())
    }
}

impl<'db> SqliteTx<'db> {
    pub(in crate::sqlite::transaction::sqlite_tx) fn prepare<S: AsRef<str>>(
        &self,
        sql: S,
    ) -> ApllodbResult<SqliteStatement<'_, '_>> {
        let sql = sql.as_ref();
        debug!("SqliteTx::prepare():\n    {}", sql);

        let raw_stmt = self
            .rusqlite_tx
            .prepare(sql)
            .map_err(|e| map_sqlite_err(e, "SQLite raised an error on prepare"))?;
        Ok(SqliteStatement::new(&self, raw_stmt))
    }

    pub(in crate::sqlite::transaction::sqlite_tx) fn execute_named<S: AsRef<str>>(
        &self,
        sql: S,
        params: &[(&str, &dyn ToSqlString)],
    ) -> ApllodbResult<()> {
        // TODO return ChangedRows(usize)

        let sql = sql.as_ref();
        debug!("SqliteTx::execute_named():\n    {}", sql);

        let params = params
            .iter()
            .map(|(pname, v)| (*pname, v.to_sql_string()))
            .collect::<Vec<(&str, String)>>();

        let msg = |prefix: &str| {
            format!(
                "{} while execute_named() with the following command:\n    {}",
                prefix, sql
            )
        };

        self.rusqlite_tx
            .execute_named(
                sql,
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
