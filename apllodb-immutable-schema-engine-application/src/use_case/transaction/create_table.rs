use crate::use_case::{UseCase, UseCaseInput, UseCaseOutput};
use apllodb_immutable_schema_engine_domain::{
    row::column::non_pk_column::{
        column_data_type::NonPKColumnDataType, filter_non_pk_column_definitions,
    },
    version::{active_version::ActiveVersion, repository::VersionRepository},
    vtable::{repository::VTableRepository, VTable},
};
use apllodb_shared_components::{
    data_structure::{ColumnDefinition, DatabaseName, TableConstraints, TableName},
    error::ApllodbResult,
};
use apllodb_storage_engine_interface::StorageEngine;

use std::{fmt::Debug, marker::PhantomData};

#[derive(Eq, PartialEq, Hash, Debug, new)]
pub struct CreateTableUseCaseInput<'a, 'tx: 'a, 'db: 'tx, Engine: StorageEngine<'db>> {
    tx: &'a Engine::Tx,
    database_name: &'a DatabaseName,
    table_name: &'a TableName,
    table_constraints: &'a TableConstraints,
    column_definitions: &'a [ColumnDefinition],

    #[new(default)]
    _marker: PhantomData<&'tx &'db ()>,
}
impl<'a, 'tx: 'tx, 'db: 'tx, Engine: StorageEngine<'db>> UseCaseInput
    for CreateTableUseCaseInput<'a, 'tx, 'db, Engine>
{
    fn validate(&self) -> ApllodbResult<()> {
        Ok(())
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct CreateTableUseCaseOutput;
impl UseCaseOutput for CreateTableUseCaseOutput {}

pub struct CreateTableUseCase<'a, 'tx: 'a, 'db: 'tx, Engine: StorageEngine<'db>> {
    _marker: PhantomData<(&'a &'tx &'db (), Engine)>,
}
impl<'a, 'tx: 'a, 'db: 'tx, Engine: StorageEngine<'db>> UseCase
    for CreateTableUseCase<'a, 'tx, 'db, Engine>
{
    type In = CreateTableUseCaseInput<'a, 'tx, 'db, Engine>;
    type Out = CreateTableUseCaseOutput;

    fn run_core(input: Self::In) -> ApllodbResult<Self::Out> {
        use apllodb_immutable_schema_engine_domain::entity::Entity;

        let vtable = VTable::create(
            input.database_name,
            input.table_name,
            input.table_constraints,
            input.column_definitions,
        )?;

        let apk_column_names = vtable.table_wide_constraints().pk_column_names();
        let column_data_types: Vec<NonPKColumnDataType> =
            filter_non_pk_column_definitions(input.column_definitions, &apk_column_names)
                .iter()
                .map(|coldef| coldef.into())
                .collect();

        let v1 = ActiveVersion::initial(vtable.id(), &column_data_types)?;

        input.tx.vtable_repo().create(&vtable)?;
        input.tx.version_repo().create(&v1)?;

        Ok(CreateTableUseCaseOutput)
    }
}
