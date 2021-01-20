use crate::use_case::{TxUseCase, UseCaseInput, UseCaseOutput};
use apllodb_immutable_schema_engine_domain::{
    abstract_types::ImmutableSchemaAbstractTypes,
    row::column::filter_non_pk_column_definitions,
    version::active_version::ActiveVersion,
    version::repository::VersionRepository,
    vtable::{repository::VTableRepository, VTable},
};
use apllodb_shared_components::{
    ApllodbResult, ColumnDataType, ColumnDefinition, DatabaseName, TableConstraints, TableName,
};
use apllodb_storage_engine_interface::StorageEngine;
use async_trait::async_trait;
use std::{fmt::Debug, marker::PhantomData};

#[derive(Eq, PartialEq, Hash, Debug, new)]
pub struct CreateTableUseCaseInput<'usecase> {
    database_name: &'usecase DatabaseName,
    table_name: &'usecase TableName,
    table_constraints: &'usecase TableConstraints,
    column_definitions: &'usecase [ColumnDefinition],
}
impl<'usecase> UseCaseInput for CreateTableUseCaseInput<'usecase> {
    fn validate(&self) -> ApllodbResult<()> {
        Ok(())
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct CreateTableUseCaseOutput;
impl UseCaseOutput for CreateTableUseCaseOutput {}

pub struct CreateTableUseCase<'usecase, Types: ImmutableSchemaAbstractTypes> {
    _marker: PhantomData<(&'usecase (), Types)>,
}

#[async_trait(?Send)]
impl<'usecase, Types: ImmutableSchemaAbstractTypes> TxUseCase<Types>
    for CreateTableUseCase<'usecase, Types>
{
    type In = CreateTableUseCaseInput<'usecase>;
    type Out = CreateTableUseCaseOutput;

    async fn run_core(
        vtable_repo: &Types::VTableRepo,
        version_repo: &Types::VersionRepo,
        input: Self::In,
    ) -> ApllodbResult<Self::Out> {
        use apllodb_immutable_schema_engine_domain::entity::Entity;

        let vtable = VTable::create(
            input.database_name,
            input.table_name,
            input.table_constraints,
            input.column_definitions,
        )?;

        let apk_column_names = vtable.table_wide_constraints().pk_column_names();
        let column_data_types: Vec<ColumnDataType> =
            filter_non_pk_column_definitions(input.column_definitions, &apk_column_names)
                .iter()
                .map(|coldef| coldef.into())
                .collect();

        let v1 = ActiveVersion::initial(vtable.id(), &column_data_types)?;

        vtable_repo.create(&vtable).await?;
        version_repo.create(&v1).await?;

        Ok(CreateTableUseCaseOutput)
    }
}
