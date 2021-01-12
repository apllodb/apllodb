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
use serde::{Deserialize, Serialize};

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize,
)]
pub struct DDLMethodsImpl;

impl DDLMethods for DDLMethodsImpl {
    fn create_table(
        &self,
        session: &mut SessionWithDb,
        table_name: &TableName,
        table_constraints: &TableConstraints,
        column_definitions: Vec<ColumnDefinition>,
    ) -> ApllodbResult<()> {
        let database_name = tx.database_name().clone();
        let input = CreateTableUseCaseInput::new(
            &database_name,
            table_name,
            table_constraints,
            &column_definitions,
        );
        let _ = CreateTableUseCase::<'_, ApllodbImmutableSchemaEngine, SqliteTypes>::run(
            &tx.vtable_repo(),
            &tx.version_repo(),
            input,
        )?;
        Ok(())
    }

    fn alter_table(
        &self,
        session: &mut SessionWithDb,
        table_name: &TableName,
        action: &AlterTableAction,
    ) -> ApllodbResult<()> {
        let database_name = tx.database_name().clone();
        let input = AlterTableUseCaseInput::new(&database_name, table_name, action);
        let _ = AlterTableUseCase::<'_, ApllodbImmutableSchemaEngine, SqliteTypes>::run(
            &tx.vtable_repo(),
            &tx.version_repo(),
            input,
        )?;
        Ok(())
    }

    fn drop_table(
        &self,
        _session: &mut SessionWithDb,
        _table_name: &TableName,
    ) -> ApllodbResult<()> {
        todo!()
    }
}
