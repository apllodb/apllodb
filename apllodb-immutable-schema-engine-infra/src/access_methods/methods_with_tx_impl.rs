use crate::{
    external_interface::ApllodbImmutableSchemaEngine,
    sqlite::{sqlite_types::SqliteTypes, transaction::sqlite_tx::SqliteTx},
    tx_repo::TxRepo,
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
    AlterTableAction, ApllodbResult, ColumnDefinition, ColumnName, DatabaseName, Expression,
    RecordIterator, SessionWithTx, TableConstraints, TableName,
};
use apllodb_storage_engine_interface::{MethodsWithTx, ProjectionQuery};

#[derive(Debug)]
pub struct MethodsWithTxImpl<'sess> {
    tx_repo: &'sess mut TxRepo<'sess>,
}

impl<'sess> MethodsWithTxImpl<'sess> {
    pub(crate) fn new(tx_repo: &'sess mut TxRepo<'sess>) -> Self {
        Self { tx_repo }
    }

    fn database_name(&self, _session: &SessionWithTx) -> &DatabaseName {
        todo!()
    }

    fn sqlite_tx(&self, _session: &SessionWithTx) -> &SqliteTx {
        todo!()
    }

    fn remove_sqlite_tx(&mut self, session: &SessionWithTx) -> ApllodbResult<SqliteTx> {
        let sid = { session.get_id().clone() };

        let sqlite_tx = self.tx_repo.remove(&sid).expect(&format!(
            "no one should remove tid `{:?}` from tx_repo",
            sid
        ));
        Ok(sqlite_tx)
    }
}

impl MethodsWithTx for MethodsWithTxImpl<'_> {
    fn create_table(
        &self,
        session: &SessionWithTx,
        table_name: &TableName,
        table_constraints: &TableConstraints,
        column_definitions: Vec<ColumnDefinition>,
    ) -> ApllodbResult<()> {
        let input = CreateTableUseCaseInput::new(
            self.database_name(session),
            table_name,
            table_constraints,
            &column_definitions,
        );
        let tx = self.sqlite_tx(session);
        let _ = CreateTableUseCase::<'_, '_, ApllodbImmutableSchemaEngine, SqliteTypes>::run(
            &tx.vtable_repo(),
            &tx.version_repo(),
            input,
        )?;
        Ok(())
    }

    fn alter_table(
        &self,
        session: &SessionWithTx,
        table_name: &TableName,
        action: &AlterTableAction,
    ) -> ApllodbResult<()> {
        let input = AlterTableUseCaseInput::new(self.database_name(session), table_name, action);
        let tx = self.sqlite_tx(session);
        let _ = AlterTableUseCase::<'_, '_, ApllodbImmutableSchemaEngine, SqliteTypes>::run(
            &tx.vtable_repo(),
            &tx.version_repo(),
            input,
        )?;
        Ok(())
    }

    fn drop_table(&self, _session: &SessionWithTx, _table_name: &TableName) -> ApllodbResult<()> {
        todo!()
    }

    fn select(
        &self,
        session: &SessionWithTx,
        table_name: &TableName,
        projection: ProjectionQuery,
    ) -> ApllodbResult<RecordIterator> {
        let input = FullScanUseCaseInput::new(self.database_name(session), table_name, projection);
        let tx = self.sqlite_tx(session);
        let output = FullScanUseCase::<'_, '_, ApllodbImmutableSchemaEngine, SqliteTypes>::run(
            &tx.vtable_repo(),
            &tx.version_repo(),
            input,
        )?;

        Ok(RecordIterator::new(output.row_iter))
    }

    fn insert(
        &self,
        session: &SessionWithTx,
        table_name: &TableName,
        records: RecordIterator,
    ) -> ApllodbResult<()> {
        let input = InsertUseCaseInput::new(self.database_name(session), table_name, records);
        let tx = self.sqlite_tx(session);
        let _ = InsertUseCase::<'_, '_, ApllodbImmutableSchemaEngine, SqliteTypes>::run(
            &tx.vtable_repo(),
            &tx.version_repo(),
            input,
        )?;
        Ok(())
    }

    fn update(
        &self,
        session: &SessionWithTx,
        table_name: &TableName,
        column_values: std::collections::HashMap<ColumnName, Expression>,
    ) -> ApllodbResult<()> {
        let input =
            UpdateAllUseCaseInput::new(self.database_name(session), table_name, column_values);
        let tx = self.sqlite_tx(session);
        let _ = UpdateAllUseCase::<'_, '_, ApllodbImmutableSchemaEngine, SqliteTypes>::run(
            &tx.vtable_repo(),
            &tx.version_repo(),
            input,
        )?;

        Ok(())
    }

    fn delete(&self, session: &SessionWithTx, table_name: &TableName) -> ApllodbResult<()> {
        let input = DeleteAllUseCaseInput::new(self.database_name(session), table_name);
        let tx = self.sqlite_tx(session);
        let _ = DeleteAllUseCase::<'_, '_, ApllodbImmutableSchemaEngine, SqliteTypes>::run(
            &tx.vtable_repo(),
            &tx.version_repo(),
            input,
        )?;

        Ok(())
    }

    fn commit(mut self, session: SessionWithTx) -> ApllodbResult<()> {
        self.remove_sqlite_tx(&session)?.commit()
    }

    fn abort(mut self, session: SessionWithTx) -> ApllodbResult<()> {
        self.remove_sqlite_tx(&session)?.abort()
    }
}
