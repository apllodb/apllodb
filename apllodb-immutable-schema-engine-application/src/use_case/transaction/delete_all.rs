use crate::use_case::{UseCase, UseCaseInput, UseCaseOutput};
use apllodb_immutable_schema_engine_domain::{
    transaction::ImmutableSchemaTransaction,
    vtable::{id::VTableId, repository::VTableRepository},
};
use apllodb_shared_components::{
    data_structure::{DatabaseName, TableName},
    error::ApllodbResult,
};
use apllodb_storage_engine_interface::StorageEngine;

use std::{fmt::Debug, marker::PhantomData};

#[derive(Eq, PartialEq, Debug, new)]
pub struct DeleteAllUseCaseInput<'a, 'tx, Engine: StorageEngine> {
    tx: tx: &'tx Engine::Tx,'tx Types::ImmutableSchemaTx,

    database_name: &'a DatabaseName,
    table_name: &'a TableName,
}
impl<'a, 'tx, Engine: StorageEngine> UseCaseInput for DeleteAllUseCaseInput<'a, 'tx, Engine> {
    fn validate(&self) -> ApllodbResult<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct DeleteAllUseCaseOutput;
impl UseCaseOutput for DeleteAllUseCaseOutput {}

pub struct DeleteAllUseCase<'a, 'tx, Engine: StorageEngine> {
    _marker: PhantomData<&'a &'tx Engine>,
}
impl<'a, 'tx, Engine: StorageEngine> UseCase for DeleteAllUseCase<'a, 'tx, Engine> {
    type In = DeleteAllUseCaseInput<'a, 'tx, Engine>;
    type Out = DeleteAllUseCaseOutput;

    fn run_core(input: Self::In) -> ApllodbResult<Self::Out> {
        let vtable_repo = input.tx.vtable_repo();
        let vtable_id = VTableId::new(input.database_name, input.table_name);
        let vtable = vtable_repo.read(&vtable_id)?;

        vtable_repo.delete_all(&vtable)?;

        Ok(DeleteAllUseCaseOutput)
    }
}
