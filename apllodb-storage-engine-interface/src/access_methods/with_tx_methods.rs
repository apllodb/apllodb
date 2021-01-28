use std::collections::HashMap;

use apllodb_shared_components::{
    AlterTableAction, ApllodbResult, ApllodbSessionError, ApllodbSessionResult, ColumnDefinition,
    ColumnName, Expression, RecordIterator, Session, SessionId, SessionWithDb, SessionWithTx,
    TableConstraints, TableName,
};
use futures::FutureExt;

use crate::ProjectionQuery;

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
        projection: ProjectionQuery,
    ) -> BoxFut<ApllodbSessionResult<(RecordIterator, SessionWithTx)>> {
        let sid = *session.get_id();
        async move {
            match self.select_core(sid, table_name, projection).await {
                Ok(records) => Ok((records, session)),
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
        projection: ProjectionQuery,
    ) -> BoxFut<ApllodbResult<RecordIterator>>;

    fn insert(
        self,
        session: SessionWithTx,
        table_name: TableName,
        records: RecordIterator,
    ) -> BoxFut<ApllodbSessionResult<SessionWithTx>> {
        let sid = *session.get_id();
        async move {
            match self.insert_core(sid, table_name, records).await {
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
        records: RecordIterator,
    ) -> BoxFut<ApllodbResult<()>>;

    fn update(
        self,
        session: SessionWithTx,
        table_name: TableName,
        column_values: HashMap<ColumnName, Expression>,
    ) -> BoxFut<ApllodbSessionResult<SessionWithTx>> {
        let sid = *session.get_id();
        async move {
            match self.update_core(sid, table_name, column_values).await {
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
    ) -> BoxFut<ApllodbResult<()>>;

    fn delete(
        self,
        session: SessionWithTx,
        table_name: TableName,
    ) -> BoxFut<ApllodbSessionResult<SessionWithTx>> {
        let sid = *session.get_id();
        async move {
            match self.delete_core(sid, table_name).await {
                Ok(_) => Ok(session),
                Err(e) => Err(ApllodbSessionError::new(e, Session::from(session))),
            }
        }
        .boxed_local()
    }

    #[doc(hidden)]
    fn delete_core(self, sid: SessionId, table_name: TableName) -> BoxFut<ApllodbResult<()>>;
}
