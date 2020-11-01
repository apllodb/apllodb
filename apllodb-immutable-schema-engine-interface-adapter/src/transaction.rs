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
use apllodb_immutable_schema_engine_domain::{
    abstract_types::AbstractTypes, row::immutable_row::ImmutableRow, transaction::ImmutableSchemaTx,
};
use apllodb_shared_components::{
    data_structure::{
        AlterTableAction, ColumnDefinition, ColumnName, DatabaseName, Expression, TableConstraints,
        TableName,
    },
    error::ApllodbResult,
};
use apllodb_storage_engine_interface::Transaction;
use std::{collections::HashMap, marker::PhantomData};

#[derive(Hash, Debug, new)]
pub struct TransactionController<'tx, 'db: 'tx, Types: AbstractTypes<'tx, 'db>> {
    tx: Types::Tx,

    #[new(default)]
    _marker: PhantomData<&'tx &'db ()>,
}

impl<'tx, 'db: 'tx, Types: AbstractTypes<'tx, 'db>> Transaction<'tx, 'db>
    for TransactionController<'tx, 'db, Types>
{
    type TID = Types::TID;
    type Db = Types::Db;
    type R = ImmutableRow;
    type RowIter = Types::ImmutableSchemaRowIter;

    fn id(&self) -> &Types::TID {
        self.tx.id()
    }

    fn begin(db: &'db mut Self::Db) -> ApllodbResult<Self>
    where
        Self: Sized,
    {
        let tx = Types::Tx::begin(db)?;
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
        &'tx self,
        table_name: &TableName,
        table_constraints: &TableConstraints,
        column_definitions: &[ColumnDefinition],
    ) -> ApllodbResult<()> {
        let database_name = self.database_name().clone();
        let input: CreateTableUseCaseInput<'_, '_, '_, Types> = CreateTableUseCaseInput::new(
            &self.tx,
            &database_name,
            table_name,
            table_constraints,
            column_definitions,
        );
        let _ = CreateTableUseCase::run(input)?;

        Ok(())
    }

    fn alter_table(
        &'tx self,
        table_name: &TableName,
        action: &AlterTableAction,
    ) -> ApllodbResult<()> {
        let database_name = self.database_name().clone();
        let input: AlterTableUseCaseInput<'_, '_, '_, Types> =
            AlterTableUseCaseInput::new(&self.tx, &database_name, table_name, action);
        let _ = AlterTableUseCase::run(input)?;

        Ok(())
    }

    fn drop_table(&'tx self, _table_name: &TableName) -> ApllodbResult<()> {
        todo!()
    }

    fn select(
        &'tx self,
        table_name: &TableName,
        column_names: &[ColumnName],
    ) -> ApllodbResult<Self::RowIter> {
        let database_name = self.database_name().clone();
        let input: FullScanUseCaseInput<'_, '_, '_, Types> =
            FullScanUseCaseInput::new(&self.tx, &database_name, table_name, &column_names);
        let output = FullScanUseCase::run(input)?;

        Ok(output.row_iter)
    }

    fn insert(
        &'tx self,
        table_name: &TableName,
        column_values: HashMap<ColumnName, Expression>,
    ) -> ApllodbResult<()> {
        let database_name = self.database_name().clone();
        let input: InsertUseCaseInput<'_, '_, '_, Types> =
            InsertUseCaseInput::new(&self.tx, &database_name, table_name, &column_values);
        let _ = InsertUseCase::run(input)?;

        Ok(())
    }

    fn update(
        &'tx self,
        table_name: &TableName,
        column_values: HashMap<ColumnName, Expression>,
    ) -> ApllodbResult<()> {
        let database_name = self.database_name().clone();
        let input: UpdateAllUseCaseInput<'_, '_, '_, Types> =
            UpdateAllUseCaseInput::new(&self.tx, &database_name, table_name, &column_values);
        let _ = UpdateAllUseCase::run(input)?;

        Ok(())
    }

    fn delete(&'tx self, table_name: &TableName) -> ApllodbResult<()> {
        let database_name = self.database_name().clone();
        let input: DeleteAllUseCaseInput<'_, '_, '_, Types> =
            DeleteAllUseCaseInput::new(&self.tx, &database_name, table_name);
        let _ = DeleteAllUseCase::run(input)?;

        Ok(())
    }
}
