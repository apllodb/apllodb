use crate::use_case::{UseCase, UseCaseInput, UseCaseOutput};
use apllodb_immutable_schema_engine_domain::{ImmutableSchemaTx, VTableId};
use apllodb_immutable_schema_engine_domain::{VTableRepository, VersionRepository};
use apllodb_shared_components::{
    data_structure::{AlterTableAction, DatabaseName, TableName},
    error::ApllodbResult,
};
use std::{fmt::Debug, marker::PhantomData};

#[derive(Eq, PartialEq, Hash, Debug, new)]
pub struct AlterTableUseCaseInput<'a, 'tx, 'db: 'tx, Tx: ImmutableSchemaTx<'tx, 'db>> {
    tx: &'tx Tx,

    database_name: &'a DatabaseName,
    table_name: &'a TableName,
    action: &'a AlterTableAction,

    #[new(default)]
    _marker: PhantomData<&'db ()>,
}
impl<'a, 'tx, 'db: 'tx, Tx: ImmutableSchemaTx<'tx, 'db>> UseCaseInput
    for AlterTableUseCaseInput<'a, 'tx, 'db, Tx>
{
    fn validate(&self) -> ApllodbResult<()> {
        Ok(())
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct AlterTableUseCaseOutput;
impl UseCaseOutput for AlterTableUseCaseOutput {}

pub struct AlterTableUseCase<'a, 'tx, 'db: 'tx, Tx: ImmutableSchemaTx<'tx, 'db>> {
    _marker: PhantomData<&'a &'tx &'db Tx>,
}
impl<'a, 'tx, 'db: 'tx, Tx: ImmutableSchemaTx<'tx, 'db>> UseCase
    for AlterTableUseCase<'a, 'tx, 'db, Tx>
{
    type In = AlterTableUseCaseInput<'a, 'tx, 'db, Tx>;
    type Out = AlterTableUseCaseOutput;

    fn run_core(input: Self::In) -> ApllodbResult<Self::Out> {
        let vtable_repo = input.tx.vtable_repo();
        let version_repo = input.tx.version_repo();

        let vtable_id = VTableId::new(input.database_name, input.table_name);
        let mut vtable = vtable_repo.read(&vtable_id)?;
        vtable.alter(input.action)?;

        let active_versions = version_repo.active_versions(&vtable_id)?;
        let current_version = active_versions.current_version()?;
        let next_version = current_version.create_next(input.action)?;

        vtable_repo.update(&vtable)?;
        version_repo.create(&next_version)?;

        Ok(AlterTableUseCaseOutput)
    }
}
