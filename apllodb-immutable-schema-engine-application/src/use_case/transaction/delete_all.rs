use crate::use_case::{UseCase, UseCaseInput, UseCaseOutput};
use apllodb_immutable_schema_engine_domain::{
    abstract_types::AbstractTypes,
    transaction::ImmutableSchemaTx,
    vtable::{id::VTableId, repository::VTableRepository},
};
use apllodb_shared_components::{
    data_structure::{DatabaseName, TableName},
    error::ApllodbResult,
};

use std::{fmt::Debug, marker::PhantomData};

#[derive(Eq, PartialEq, Debug, new)]
pub struct DeleteAllUseCaseInput<'a, 'tx, 'db: 'tx, Types: AbstractTypes<'tx, 'db>> {
    tx: &'tx Types::Tx,

    database_name: &'a DatabaseName,
    table_name: &'a TableName,

    #[new(default)]
    _marker: PhantomData<&'db ()>,
}
impl<'a, 'tx, 'db: 'tx, Types: AbstractTypes<'tx, 'db>> UseCaseInput
    for DeleteAllUseCaseInput<'a, 'tx, 'db, Types>
{
    fn validate(&self) -> ApllodbResult<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct DeleteAllUseCaseOutput;
impl UseCaseOutput for DeleteAllUseCaseOutput {}

pub struct DeleteAllUseCase<'a, 'tx, 'db: 'tx, Types: AbstractTypes<'tx, 'db>> {
    _marker: PhantomData<&'a &'tx &'db Types>,
}
impl<'a, 'tx, 'db: 'tx, Types: AbstractTypes<'tx, 'db>> UseCase
    for DeleteAllUseCase<'a, 'tx, 'db, Types>
{
    type In = DeleteAllUseCaseInput<'a, 'tx, 'db, Types>;
    type Out = DeleteAllUseCaseOutput;

    fn run_core(input: Self::In) -> ApllodbResult<Self::Out> {
        let vtable_repo = input.tx.vtable_repo();
        let vtable_id = VTableId::new(input.database_name, input.table_name);
        let vtable = vtable_repo.read(&vtable_id)?;

        vtable_repo.delete_all(&vtable)?;

        Ok(DeleteAllUseCaseOutput)
    }
}
