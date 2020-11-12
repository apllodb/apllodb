use crate::use_case::{UseCase, UseCaseInput, UseCaseOutput};
use apllodb_immutable_schema_engine_domain::{
    abstract_types::ImmutableSchemaAbstractTypes,
    transaction::ImmutableSchemaTransaction,
    version::repository::VersionRepository,
    vtable::{id::VTableId, repository::VTableRepository},
};
use apllodb_shared_components::{
    data_structure::{AlterTableAction, DatabaseName, TableName},
    error::ApllodbResult,
};
use apllodb_storage_engine_interface::StorageEngine;
use std::{fmt::Debug, marker::PhantomData};

#[derive(Eq, PartialEq, Hash, Debug, new)]
pub struct AlterTableUseCaseInput<
    'a,
    'tx: 'a,
    'db: 'tx,
    Engine: StorageEngine<'db>,
    Types: ImmutableSchemaAbstractTypes<'tx, 'db, Engine>,
> {
    tx: &'a Types::ImmutableSchemaTx,
    database_name: &'a DatabaseName,
    table_name: &'a TableName,
    action: &'a AlterTableAction,

    #[new(default)]
    _marker: PhantomData<&'tx &'db ()>,
}
impl<
        'a,
        'tx: 'a,
        'db: 'tx,
        Engine: StorageEngine<'db>,
        Types: ImmutableSchemaAbstractTypes<'tx, 'db, Engine>,
    > UseCaseInput for AlterTableUseCaseInput<'a, 'tx, 'db, Engine, Types>
{
    fn validate(&self) -> ApllodbResult<()> {
        Ok(())
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct AlterTableUseCaseOutput;
impl UseCaseOutput for AlterTableUseCaseOutput {}

pub struct AlterTableUseCase<
    'a,
    'tx: 'a,
    'db: 'tx,
    Engine: StorageEngine<'db>,
    Types: ImmutableSchemaAbstractTypes<'tx, 'db, Engine>,
> {
    _marker: PhantomData<(&'a &'tx &'db (), Types, Engine)>,
}
impl<
        'a,
        'tx: 'a,
        'db: 'tx,
        Engine: StorageEngine<'db>,
        Types: ImmutableSchemaAbstractTypes<'tx, 'db, Engine>,
    > UseCase for AlterTableUseCase<'a, 'tx, 'db, Engine, Types>
{
    type In = AlterTableUseCaseInput<'a, 'tx, 'db, Engine, Types>;
    type Out = AlterTableUseCaseOutput;

    fn run_core(input: Self::In) -> ApllodbResult<Self::Out> {
        let vtable_repo = input.tx.vtable_repo();
        let version_repo = input.tx.version_repo();

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
