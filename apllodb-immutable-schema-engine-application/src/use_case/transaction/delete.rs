use crate::use_case::{TxUseCase, UseCaseInput, UseCaseOutput};
use apllodb_immutable_schema_engine_domain::{
    abstract_types::ImmutableSchemaAbstractTypes,
    vtable::{id::VTableId, repository::VTableRepository},
};
use apllodb_shared_components::{ApllodbError, ApllodbResult, DatabaseName};
use apllodb_storage_engine_interface::{RowSelectionQuery, TableName};
use async_trait::async_trait;
use std::{fmt::Debug, marker::PhantomData};

#[derive(PartialEq, Debug, new)]
pub struct DeleteUseCaseInput<'usecase> {
    database_name: &'usecase DatabaseName,
    table_name: &'usecase TableName,
    selection: &'usecase RowSelectionQuery,
}
impl<'usecase> UseCaseInput for DeleteUseCaseInput<'usecase> {
    fn validate(&self) -> ApllodbResult<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct DeleteUseCaseOutput;
impl UseCaseOutput for DeleteUseCaseOutput {}

pub struct DeleteUseCase<'usecase, Types: ImmutableSchemaAbstractTypes> {
    _marker: PhantomData<(&'usecase (), Types)>,
}

#[async_trait(?Send)]
impl<'usecase, Types: ImmutableSchemaAbstractTypes> TxUseCase<Types>
    for DeleteUseCase<'usecase, Types>
{
    type In = DeleteUseCaseInput<'usecase>;
    type Out = DeleteUseCaseOutput;

    async fn run_core(
        vtable_repo: &Types::VTableRepo,
        _version_repo: &Types::VersionRepo,
        input: Self::In,
    ) -> ApllodbResult<Self::Out> {
        let vtable_id = VTableId::new(input.database_name, input.table_name);
        let vtable = vtable_repo.read(&vtable_id).await?;

        match input.selection {
            RowSelectionQuery::FullScan => vtable_repo._delete_all(&vtable).await,
            RowSelectionQuery::Condition(_) => Err(ApllodbError::feature_not_supported(
                "DELETE ... WHERE ... is not supported currently",
            )),
        }?;

        Ok(DeleteUseCaseOutput)
    }
}
