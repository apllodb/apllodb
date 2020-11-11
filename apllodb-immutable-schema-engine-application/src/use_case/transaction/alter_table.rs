use crate::use_case::{UseCase, UseCaseInput, UseCaseOutput};
use apllodb_immutable_schema_engine_domain::{
    abstract_types::ImmutableSchemaAbstractTypes, vtable::id::VTableId,
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
    'tx,
    Engine: StorageEngine,
    Types: ImmutableSchemaAbstractTypes<Engine>,
> {
    tx: &'tx Types::ImmutableSchemaTx,

    database_name: &'a DatabaseName,
    table_name: &'a TableName,
    action: &'a AlterTableAction,
}
impl<'a, 'tx, Engine: StorageEngine, Types: ImmutableSchemaAbstractTypes<Engine>> UseCaseInput
    for AlterTableUseCaseInput<'a, 'tx, Engine, Types>
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
    'tx,
    Engine: StorageEngine,
    Types: ImmutableSchemaAbstractTypes<Engine>,
> {
    _marker: PhantomData<(&'a &'tx Engine, Types)>,
}
impl<'a, 'tx, Engine: StorageEngine, Types: ImmutableSchemaAbstractTypes<Engine>> UseCase
    for AlterTableUseCase<'a, 'tx, Engine, Types>
{
    type In = AlterTableUseCaseInput<'a, 'tx, Engine, Types>; //警告の内容からして、結局 Types 経由で 'tx 渡して、 ImmutableSchemaTransaction + 'tx にする必要ありそう
                                                              // じゃあ、Inputに &'tx  tx をもたせるのではなく、 run_core で &tx を持たせたらどうか。

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
