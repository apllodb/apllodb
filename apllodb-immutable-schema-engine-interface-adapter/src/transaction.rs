use apllodb_immutable_schema_engine_application::use_case::{
    transaction::{
        alter_table::{AlterTableUseCase, AlterTableUseCaseInput},
        create_table::{CreateTableUseCase, CreateTableUseCaseInput},
        delete_all::{DeleteAllUseCase, DeleteAllUseCaseInput},
        full_scan::{FullScanUseCase, FullScanUseCaseInput},
        insert::{InsertUseCase, InsertUseCaseInput},
        update_all::{UpdateAllUseCase, UpdateAllUseCaseInput},
    },
    UseCase,
};
use apllodb_immutable_schema_engine_domain::transaction::ImmutableSchemaTx;
use apllodb_shared_components::{
    data_structure::{
        AlterTableAction, ColumnDefinition, ColumnName, DatabaseName, Expression, TableConstraints,
        TableName,
    },
    error::ApllodbResult,
};
use apllodb_storage_engine_interface::{StorageEngine, Transaction};
use std::{collections::HashMap, marker::PhantomData};

#[derive(Hash, Debug, new)]
pub struct TransactionController<'tx, 'db: 'tx, Engine: StorageEngine> {
    tx: Engine::Tx,

    #[new(default)]
    _marker: PhantomData<&'tx &'db ()>,
}

impl<'tx, 'db: 'tx, Engine: StorageEngine> Transaction<'tx, 'db, Engine>
    for TransactionController<'tx, 'db, Engine>
{
    fn id(&self) -> &Engine::TID {
        self.tx.id()
    }

    fn begin(db: &'db mut Engine::Db) -> ApllodbResult<Self>
    where
        Self: Sized,
    {
        let tx = Engine::Tx::begin(db)?;
        Ok(Self::new(tx))
    }

    fn commit(self) -> ApllodbResult<()> {
        self.tx.commit()
    }
    fn abort(self) -> ApllodbResult<()> {
        self.tx.abort()
    }

    fn database_name(&self) -> &DatabaseName {
        self.tx.database_name()
    }

    fn create_table(
        &self,
        table_name: &TableName,
        table_constraints: &TableConstraints,
        column_definitions: &[ColumnDefinition],
    ) -> ApllodbResult<()> {
        let database_name = self.database_name().clone();
        let input = CreateTableUseCaseInput::new(
            &self.tx,
            &database_name,
            table_name,
            table_constraints,
            column_definitions,
        );
        let _ = CreateTableUseCase::run(input)?;

        Ok(())
    }

    fn alter_table(&self, table_name: &TableName, action: &AlterTableAction) -> ApllodbResult<()> {
        let database_name = self.database_name().clone();
        let input = AlterTableUseCaseInput::new(&self.tx, &database_name, table_name, action);
        let _ = AlterTableUseCase::run(input)?;

        Ok(())
    }

    fn drop_table(&self, _table_name: &TableName) -> ApllodbResult<()> {
        todo!()
    }

    fn select(
        &self,
        table_name: &TableName,
        column_names: &[ColumnName],
    ) -> ApllodbResult<Engine::RowIter> {
        let database_name = self.database_name().clone();
        let input = FullScanUseCaseInput::new(&self.tx, &database_name, table_name, &column_names);
        let output = FullScanUseCase::run(input)?;

        Ok(output.row_iter)
    }

    fn insert(
        &self,
        table_name: &TableName,
        column_values: HashMap<ColumnName, Expression>,
    ) -> ApllodbResult<()> {
        let database_name = self.database_name().clone();
        let input = InsertUseCaseInput::new(&self.tx, &database_name, table_name, &column_values);
        let _ = InsertUseCase::run(input)?;

        Ok(())
    }

    fn update(
        &self,
        table_name: &TableName,
        column_values: HashMap<ColumnName, Expression>,
    ) -> ApllodbResult<()> {
        let database_name = self.database_name().clone();
        let input =
            UpdateAllUseCaseInput::new(&self.tx, &database_name, table_name, &column_values);
        let _ = UpdateAllUseCase::run(input)?;

        Ok(())
    }

    fn delete(&self, table_name: &TableName) -> ApllodbResult<()> {
        let database_name = self.database_name().clone();
        let input = DeleteAllUseCaseInput::new(&self.tx, &database_name, table_name);
        let _ = DeleteAllUseCase::run(input)?;

        Ok(())
    }
}
