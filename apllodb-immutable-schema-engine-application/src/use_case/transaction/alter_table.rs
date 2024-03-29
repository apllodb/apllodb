use crate::use_case::{TxUseCase, UseCaseInput, UseCaseOutput};
use apllodb_immutable_schema_engine_domain::{
    abstract_types::ImmutableSchemaAbstractTypes,
    version::repository::VersionRepository,
    vtable::{id::VTableId, repository::VTableRepository},
};
use apllodb_shared_components::{ApllodbResult, DatabaseName};
use apllodb_storage_engine_interface::{AlterTableAction, TableName};
use async_trait::async_trait;
use std::{fmt::Debug, marker::PhantomData};

#[derive(Eq, PartialEq, Hash, Debug, new)]
pub struct AlterTableUseCaseInput<'usecase> {
    database_name: &'usecase DatabaseName,
    table_name: &'usecase TableName,
    action: &'usecase AlterTableAction,
}
impl<'usecase> UseCaseInput for AlterTableUseCaseInput<'usecase> {
    fn validate(&self) -> ApllodbResult<()> {
        Ok(())
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct AlterTableUseCaseOutput;
impl UseCaseOutput for AlterTableUseCaseOutput {}

pub struct AlterTableUseCase<'usecase, Types: ImmutableSchemaAbstractTypes> {
    _marker: PhantomData<(&'usecase (), Types)>,
}

#[async_trait(?Send)]
impl<'usecase, Types: ImmutableSchemaAbstractTypes> TxUseCase<Types>
    for AlterTableUseCase<'usecase, Types>
{
    type In = AlterTableUseCaseInput<'usecase>;
    type Out = AlterTableUseCaseOutput;

    async fn run_core(
        vtable_repo: &Types::VTableRepo,
        version_repo: &Types::VersionRepo,
        input: Self::In,
    ) -> ApllodbResult<Self::Out> {
        let vtable_id = VTableId::new(input.database_name, input.table_name);
        let vtable = vtable_repo.read(&vtable_id).await?;
        vtable.alter(input.action)?;

        let active_versions = vtable_repo.active_versions(&vtable).await?;
        let current_version = active_versions.current_version()?;
        let next_version = current_version.create_next(input.action)?; // TODO At first, DROPping any column included in PK should be UnsupportedError (although other DBMSs can).

        // TODO `ALTER TABLE` may inactivate target version (auto-upgrade).
        // Needs to do something to navi table (update to new VersionNumber?).
        vtable_repo.update(&vtable).await?;
        version_repo.create(&next_version).await?;

        Ok(AlterTableUseCaseOutput)
    }
}
