use apllodb_immutable_schema_engine_application::use_case::transaction::{
    delete_all::{DeleteAllUseCase, DeleteAllUseCaseInput},
    full_scan::{FullScanUseCase, FullScanUseCaseInput},
    insert::{InsertUseCase, InsertUseCaseInput},
    update_all::{UpdateAllUseCase, UpdateAllUseCaseInput},
};
use apllodb_immutable_schema_engine_application::use_case::TxUseCase;
use apllodb_shared_components::{ApllodbResult, SessionWithDb, TableName};
use apllodb_shared_components::{ColumnName, Expression, RecordIterator};
use apllodb_storage_engine_interface::DMLMethods;
use apllodb_storage_engine_interface::ProjectionQuery;

use crate::{external_interface::ApllodbImmutableSchemaEngine, sqlite::sqlite_types::SqliteTypes};

use super::transaction_methods_impl::tx_repo::TxRepo;

#[derive(Debug)]
pub struct DMLMethodsImpl<'sess> {
    tx_repo: &'sess TxRepo<'sess>,
}

impl<'sess> DMLMethodsImpl<'sess> {
    pub(crate) fn new(tx_repo: &'sess mut TxRepo<'sess>) -> Self {
        Self { tx_repo }
    }
}

impl DMLMethods for DMLMethodsImpl<'_> {
    fn select(
        &self,
        session: &mut SessionWithDb,
        table_name: &TableName,
        projection: ProjectionQuery,
    ) -> ApllodbResult<RecordIterator> {
        let database_name = session.get_db().clone();
        let input = FullScanUseCaseInput::new(&database_name, table_name, projection);
        let tx = self.tx_repo.get(session.get_tid()?)?;
        let output = FullScanUseCase::<'_, ApllodbImmutableSchemaEngine, SqliteTypes>::run(
            &tx.vtable_repo(),
            &tx.version_repo(),
            input,
        )?;

        Ok(RecordIterator::new(output.row_iter))
    }

    fn insert(
        &self,
        session: &mut SessionWithDb,
        table_name: &TableName,
        records: RecordIterator,
    ) -> ApllodbResult<()> {
        let database_name = session.get_db().clone();
        let input = InsertUseCaseInput::new(&database_name, table_name, records);
        let tx = self.tx_repo.get(session.get_tid()?)?;
        let _ = InsertUseCase::<'_, ApllodbImmutableSchemaEngine, SqliteTypes>::run(
            &tx.vtable_repo(),
            &tx.version_repo(),
            input,
        )?;
        Ok(())
    }

    fn update(
        &self,
        session: &mut SessionWithDb,
        table_name: &TableName,
        column_values: std::collections::HashMap<ColumnName, Expression>,
    ) -> ApllodbResult<()> {
        let database_name = session.get_db().clone();
        let input = UpdateAllUseCaseInput::new(&database_name, table_name, column_values);
        let tx = self.tx_repo.get(session.get_tid()?)?;
        let _ = UpdateAllUseCase::<'_, ApllodbImmutableSchemaEngine, SqliteTypes>::run(
            &tx.vtable_repo(),
            &tx.version_repo(),
            input,
        )?;

        Ok(())
    }

    fn delete(&self, session: &mut SessionWithDb, table_name: &TableName) -> ApllodbResult<()> {
        let database_name = session.get_db().clone();
        let input = DeleteAllUseCaseInput::new(&database_name, table_name);
        let tx = self.tx_repo.get(session.get_tid()?)?;
        let _ = DeleteAllUseCase::<'_, ApllodbImmutableSchemaEngine, SqliteTypes>::run(
            &tx.vtable_repo(),
            &tx.version_repo(),
            input,
        )?;

        Ok(())
    }
}
