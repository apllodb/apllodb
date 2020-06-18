use apllodb_immutable_schema_engine_application::use_case::{
    transaction::create_table::{CreateTableUseCase, CreateTableUseCaseInput},
    UseCase,
};
use apllodb_immutable_schema_engine_domain::{
    ImmutableSchemaRowIter, ImmutableSchemaTx, VersionRowIter,
};
use apllodb_shared_components::{
    data_structure::{
        AlterTableAction, ColumnDefinition, ColumnName, DatabaseName, Expression, TableConstraints,
        TableName,
    },
    error::ApllodbResult,
};
use apllodb_storage_engine_interface::{Row, Transaction};
use std::marker::PhantomData;

pub struct TransactionController<
    'tx,
    'db: 'tx,
    Tx: ImmutableSchemaTx<'tx, 'db> + 'db,
    It: VersionRowIter<Item = ApllodbResult<Row>>,
> {
    tx: Tx,

    _marker: (PhantomData<&'tx &'db ()>, PhantomData<Tx>, PhantomData<It>),
}

impl<
        'tx,
        'db: 'tx,
        Tx: ImmutableSchemaTx<'tx, 'db> + 'db,
        It: VersionRowIter<Item = ApllodbResult<Row>>,
    > Transaction<'tx, 'db> for TransactionController<'tx, 'db, Tx, It>
{
    type Db = Tx::Db;
    type RowIter = ImmutableSchemaRowIter<It>;

    fn begin(db: &'db mut Self::Db) -> ApllodbResult<Self>
    where
        Self: Sized,
    {
        let tx = Tx::begin(db)?;
        Ok(Self {
            tx,
            _marker: (PhantomData, PhantomData, PhantomData),
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
        let database_name = { self.database_name().clone() };
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
        todo!()
    }
    fn drop_table(&'tx self, table_name: &TableName) -> ApllodbResult<()> {
        todo!()
    }
    fn select(
        &'tx self,
        table_name: &TableName,
        column_names: &[ColumnName],
    ) -> ApllodbResult<Self::RowIter> {
        todo!()
    }
    fn insert(
        &'tx self,
        table_name: &TableName,
        column_values: std::collections::HashMap<ColumnName, Expression>,
    ) -> ApllodbResult<()> {
        todo!()
    }
    fn update(&'tx self, table_name: &TableName) -> ApllodbResult<()> {
        todo!()
    }
    fn delete(&'tx self, table_name: &TableName) -> ApllodbResult<()> {
        todo!()
    }
}
