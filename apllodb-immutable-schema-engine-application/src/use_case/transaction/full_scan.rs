use crate::use_case::{TxUseCase, UseCaseInput, UseCaseOutput};
use apllodb_immutable_schema_engine_domain::abstract_types::ImmutableSchemaAbstractTypes;
use apllodb_immutable_schema_engine_domain::{
    query::projection::ProjectionResult,
    vtable::{id::VTableId, repository::VTableRepository},
};
use apllodb_shared_components::{ApllodbResult, DatabaseName, TableName};
use apllodb_storage_engine_interface::{ProjectionQuery, StorageEngine};
use async_trait::async_trait;
use std::{fmt::Debug, marker::PhantomData};

#[derive(Eq, PartialEq, Debug, new)]
pub struct FullScanUseCaseInput<'usecase> {
    database_name: &'usecase DatabaseName,
    table_name: &'usecase TableName,
    projection: ProjectionQuery,
}
impl<'usecase> UseCaseInput for FullScanUseCaseInput<'usecase> {
    fn validate(&self) -> ApllodbResult<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct FullScanUseCaseOutput<Types: ImmutableSchemaAbstractTypes> {
    pub row_iter: Types::ImmutableSchemaRowIter,
}
impl<Types: ImmutableSchemaAbstractTypes> UseCaseOutput for FullScanUseCaseOutput<Types> {}

pub struct FullScanUseCase<'usecase, Types: ImmutableSchemaAbstractTypes> {
    _marker: PhantomData<(&'usecase (), Types)>,
}

#[async_trait(?Send)]
impl<'usecase, Types: ImmutableSchemaAbstractTypes> TxUseCase<Types>
    for FullScanUseCase<'usecase, Types>
{
    type In = FullScanUseCaseInput<'usecase>;
    type Out = FullScanUseCaseOutput<Types>;

    /// # Failures
    ///
    /// - [FeatureNotSupported](apllodb_shared_components::ApllodbErrorKind::FeatureNotSupported) when:
    ///   - any column_values' Expression is not a ConstantVariant.
    async fn run_core(
        vtable_repo: &Types::VTableRepo,
        _version_repo: &Types::VersionRepo,
        input: Self::In,
    ) -> ApllodbResult<Self::Out> {
        let vtable_id = VTableId::new(input.database_name, input.table_name);
        let vtable = vtable_repo.read(&vtable_id).await?;

        let active_versions = vtable_repo.active_versions(&vtable).await?;

        let projection_result: ProjectionResult =
            ProjectionResult::new(&vtable, active_versions, input.projection)?;
        let row_iter = vtable_repo.full_scan(&vtable, projection_result).await?;
        Ok(FullScanUseCaseOutput { row_iter })
    }
}
