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
    AlterTableAction, ApllodbResult, ColumnDefinition, ColumnName, Expression, RecordIterator,
    SessionWithTx, TableConstraints, TableName,
};
use apllodb_storage_engine_interface::{MethodsWithTx, ProjectionQuery};

#[derive(Debug)]
pub struct MethodsWithTxImpl<'sess> {
    session: &'sess SessionWithTx,
    tx_repo: &'sess TxRepo<'sess>,
}

impl<'sess> MethodsWithTxImpl<'sess> {
    pub(crate) fn new(session: &'sess SessionWithTx, tx_repo: &'sess mut TxRepo<'sess>) -> Self {
        Self { session, tx_repo }
    }

    fn remove_sqlite_tx(&mut self) -> ApllodbResult<SqliteTx> {
        let session = self.session;
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
        table_name: &TableName,
        table_constraints: &TableConstraints,
        column_definitions: Vec<ColumnDefinition>,
    ) -> ApllodbResult<()> {
        let database_name = self.session.get_db().clone();
        let input = CreateTableUseCaseInput::new(
            &database_name,
            table_name,
            table_constraints,
            &column_definitions,
        );
        let tx = self.tx_repo.get(self.session.get_tid()?)?;
        let _ = CreateTableUseCase::<'_, '_, ApllodbImmutableSchemaEngine, SqliteTypes>::run(
            &tx.vtable_repo(),
            &tx.version_repo(),
            input,
        )?;
        Ok(())
    }

    fn alter_table(&self, table_name: &TableName, action: &AlterTableAction) -> ApllodbResult<()> {
        let database_name = self.session.get_db().clone();
        let input = AlterTableUseCaseInput::new(&database_name, table_name, action);
        let tx = self.tx_repo.get(self.session.get_tid()?)?;
        let _ = AlterTableUseCase::<'_, '_, ApllodbImmutableSchemaEngine, SqliteTypes>::run(
            &tx.vtable_repo(),
            &tx.version_repo(),
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
    ) -> ApllodbResult<RecordIterator> {
        let database_name = self.session.get_db().clone();
        let input = FullScanUseCaseInput::new(&database_name, table_name, projection);
        let tx = self.tx_repo.get(self.session.get_tid()?)?;
        let output = FullScanUseCase::<'_, '_, ApllodbImmutableSchemaEngine, SqliteTypes>::run(
            &tx.vtable_repo(),
            &tx.version_repo(),
            input,
        )?;

        Ok(RecordIterator::new(output.row_iter))
    }

    fn insert(&self, table_name: &TableName, records: RecordIterator) -> ApllodbResult<()> {
        let database_name = self.session.get_db().clone();
        let input = InsertUseCaseInput::new(&database_name, table_name, records);
        let tx = self.tx_repo.get(self.session.get_tid()?)?;
        let _ = InsertUseCase::<'_, '_, ApllodbImmutableSchemaEngine, SqliteTypes>::run(
            &tx.vtable_repo(),
            &tx.version_repo(),
            input,
        )?;
        Ok(())
    }

    fn update(
        &self,
        table_name: &TableName,
        column_values: std::collections::HashMap<ColumnName, Expression>,
    ) -> ApllodbResult<()> {
        let database_name = self.session.get_db().clone();
        let input = UpdateAllUseCaseInput::new(&database_name, table_name, column_values);
        let tx = self.tx_repo.get(self.session.get_tid()?)?;
        let _ = UpdateAllUseCase::<'_, '_, ApllodbImmutableSchemaEngine, SqliteTypes>::run(
            &tx.vtable_repo(),
            &tx.version_repo(),
            input,
        )?;

        Ok(())
    }

    fn delete(&self, table_name: &TableName) -> ApllodbResult<()> {
        let database_name = self.session.get_db().clone();
        let input = DeleteAllUseCaseInput::new(&database_name, table_name);
        let tx = self.tx_repo.get(self.session.get_tid()?)?;
        let _ = DeleteAllUseCase::<'_, '_, ApllodbImmutableSchemaEngine, SqliteTypes>::run(
            &tx.vtable_repo(),
            &tx.version_repo(),
            input,
        )?;

        Ok(())
    }

    fn commit(self) -> ApllodbResult<()> {
        self.remove_sqlite_tx()?.commit()
    }

    fn abort(self) -> ApllodbResult<()> {
        self.remove_sqlite_tx()?.abort()
    }
}
