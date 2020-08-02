use apllodb_immutable_schema_engine_application::use_case::{
    transaction::{
        alter_table::{AlterTableUseCase, AlterTableUseCaseInput},
        create_table::{CreateTableUseCase, CreateTableUseCaseInput},
        full_scan::{FullScanUseCase, FullScanUseCaseInput},
        insert::{InsertUseCase, InsertUseCaseInput},
    },
    UseCase,
};
use apllodb_immutable_schema_engine_domain::{
    row::immutable_row::ImmutableRow, row_iter::ImmutableSchemaRowIter, traits::VersionRepository,
    transaction::ImmutableSchemaTx,
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

pub struct TransactionController<'tx, 'db: 'tx, Tx: ImmutableSchemaTx<'tx, 'db> + 'db> {
    tx: Tx,

    _marker: (PhantomData<&'tx &'db ()>, PhantomData<Tx>),
}

impl<'tx, 'db: 'tx, Tx: ImmutableSchemaTx<'tx, 'db> + 'db> Transaction<'tx, 'db>
    for TransactionController<'tx, 'db, Tx>
{
    type TID = Tx::TID;
    type Db = Tx::Db;
    type R = ImmutableRow;
    type RowIter = ImmutableSchemaRowIter<
        <<Tx as ImmutableSchemaTx<'tx, 'db>>::VRepo as VersionRepository<'tx, 'db>>::VerRowIter,
    >;

    fn id(&self) -> &Self::TID {
        self.tx.id()
    }

    fn begin(db: &'db mut Self::Db) -> ApllodbResult<Self>
    where
        Self: Sized,
    {
        let tx = Tx::begin(db)?;
        Ok(Self {
            tx,
            _marker: (PhantomData, PhantomData),
        })
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

    fn alter_table(
        &'tx self,
        table_name: &TableName,
        action: &AlterTableAction,
    ) -> ApllodbResult<()> {
        let database_name = self.database_name().clone();
        let input = AlterTableUseCaseInput::new(&self.tx, &database_name, table_name, action);
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
        let input = FullScanUseCaseInput::new(&self.tx, &database_name, table_name, &column_names);
        let output = FullScanUseCase::run(input)?;

        Ok(output.row_iter)
    }

    fn insert(
        &'tx self,
        table_name: &TableName,
        column_values: HashMap<ColumnName, Expression>,
    ) -> ApllodbResult<()> {
        let database_name = self.database_name().clone();
        let input = InsertUseCaseInput::new(&self.tx, &database_name, table_name, &column_values);
        let _ = InsertUseCase::run(input)?;

        Ok(())
    }

    fn update(&'tx self, _table_name: &TableName) -> ApllodbResult<()> {
        todo!()
    }
    fn delete(&'tx self, _table_name: &TableName) -> ApllodbResult<()> {
        todo!()
    }
}
