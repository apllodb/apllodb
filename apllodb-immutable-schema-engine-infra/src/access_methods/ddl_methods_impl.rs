use crate::{external_interface::ApllodbImmutableSchemaEngine, sqlite::sqlite_types::SqliteTypes};
use apllodb_immutable_schema_engine_application::use_case::transaction::{
    alter_table::{AlterTableUseCase, AlterTableUseCaseInput},
    create_table::{CreateTableUseCase, CreateTableUseCaseInput},
};
use apllodb_immutable_schema_engine_application::use_case::TxUseCase;
use apllodb_shared_components::{
    AlterTableAction, ApllodbResult, ColumnDefinition, SessionWithDb, TableConstraints, TableName,
};
use apllodb_storage_engine_interface::DDLMethods;

use super::transaction_methods_impl::tx_repo::TxRepo;

#[derive(Debug)]
pub struct DDLMethodsImpl<'sess> {
    tx_repo: &'sess TxRepo<'sess>,
}

impl<'sess> DDLMethodsImpl<'sess> {
    pub(crate) fn new(tx_repo: &'sess mut TxRepo<'sess>) -> Self {
        Self { tx_repo }
    }
}

impl DDLMethods for DDLMethodsImpl<'_> {
    fn create_table(
        &self,
        session: &SessionWithTx,
        table_name: &TableName,
        table_constraints: &TableConstraints,
        column_definitions: Vec<ColumnDefinition>,
    ) -> ApllodbResult<()> {
        let database_name = session.get_db().clone();
        let input = CreateTableUseCaseInput::new(
            &database_name,
            table_name,
            table_constraints,
            &column_definitins,
        );
        let tx = self.tx_repo.get(session.get_tid()?)?;
        let _ = CreateTableUseCase::<'_, ApllodbImmutableSchemaEngine, SqliteTypes>::run(
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
        let database_name = session.get_db().clone();
        let input = AlterTableUseCaseInput::new(&database_name, table_name, action);
        let tx = self.tx_repo.get(session.get_tid()?)?;
        let _ = AlterTableUseCase::<'_, ApllodbImmutableSchemaEngine, SqliteTypes>::run(
            &tx.vtable_repo(),
            &tx.version_repo(),
            input,
        )?;
        Ok(())
    }

    fn drop_table(&self, _session: &SessionWithTx, _table_name: &TableName) -> ApllodbResult<()> {
        todo!()
    }
}
