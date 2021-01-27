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
use apllodb_shared_components::{
    AlterTableAction, ColumnDefinition, ColumnName, Expression, RecordIterator, SessionWithDb,
    SessionWithTx, TableConstraints, TableName,
};
use apllodb_storage_engine_interface::{ProjectionQuery, WithTxMethods};
use futures::FutureExt;

use super::FutRes;

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
    fn commit_transaction(self, session: SessionWithTx) -> FutRes<SessionWithDb> {
        async move {
            let mut tx_pool = self.tx_pool.borrow_mut();
            let tx = tx_pool.remove_tx(session.get_id())?;
            tx.borrow_mut().commit().await?;
            Ok(session.downgrade())
        }
        .boxed_local()
    }

    fn abort_transaction(self, session: SessionWithTx) -> FutRes<SessionWithDb> {
        async move {
            let mut tx_pool = self.tx_pool.borrow_mut();
            let tx = tx_pool.remove_tx(session.get_id())?;
            tx.borrow_mut().abort().await?;
            Ok(session.downgrade())
        }
        .boxed_local()
    }

    // ========================================================================
    // DDL
    // ========================================================================
    fn create_table(
        self,
        session: SessionWithTx,
        table_name: TableName,
        table_constraints: TableConstraints,
        column_definitions: Vec<ColumnDefinition>,
    ) -> FutRes<SessionWithTx> {
        async move {
            let tx_pool = self.tx_pool.borrow();
            let tx = tx_pool.get_tx(session.get_id())?;

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

            Ok(session)
        }
        .boxed_local()
    }

    fn alter_table(
        self,
        session: SessionWithTx,
        table_name: TableName,
        action: AlterTableAction,
    ) -> FutRes<SessionWithTx> {
        async move {
            let tx_pool = self.tx_pool.borrow();
            let tx = tx_pool.get_tx(session.get_id())?;

            let database_name = tx.borrow().database_name().clone();
            let input = AlterTableUseCaseInput::new(&database_name, &table_name, &action);
            AlterTableUseCase::<'_, SqliteTypes>::run(
                &SqliteTx::vtable_repo(tx.clone()),
                &SqliteTx::version_repo(tx.clone()),
                input,
            )
            .await?;

            Ok(session)
        }
        .boxed_local()
    }

    fn drop_table(self, _session: SessionWithTx, _table_name: TableName) -> FutRes<SessionWithTx> {
        async move { todo!() }.boxed_local()
    }

    // ========================================================================
    // DML
    // ========================================================================
    fn select(
        self,
        session: SessionWithTx,
        table_name: TableName,
        projection: ProjectionQuery,
    ) -> FutRes<(RecordIterator, SessionWithTx)> {
        async move {
            let tx_pool = self.tx_pool.borrow();
            let tx = tx_pool.get_tx(session.get_id())?;

            let database_name = tx.borrow().database_name().clone();
            let input = FullScanUseCaseInput::new(&database_name, &table_name, projection);
            let output = FullScanUseCase::<'_, SqliteTypes>::run(
                &SqliteTx::vtable_repo(tx.clone()),
                &SqliteTx::version_repo(tx.clone()),
                input,
            )
            .await?;

            Ok((RecordIterator::new(output.row_iter), session))
        }
        .boxed_local()
    }

    fn insert(
        self,
        session: SessionWithTx,
        table_name: TableName,
        records: RecordIterator,
    ) -> FutRes<SessionWithTx> {
        async move {
            let tx_pool = self.tx_pool.borrow();
            let tx = tx_pool.get_tx(session.get_id())?;

            let database_name = tx.borrow().database_name().clone();
            let input = InsertUseCaseInput::new(&database_name, &table_name, records);
            InsertUseCase::<'_, SqliteTypes>::run(
                &SqliteTx::vtable_repo(tx.clone()),
                &SqliteTx::version_repo(tx.clone()),
                input,
            )
            .await?;

            Ok(session)
        }
        .boxed_local()
    }

    fn update(
        self,
        session: SessionWithTx,
        table_name: TableName,
        column_values: HashMap<ColumnName, Expression>,
    ) -> FutRes<SessionWithTx> {
        async move {
            let tx_pool = self.tx_pool.borrow();
            let tx = tx_pool.get_tx(session.get_id())?;

            let database_name = tx.borrow().database_name().clone();
            let input = UpdateAllUseCaseInput::new(&database_name, &table_name, column_values);
            UpdateAllUseCase::<'_, SqliteTypes>::run(
                &SqliteTx::vtable_repo(tx.clone()),
                &SqliteTx::version_repo(tx.clone()),
                input,
            )
            .await?;

            Ok(session)
        }
        .boxed_local()
    }

    fn delete(self, session: SessionWithTx, table_name: TableName) -> FutRes<SessionWithTx> {
        async move {
            let tx_pool = self.tx_pool.borrow();
            let tx = tx_pool.get_tx(session.get_id())?;

            let database_name = tx.borrow().database_name().clone();
            let input = DeleteAllUseCaseInput::new(&database_name, &table_name);
            DeleteAllUseCase::<'_, SqliteTypes>::run(
                &SqliteTx::vtable_repo(tx.clone()),
                &SqliteTx::version_repo(tx.clone()),
                input,
            )
            .await?;

            Ok(session)
        }
        .boxed_local()
    }
}
