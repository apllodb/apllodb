use apllodb_immutable_schema_engine_application::use_case::{
    transaction::create_table::{CreateTableUseCase, CreateTableUseCaseInput},
    UseCase,
};
use apllodb_immutable_schema_engine_domain::{
    abstract_types::ImmutableSchemaAbstractTypes, transaction::ImmutableSchemaTransaction,
};
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
pub struct TransactionController<Engine: StorageEngine, Types: ImmutableSchemaAbstractTypes<Engine>>
{
    tx: Types::ImmutableSchemaTx,

    #[new(default)]
    _marker: PhantomData<Types>,
}

impl<Engine: StorageEngine, Types: ImmutableSchemaAbstractTypes<Engine>> Transaction<Engine>
    for TransactionController<Engine, Types>
{
    fn id(&self) -> &Engine::TID {
        self.tx.id()
    }

    fn begin<'db>(db: &'db mut Engine::Db) -> ApllodbResult<Self>
    where
        Self: Sized,
    {
        let tx = Types::ImmutableSchemaTx::begin(db)?;
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
            &database_name,
            table_name,
            table_constraints,
            column_definitions,
        );
        let _ = CreateTableUseCase::<'_, Engine, Types>::run(&self.tx, input)?; // ここの type annotation は、 TransactionControllerをインフラ層に持ってきて、ストレージエンジンの型の実態が与えられれば解決するはず

        Ok(())
    }

    // fn alter_table(&self, table_name: &TableName, action: &AlterTableAction) -> ApllodbResult<()> {
    //     let database_name = self.database_name().clone();
    //     let input = AlterTableUseCaseInput::new(&self.tx, &database_name, table_name, action);
    //     let _ = AlterTableUseCase::run(input)?;

    //     Ok(())
    // }

    // fn drop_table(&self, _table_name: &TableName) -> ApllodbResult<()> {
    //     todo!()
    // }

    // fn select(
    //     &self,
    //     table_name: &TableName,
    //     column_names: &[ColumnName],
    // ) -> ApllodbResult<Engine::RowIter> {
    //     let database_name = self.database_name().clone();
    //     let input = FullScanUseCaseInput::new(&self.tx, &database_name, table_name, &column_names);
    //     let output = FullScanUseCase::run(input)?;

    //     Ok(output.row_iter)
    // }

    // fn insert(
    //     &self,
    //     table_name: &TableName,
    //     column_values: HashMap<ColumnName, Expression>,
    // ) -> ApllodbResult<()> {
    //     let database_name = self.database_name().clone();
    //     let input = InsertUseCaseInput::new(&self.tx, &database_name, table_name, &column_values);
    //     let _ = InsertUseCase::run(input)?;

    //     Ok(())
    // }

    // fn update(
    //     &self,
    //     table_name: &TableName,
    //     column_values: HashMap<ColumnName, Expression>,
    // ) -> ApllodbResult<()> {
    //     let database_name = self.database_name().clone();
    //     let input =
    //         UpdateAllUseCaseInput::new(&self.tx, &database_name, table_name, &column_values);
    //     let _ = UpdateAllUseCase::run(input)?;

    //     Ok(())
    // }

    // fn delete(&self, table_name: &TableName) -> ApllodbResult<()> {
    //     let database_name = self.database_name().clone();
    //     let input = DeleteAllUseCaseInput::new(&self.tx, &database_name, table_name);
    //     let _ = DeleteAllUseCase::run(input)?;

    //     Ok(())
    // }
}
