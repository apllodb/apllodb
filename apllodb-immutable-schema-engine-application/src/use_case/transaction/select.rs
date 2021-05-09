use crate::use_case::{TxUseCase, UseCaseInput, UseCaseOutput};
use apllodb_immutable_schema_engine_domain::{
    abstract_types::ImmutableSchemaAbstractTypes, row_selection_plan::RowSelectionPlan,
};
use apllodb_immutable_schema_engine_domain::{
    row_projection_result::RowProjectionResult,
    vtable::{id::VTableId, repository::VTableRepository},
};
use apllodb_shared_components::{ApllodbResult, DatabaseName};
use apllodb_storage_engine_interface::{RowProjectionQuery, Rows, TableName};
use async_trait::async_trait;
use std::{fmt::Debug, marker::PhantomData};

#[derive(PartialEq, Debug, new)]
pub struct SelectUseCaseInput<'usecase, Types: ImmutableSchemaAbstractTypes> {
    database_name: &'usecase DatabaseName,
    table_name: &'usecase TableName,
    projection: RowProjectionQuery,
    selection: RowSelectionPlan<Types>,
}
impl<'usecase, Types: ImmutableSchemaAbstractTypes> UseCaseInput
    for SelectUseCaseInput<'usecase, Types>
{
    fn validate(&self) -> ApllodbResult<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct SelectUseCaseOutput {
    pub rows: Rows,
}
impl UseCaseOutput for SelectUseCaseOutput {}

pub struct SelectUseCase<'usecase, Types: ImmutableSchemaAbstractTypes> {
    _marker: PhantomData<(&'usecase (), Types)>,
}

#[async_trait(?Send)]
impl<'usecase, Types: ImmutableSchemaAbstractTypes + 'usecase> TxUseCase<Types>
    for SelectUseCase<'usecase, Types>
{
    type In = SelectUseCaseInput<'usecase, Types>;
    type Out = SelectUseCaseOutput;

    /// # Failures
    ///
    /// - [FeatureNotSupported](apllodb_shared_components::SqlState::FeatureNotSupported) when:
    ///   - any column_values' Expression is not a ConstantVariant.
    async fn run_core(
        vtable_repo: &Types::VTableRepo,
        _version_repo: &Types::VersionRepo,
        input: Self::In,
    ) -> ApllodbResult<Self::Out> {
        let vtable_id = VTableId::new(input.database_name, input.table_name);
        let vtable = vtable_repo.read(&vtable_id).await?;

        let active_versions = vtable_repo.active_versions(&vtable).await?;

        let projection_result =
            RowProjectionResult::new(&vtable, active_versions, &input.projection)?;
        let rows = vtable_repo
            .select(&vtable, projection_result, input.selection)
            .await?;

        Ok(SelectUseCaseOutput { rows })
    }
}
