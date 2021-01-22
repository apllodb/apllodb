use std::{cell::RefCell, rc::Rc};

use crate::sqlite::{
    sqlite_resource_pool::tx_pool::SqliteTxPool, sqlite_types::SqliteTypes,
    transaction::sqlite_tx::SqliteTx,
};
use apllodb_immutable_schema_engine_application::use_case::transaction::create_table::{
    CreateTableUseCase, CreateTableUseCaseInput,
};
use apllodb_immutable_schema_engine_application::use_case::TxUseCase;
use apllodb_shared_components::{ColumnDefinition, SessionWithTx, TableConstraints, TableName};
use futures::FutureExt;

use super::FutRes;

#[derive(Clone, Debug, Default)]
pub struct WithTxMethodsImpl {
    tx_pool: Rc<RefCell<SqliteTxPool>>,
}

impl WithTxMethodsImpl {
    pub(crate) fn new(tx_pool: Rc<RefCell<SqliteTxPool>>) -> Self {
        Self { tx_pool }
    }

    // ========================================================================
    // Transaction
    // ========================================================================
    pub fn commit_transaction(self, session: SessionWithTx) -> FutRes<()> {
        async move {
            let mut tx_pool = self.tx_pool.borrow_mut();
            let tx = tx_pool.remove_tx(session.get_id())?;
            tx.borrow_mut().commit().await?;
            Ok(())
        }
        .boxed_local()
    }

    // ========================================================================
    // DDL
    // ========================================================================
    pub fn create_table(
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
}
