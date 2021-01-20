use crate::use_case::{TxUseCase, UseCaseInput, UseCaseOutput};
use apllodb_immutable_schema_engine_domain::{
    abstract_types::ImmutableSchemaAbstractTypes,
    vtable::{id::VTableId, repository::VTableRepository},
};
use apllodb_shared_components::{ApllodbResult, DatabaseName, TableName};
use apllodb_storage_engine_interface::StorageEngine;
use async_trait::async_trait;
use std::{fmt::Debug, marker::PhantomData};

#[derive(Eq, PartialEq, Debug, new)]
pub struct DeleteAllUseCaseInput<'usecase> {
    database_name: &'usecase DatabaseName,
    table_name: &'usecase TableName,
}
impl<'usecase> UseCaseInput for DeleteAllUseCaseInput<'usecase> {
    fn validate(&self) -> ApllodbResult<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct DeleteAllUseCaseOutput;
impl UseCaseOutput for DeleteAllUseCaseOutput {}

pub struct DeleteAllUseCase<
    'usecase,
    Engine: StorageEngine,
    Types: ImmutableSchemaAbstractTypes<Engine>,
> {
    _marker: PhantomData<(&'usecase (), Engine, Types)>,
}

#[async_trait(?Send)]
impl<'usecase, Engine: StorageEngine, Types: ImmutableSchemaAbstractTypes<Engine>>
    TxUseCase<Engine, Types> for DeleteAllUseCase<'usecase, Engine, Types>
{
    type In = DeleteAllUseCaseInput<'usecase>;
    type Out = DeleteAllUseCaseOutput;

    async fn run_core(
        vtable_repo: &Types::VTableRepo,
        _version_repo: &Types::VersionRepo,
        input: Self::In,
    ) -> ApllodbResult<Self::Out> {
        let vtable_id = VTableId::new(input.database_name, input.table_name);
        let vtable = vtable_repo.read(&vtable_id).await?;

        vtable_repo.delete_all(&vtable).await?;

        Ok(DeleteAllUseCaseOutput)
    }
}
