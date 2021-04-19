use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::sqlite::{
    sqlite_resource_pool::{db_pool::SqliteDatabasePool, tx_pool::SqliteTxPool},
    sqlite_types::SqliteTypes,
    transaction::sqlite_tx::SqliteTx,
};
use apllodb_immutable_schema_engine_application::use_case::transaction::{
    alter_table::{AlterTableUseCase, AlterTableUseCaseInput},
    create_table::{CreateTableUseCase, CreateTableUseCaseInput},
    delete_all::{DeleteAllUseCase, DeleteAllUseCaseInput},
    full_scan::{FullScanUseCase, FullScanUseCaseInput},
    insert::{InsertUseCase, InsertUseCaseInput},
    update_all::{UpdateAllUseCase, UpdateAllUseCaseInput},
};
use apllodb_immutable_schema_engine_application::use_case::TxUseCase;
use apllodb_shared_components::{ApllodbError, Expression, SessionId};
use apllodb_storage_engine_interface::{
    AlterTableAction, ColumnDefinition, ColumnName, Row, RowProjectionQuery, Rows,
    TableConstraints, TableName, WithTxMethods,
};
use futures::FutureExt;

use super::BoxFutRes;

#[derive(Clone, Debug, Default)]
pub struct WithTxMethodsImpl {
    db_pool: Rc<RefCell<SqliteDatabasePool>>,
    tx_pool: Rc<RefCell<SqliteTxPool>>,
}

impl WithTxMethodsImpl {
    pub(crate) fn new(
        db_pool: Rc<RefCell<SqliteDatabasePool>>,
        tx_pool: Rc<RefCell<SqliteTxPool>>,
    ) -> Self {
        Self { db_pool, tx_pool }
    }
}

impl WithTxMethods for WithTxMethodsImpl {
    // ========================================================================
    // Transaction
    // ========================================================================
    fn commit_transaction_core(self, sid: SessionId) -> BoxFutRes<()> {
        async move {
            let mut tx_pool = self.tx_pool.borrow_mut();
            let tx = tx_pool.remove_tx(&sid)?;
            tx.borrow_mut().commit().await?;
            Ok(())
        }
        .boxed_local()
    }

    fn abort_transaction_core(self, sid: SessionId) -> BoxFutRes<()> {
        async move {
            let mut tx_pool = self.tx_pool.borrow_mut();
            let tx = tx_pool.remove_tx(&sid)?;
            tx.borrow_mut().abort().await?;
            Ok(())
        }
        .boxed_local()
    }

    // ========================================================================
    // DDL
    // ========================================================================
    fn create_table_core(
        self,
        sid: SessionId,
        table_name: TableName,
        table_constraints: TableConstraints,
        column_definitions: Vec<ColumnDefinition>,
    ) -> BoxFutRes<()> {
        async move {
            let tx_pool = self.tx_pool.borrow();
            let tx = tx_pool.get_tx(&sid)?;

            let database_name = tx.borrow().database_name().clone();
            let input = CreateTableUseCaseInput::new(
                &database_name,
                &table_name,
                &table_constraints,
                &column_definitions,
            );

            CreateTableUseCase::<'_, SqliteTypes>::run(
                &SqliteTx::vtable_repo(tx.clone()),
                &SqliteTx::version_repo(tx.clone()),
                input,
            )
            .await?;

            Ok(())
        }
        .boxed_local()
    }

    fn alter_table_core(
        self,
        sid: SessionId,
        table_name: TableName,
        action: AlterTableAction,
    ) -> BoxFutRes<()> {
        async move {
            let tx_pool = self.tx_pool.borrow();
            let tx = tx_pool.get_tx(&sid)?;

            let database_name = tx.borrow().database_name().clone();
            let input = AlterTableUseCaseInput::new(&database_name, &table_name, &action);
            AlterTableUseCase::<'_, SqliteTypes>::run(
                &SqliteTx::vtable_repo(tx.clone()),
                &SqliteTx::version_repo(tx.clone()),
                input,
            )
            .await?;

            Ok(())
        }
        .boxed_local()
    }

    fn drop_table_core(self, _sid: SessionId, _table_name: TableName) -> BoxFutRes<()> {
        async move {
            Err(ApllodbError::feature_not_supported(
                "DROP TABLE is not supported currently",
            ))
        }
        .boxed_local()
    }

    // ========================================================================
    // DML
    // ========================================================================
    fn select_core(
        self,
        sid: SessionId,
        table_name: TableName,
        projection: RowProjectionQuery,
        selection: RowSelectionQuery,
    ) -> BoxFutRes<Rows> {
        async move {
            let tx_pool = self.tx_pool.borrow();
            let tx = tx_pool.get_tx(&sid)?;

            let database_name = tx.borrow().database_name().clone();
            let input = FullScanUseCaseInput::new(&database_name, &table_name, projection);
            let output = FullScanUseCase::<'_, SqliteTypes>::run(
                &SqliteTx::vtable_repo(tx.clone()),
                &SqliteTx::version_repo(tx.clone()),
                input,
            )
            .await?;

            Ok(output.rows)
        }
        .boxed_local()
    }

    fn insert_core(
        self,
        sid: SessionId,
        table_name: TableName,
        column_names: Vec<ColumnName>,
        rows: Vec<Row>,
    ) -> BoxFutRes<()> {
        async move {
            let tx_pool = self.tx_pool.borrow();
            let tx = tx_pool.get_tx(&sid)?;

            let database_name = tx.borrow().database_name().clone();
            let input = InsertUseCaseInput::new(&database_name, &table_name, &column_names, rows);
            InsertUseCase::<'_, SqliteTypes>::run(
                &SqliteTx::vtable_repo(tx.clone()),
                &SqliteTx::version_repo(tx.clone()),
                input,
            )
            .await?;

            Ok(())
        }
        .boxed_local()
    }

    fn update_core(
        self,
        sid: SessionId,
        table_name: TableName,
        column_values: HashMap<ColumnName, Expression>,
        selection: RowSelectionQuery,
    ) -> BoxFutRes<()> {
        async move {
            let tx_pool = self.tx_pool.borrow();
            let tx = tx_pool.get_tx(&sid)?;

            let database_name = tx.borrow().database_name().clone();
            let input = UpdateAllUseCaseInput::new(&database_name, &table_name, column_values);
            UpdateAllUseCase::<'_, SqliteTypes>::run(
                &SqliteTx::vtable_repo(tx.clone()),
                &SqliteTx::version_repo(tx.clone()),
                input,
            )
            .await?;

            Ok(())
        }
        .boxed_local()
    }

    fn delete_core(self, sid: SessionId, table_name: TableName) -> BoxFutRes<()> {
        async move {
            let tx_pool = self.tx_pool.borrow();
            let tx = tx_pool.get_tx(&sid)?;

            let database_name = tx.borrow().database_name().clone();
            let input = DeleteAllUseCaseInput::new(&database_name, &table_name);
            DeleteAllUseCase::<'_, SqliteTypes>::run(
                &SqliteTx::vtable_repo(tx.clone()),
                &SqliteTx::version_repo(tx.clone()),
                input,
            )
            .await?;

            Ok(())
        }
        .boxed_local()
    }
}
