use std::collections::HashMap;

use apllodb_shared_components::{
    ApllodbResult, ApllodbSessionError, ApllodbSessionResult, Expression, Session, SessionId,
    SessionWithDb, SessionWithTx,
};
use futures::FutureExt;

use crate::{
    alter_table_action::AlterTableAction,
    column::{column_definition::ColumnDefinition, column_name::ColumnName},
    row_selection_query::RowSelectionQuery,
    rows::row::Row,
    table::{table_constraints::TableConstraints, table_name::TableName},
    RowProjectionQuery, Rows,
};

use super::BoxFut;

#[cfg_attr(feature = "test-support", mockall::automock)]
pub trait WithTxMethods: Sized + 'static {
    // ========================================================================
    // Transaction
    // ========================================================================
    fn commit_transaction(
        self,
        session: SessionWithTx,
    ) -> BoxFut<ApllodbSessionResult<SessionWithDb>> {
        let sid = *session.get_id();
        async move {
            match self.commit_transaction_core(sid).await {
                Ok(_) => Ok(session.downgrade()),
                Err(e) => Err(ApllodbSessionError::new(e, Session::from(session))),
            }
        }
        .boxed_local()
    }

    #[doc(hidden)]
    fn commit_transaction_core(self, sid: SessionId) -> BoxFut<ApllodbResult<()>>;

    fn abort_transaction(
        self,
        session: SessionWithTx,
    ) -> BoxFut<ApllodbSessionResult<SessionWithDb>> {
        let sid = *session.get_id();
        async move {
            match self.abort_transaction_core(sid).await {
                Ok(_) => Ok(session.downgrade()),
                Err(e) => Err(ApllodbSessionError::new(e, Session::from(session))),
            }
        }
        .boxed_local()
    }

    #[doc(hidden)]
    fn abort_transaction_core(self, sid: SessionId) -> BoxFut<ApllodbResult<()>>;

    // ========================================================================
    // DDL
    // ========================================================================
    fn create_table(
        self,
        session: SessionWithTx,
        table_name: TableName,
        table_constraints: TableConstraints,
        column_definitions: Vec<ColumnDefinition>,
    ) -> BoxFut<ApllodbSessionResult<SessionWithTx>> {
        let sid = *session.get_id();
        async move {
            match self
                .create_table_core(sid, table_name, table_constraints, column_definitions)
                .await
            {
                Ok(_) => Ok(session),
                Err(e) => Err(ApllodbSessionError::new(e, Session::from(session))),
            }
        }
        .boxed_local()
    }

    #[doc(hidden)]
    fn create_table_core(
        self,
        sid: SessionId,
        table_name: TableName,
        table_constraints: TableConstraints,
        column_definitions: Vec<ColumnDefinition>,
    ) -> BoxFut<ApllodbResult<()>>;

    fn alter_table(
        self,
        session: SessionWithTx,
        table_name: TableName,
        action: AlterTableAction,
    ) -> BoxFut<ApllodbSessionResult<SessionWithTx>> {
        let sid = *session.get_id();
        async move {
            match self.alter_table_core(sid, table_name, action).await {
                Ok(_) => Ok(session),
                Err(e) => Err(ApllodbSessionError::new(e, Session::from(session))),
            }
        }
        .boxed_local()
    }

    #[doc(hidden)]
    fn alter_table_core(
        self,
        sid: SessionId,
        table_name: TableName,
        action: AlterTableAction,
    ) -> BoxFut<ApllodbResult<()>>;

    fn drop_table(
        self,
        session: SessionWithTx,
        table_name: TableName,
    ) -> BoxFut<ApllodbSessionResult<SessionWithTx>> {
        let sid = *session.get_id();
        async move {
            match self.drop_table_core(sid, table_name).await {
                Ok(_) => Ok(session),
                Err(e) => Err(ApllodbSessionError::new(e, Session::from(session))),
            }
        }
        .boxed_local()
    }

    #[doc(hidden)]
    fn drop_table_core(self, sid: SessionId, table_name: TableName) -> BoxFut<ApllodbResult<()>>;

    // ========================================================================
    // DML
    // ========================================================================
    fn select(
        self,
        session: SessionWithTx,
        table_name: TableName,
        projection: RowProjectionQuery,
        selection: RowSelectionQuery,
    ) -> BoxFut<ApllodbSessionResult<(Rows, SessionWithTx)>> {
        let sid = *session.get_id();
        async move {
            match self
                .select_core(sid, table_name, projection, selection)
                .await
            {
                Ok(rows) => Ok((rows, session)),
                Err(e) => Err(ApllodbSessionError::new(e, Session::from(session))),
            }
        }
        .boxed_local()
    }

    #[doc(hidden)]
    fn select_core(
        self,
        sid: SessionId,
        table_name: TableName,
        projection: RowProjectionQuery,
        selection: RowSelectionQuery,
    ) -> BoxFut<ApllodbResult<Rows>>;

    fn insert(
        self,
        session: SessionWithTx,
        table_name: TableName,
        column_names: Vec<ColumnName>,
        values: Vec<Row>,
    ) -> BoxFut<ApllodbSessionResult<SessionWithTx>> {
        let sid = *session.get_id();
        async move {
            match self
                .insert_core(sid, table_name, column_names, values)
                .await
            {
                Ok(_) => Ok(session),
                Err(e) => Err(ApllodbSessionError::new(e, Session::from(session))),
            }
        }
        .boxed_local()
    }

    #[doc(hidden)]
    fn insert_core(
        self,
        sid: SessionId,
        table_name: TableName,
        column_names: Vec<ColumnName>,
        values: Vec<Row>,
    ) -> BoxFut<ApllodbResult<()>>;

    fn update(
        self,
        session: SessionWithTx,
        table_name: TableName,
        column_values: HashMap<ColumnName, Expression>,
        selection: RowSelectionQuery,
    ) -> BoxFut<ApllodbSessionResult<SessionWithTx>> {
        let sid = *session.get_id();
        async move {
            match self
                .update_core(sid, table_name, column_values, selection)
                .await
            {
                Ok(_) => Ok(session),
                Err(e) => Err(ApllodbSessionError::new(e, Session::from(session))),
            }
        }
        .boxed_local()
    }

    #[doc(hidden)]
    fn update_core(
        self,
        sid: SessionId,
        table_name: TableName,
        column_values: HashMap<ColumnName, Expression>,
        selection: RowSelectionQuery,
    ) -> BoxFut<ApllodbResult<()>>;

    fn delete(
        self,
        session: SessionWithTx,
        table_name: TableName,
        selection: RowSelectionQuery,
    ) -> BoxFut<ApllodbSessionResult<SessionWithTx>> {
        let sid = *session.get_id();
        async move {
            match self.delete_core(sid, table_name, selection).await {
                Ok(_) => Ok(session),
                Err(e) => Err(ApllodbSessionError::new(e, Session::from(session))),
            }
        }
        .boxed_local()
    }

    #[doc(hidden)]
    fn delete_core(
        self,
        sid: SessionId,
        table_name: TableName,
        selection: RowSelectionQuery,
    ) -> BoxFut<ApllodbResult<()>>;
}
