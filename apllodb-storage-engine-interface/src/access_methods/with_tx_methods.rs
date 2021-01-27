use std::collections::HashMap;

use apllodb_shared_components::{
    AlterTableAction, ColumnDefinition, ColumnName, Expression, RecordIterator, SessionWithDb,
    SessionWithTx, TableConstraints, TableName,
};

use crate::ProjectionQuery;

use super::FutRes;

#[cfg_attr(feature = "test-support", mockall::automock)]
pub trait WithTxMethods {
    // ========================================================================
    // Transaction
    // ========================================================================
    fn commit_transaction(self, session: SessionWithTx) -> FutRes<SessionWithDb>;

    fn abort_transaction(self, session: SessionWithTx) -> FutRes<SessionWithDb>;

    // ========================================================================
    // DDL
    // ========================================================================
    fn create_table(
        self,
        session: SessionWithTx,
        table_name: TableName,
        table_constraints: TableConstraints,
        column_definitions: Vec<ColumnDefinition>,
    ) -> FutRes<SessionWithTx>;

    fn alter_table(
        self,
        session: SessionWithTx,
        table_name: TableName,
        action: AlterTableAction,
    ) -> FutRes<SessionWithTx>;

    fn drop_table(self, _session: SessionWithTx, _table_name: TableName) -> FutRes<SessionWithTx>;

    // ========================================================================
    // DML
    // ========================================================================
    fn select(
        self,
        session: SessionWithTx,
        table_name: TableName,
        projection: ProjectionQuery,
    ) -> FutRes<(RecordIterator, SessionWithTx)>;

    fn insert(
        self,
        session: SessionWithTx,
        table_name: TableName,
        records: RecordIterator,
    ) -> FutRes<SessionWithTx>;

    fn update(
        self,
        session: SessionWithTx,
        table_name: TableName,
        column_values: HashMap<ColumnName, Expression>,
    ) -> FutRes<SessionWithTx>;

    fn delete(self, session: SessionWithTx, table_name: TableName) -> FutRes<SessionWithTx>;
}
