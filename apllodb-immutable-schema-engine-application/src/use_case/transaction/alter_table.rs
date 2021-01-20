use crate::use_case::{TxUseCase, UseCaseInput, UseCaseOutput};
use apllodb_immutable_schema_engine_domain::{
    abstract_types::ImmutableSchemaAbstractTypes,
    version::repository::VersionRepository,
    vtable::{id::VTableId, repository::VTableRepository},
};
use apllodb_shared_components::{AlterTableAction, ApllodbResult, DatabaseName, TableName};
use apllodb_storage_engine_interface::StorageEngine;
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

pub struct AlterTableUseCase<
    'usecase,
    Engine: StorageEngine,
    Types: ImmutableSchemaAbstractTypes<Engine>,
> {
    _marker: PhantomData<(&'usecase (), Engine, Types)>,
}

#[async_trait(?Send)]
impl<
        'usecase,
        'db: 'usecase,
        Engine: StorageEngine,
        Types: ImmutableSchemaAbstractTypes<Engine>,
    > TxUseCase<Engine, Types> for AlterTableUseCase<'usecase, Engine, Types>
{
    type In = AlterTableUseCaseInput<'usecase>;
    type Out = AlterTableUseCaseOutput;

    async fn run_core(
        vtable_repo: &Types::VTableRepo,
        version_repo: &Types::VersionRepo,
        input: Self::In,
    ) -> ApllodbResult<Self::Out> {
        let vtable_id = VTableId::new(input.database_name, input.table_name);
        let mut vtable = vtable_repo.read(&vtable_id).await?;
        vtable.alter(input.action)?;

        let active_versions = vtable_repo.active_versions(&vtable).await?;
        let current_version = active_versions.current_version()?;
        let next_version = current_version.create_next(input.action)?; // TODO こいつの中で、PKの一部のカラムをDROPさせることは一旦UnsupportedErrorにする（他のDBMSは対応していた）

        // TODO naviテーブルに、これからinactivateされるVersionNumberが書かれていることの対処を考える
        vtable_repo.update(&vtable).await?;
        version_repo.create(&next_version).await?;

        Ok(AlterTableUseCaseOutput)
    }
}
