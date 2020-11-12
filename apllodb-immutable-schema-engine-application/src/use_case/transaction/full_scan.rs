use crate::use_case::{UseCase, UseCaseInput, UseCaseOutput};
use apllodb_immutable_schema_engine_domain::{
    abstract_types::ImmutableSchemaAbstractTypes,
    row::column::non_pk_column::filter_non_pk_column_names,
};
use apllodb_immutable_schema_engine_domain::{
    transaction::ImmutableSchemaTx,
    vtable::{id::VTableId, repository::VTableRepository},
};
use apllodb_shared_components::{
    data_structure::{ColumnName, DatabaseName, TableName},
    error::ApllodbResult,
};
use apllodb_storage_engine_interface::StorageEngine;

use std::{fmt::Debug, marker::PhantomData};

#[derive(Eq, PartialEq, Debug, new)]
pub struct FullScanUseCaseInput<
    'a: 'tx,
    'tx: 'db,
    'db,
    Engine: StorageEngine,
    Types: ImmutableSchemaAbstractTypes<'tx, 'db, Engine>,
> {
    tx: &'a Engine::Tx,
    database_name: &'a DatabaseName,
    table_name: &'a TableName,
    column_names: &'a [ColumnName],

    #[new(default)]
    _marker: PhantomData<&'db &'tx Types>,
}
impl<
        'a: 'tx,
        'tx: 'db,
        'db,
        Engine: StorageEngine,
        Types: ImmutableSchemaAbstractTypes<'tx, 'db, Engine>,
    > UseCaseInput for FullScanUseCaseInput<'a, 'tx, 'db, Engine, Types>
{
    fn validate(&self) -> ApllodbResult<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct FullScanUseCaseOutput<
    'tx,
    'db: 'tx,
    Engine: StorageEngine,
    Types: ImmutableSchemaAbstractTypes<'tx, 'db, Engine>,
> {
    pub row_iter: Types::ImmutableSchemaRowIter,
}
impl<
        'tx,
        'db: 'tx,
        Engine: StorageEngine,
        Types: ImmutableSchemaAbstractTypes<'tx, 'db, Engine>,
    > UseCaseOutput for FullScanUseCaseOutput<'tx, 'db, Engine, Types>
{
}

pub struct FullScanUseCase<
    'a: 'tx,
    'tx: 'db,
    'db,
    Engine: StorageEngine,
    Types: ImmutableSchemaAbstractTypes<'tx, 'db, Engine>,
> {
    _marker: PhantomData<(&'a &'tx &'db Types, Engine)>,
}
impl<
        'a: 'tx,
        'tx: 'db,
        'db,
        Engine: StorageEngine,
        Types: ImmutableSchemaAbstractTypes<'tx, 'db, Engine>,
    > UseCase for FullScanUseCase<'a, 'tx, 'db, Engine, Types>
{
    type In = FullScanUseCaseInput<'a, 'tx, 'db, Engine, Types>;
    type Out = FullScanUseCaseOutput<'tx, 'db, Engine, Types>;

    /// # Failures
    ///
    /// - [FeatureNotSupported](error/enum.ApllodbErrorKind.html#variant.FeatureNotSupported) when:
    ///   - any column_values' Expression is not a ConstantVariant.
    fn run_core(input: Self::In) -> ApllodbResult<Self::Out> {
        let vtable_repo = input.tx.vtable_repo();

        let vtable_id = VTableId::new(input.database_name, input.table_name);
        let vtable = vtable_repo.read(&vtable_id)?;
        let non_pk_column_names = filter_non_pk_column_names(
            input.column_names,
            &vtable.table_wide_constraints().pk_column_names(),
        );

        let row_iter = vtable_repo.full_scan(&vtable_id, &non_pk_column_names)?;
        Ok(FullScanUseCaseOutput { row_iter })
    }
}
