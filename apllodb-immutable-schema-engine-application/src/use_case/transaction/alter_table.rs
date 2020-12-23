use crate::use_case::{UseCase, UseCaseInput, UseCaseOutput};
use apllodb_immutable_schema_engine_domain::{
    abstract_types::ImmutableSchemaAbstractTypes,
    version::repository::VersionRepository,
    vtable::{id::VTableId, repository::VTableRepository},
};
use apllodb_shared_components::{AlterTableAction, ApllodbResult, DatabaseName, TableName};
use apllodb_storage_engine_interface::StorageEngine;
use std::{fmt::Debug, marker::PhantomData};

#[derive(Eq, PartialEq, Hash, Debug, new)]
pub struct AlterTableUseCaseInput<
    'usecase,
    'db: 'usecase,
    Engine: StorageEngine<'usecase, 'db>,
    Types: ImmutableSchemaAbstractTypes<'usecase, 'db, Engine>,
> {
    tx: &'usecase Engine::Tx,
    database_name: &'usecase DatabaseName,
    table_name: &'usecase TableName,
    action: &'usecase AlterTableAction,

    #[new(default)]
    _marker: PhantomData<(&'db (), Types)>,
}
impl<
        'usecase,
        'db: 'usecase,
        Engine: StorageEngine<'usecase, 'db>,
        Types: ImmutableSchemaAbstractTypes<'usecase, 'db, Engine>,
    > UseCaseInput for AlterTableUseCaseInput<'usecase, 'db, Engine, Types>
{
    fn validate(&self) -> ApllodbResult<()> {
        Ok(())
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct AlterTableUseCaseOutput;
impl UseCaseOutput for AlterTableUseCaseOutput {}

pub struct AlterTableUseCase<
    'usecase,
    'db: 'usecase,
    Engine: StorageEngine<'usecase, 'db>,
    Types: ImmutableSchemaAbstractTypes<'usecase, 'db, Engine>,
> {
    _marker: PhantomData<(&'usecase &'db (), Engine, Types)>,
}
impl<
        'usecase,
        'db: 'usecase,
        Engine: StorageEngine<'usecase, 'db>,
        Types: ImmutableSchemaAbstractTypes<'usecase, 'db, Engine>,
    > UseCase for AlterTableUseCase<'usecase, 'db, Engine, Types>
{
    type In = AlterTableUseCaseInput<'usecase, 'db, Engine, Types>;
    type Out = AlterTableUseCaseOutput;

    fn run_core(input: Self::In) -> ApllodbResult<Self::Out> {
        let vtable_repo = Types::VTableRepo::new(&input.tx);
        let version_repo = Types::VersionRepo::new(&input.tx);

        let vtable_id = VTableId::new(input.database_name, input.table_name);
        let mut vtable = vtable_repo.read(&vtable_id)?;
        vtable.alter(input.action)?;

        let active_versions = vtable_repo.active_versions(&vtable)?;
        let current_version = active_versions.current_version()?;
        let next_version = current_version.create_next(input.action)?; // TODO こいつの中で、PKの一部のカラムをDROPさせることは一旦UnsupportedErrorにする（他のDBMSは対応していた）

        // TODO naviテーブルに、これからinactivateされるVersionNumberが書かれていることの対処を考える
        vtable_repo.update(&vtable)?;
        version_repo.create(&next_version)?;

        Ok(AlterTableUseCaseOutput)
    }
}
