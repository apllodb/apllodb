use crate::use_case::{UseCase, UseCaseInput, UseCaseOutput};
use apllodb_immutable_schema_engine_domain::{
    abstract_types::ImmutableSchemaAbstractTypes,
    row::column::non_pk_column::filter_non_pk_column_names,
};
use apllodb_immutable_schema_engine_domain::{
    transaction::ImmutableSchemaTransaction,
    vtable::{id::VTableId, repository::VTableRepository},
};
use apllodb_shared_components::{
    data_structure::{ColumnName, DatabaseName, TableName},
    error::ApllodbResult,
};
use apllodb_storage_engine_interface::StorageEngine;

use std::{fmt::Debug, marker::PhantomData};

#[derive(Eq, PartialEq, Debug, new)]
pub struct FullScanUseCaseInput<'a, 'tx, Engine: StorageEngine> {
    tx: tx: &'tx Engine::Tx,'tx Types::ImmutableSchemaTx,

    database_name: &'a DatabaseName,
    table_name: &'a TableName,
    column_names: &'a [ColumnName],
}
impl<'a, 'tx, Engine: StorageEngine> UseCaseInput for FullScanUseCaseInput<'a, 'tx, Engine> {
    fn validate(&self) -> ApllodbResult<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct FullScanUseCaseOutput<Engine: StorageEngine, Types: ImmutableSchemaAbstractTypes<Engine>>
{
    pub row_iter: Types::ImmutableSchemaRowIter,
}
impl<'tx, Engine: StorageEngine, Types: ImmutableSchemaAbstractTypes<Engine>> UseCaseOutput
    for FullScanUseCaseOutput<Engine, Types>
{
}

pub struct FullScanUseCase<
    'a,
    'tx,
    Engine: StorageEngine,
    Types: ImmutableSchemaAbstractTypes<Engine>,
> {
    _marker: PhantomData<(&'a &'tx Engine, Types)>,
}
impl<'a, 'tx, Engine: StorageEngine, Types: ImmutableSchemaAbstractTypes<Engine>> UseCase
    for FullScanUseCase<'a, 'tx, Engine, Types>
{
    type In = FullScanUseCaseInput<'a, 'tx, Engine>;
    type Out = FullScanUseCaseOutput<Engine, Types>;

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
