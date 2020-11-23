use crate::use_case::{UseCase, UseCaseInput, UseCaseOutput};
use apllodb_immutable_schema_engine_domain::{abstract_types::ImmutableSchemaAbstractTypes, row::column::filter_non_pk_column_definitions, version::{active_version::ActiveVersion, repository::VersionRepository}, vtable::{repository::VTableRepository, VTable}};
use apllodb_shared_components::{data_structure::{ColumnDefinition, DatabaseName, TableConstraints, TableName}, error::ApllodbResult, data_structure::ColumnDataType};
use apllodb_storage_engine_interface::StorageEngine;

use std::{fmt::Debug, marker::PhantomData};

#[derive(Eq, PartialEq, Hash, Debug, new)]
pub struct CreateTableUseCaseInput<
    'usecase,
    'db: 'usecase,
    Engine: StorageEngine<'usecase, 'db>,
    Types: ImmutableSchemaAbstractTypes<'usecase, 'db, Engine>,
> {
    tx: &'usecase Engine::Tx,
    database_name: &'usecase DatabaseName,
    table_name: &'usecase TableName,
    table_constraints: &'usecase TableConstraints,
    column_definitions: &'usecase [ColumnDefinition],

    #[new(default)]
    _marker: PhantomData<(&'db (), Types)>,
}
impl<
        'usecase,
        'db: 'usecase,
        Engine: StorageEngine<'usecase, 'db>,
        Types: ImmutableSchemaAbstractTypes<'usecase, 'db, Engine>,
    > UseCaseInput for CreateTableUseCaseInput<'usecase, 'db, Engine, Types>
{
    fn validate(&self) -> ApllodbResult<()> {
        Ok(())
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct CreateTableUseCaseOutput;
impl UseCaseOutput for CreateTableUseCaseOutput {}

pub struct CreateTableUseCase<
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
    > UseCase for CreateTableUseCase<'usecase, 'db, Engine, Types>
{
    type In = CreateTableUseCaseInput<'usecase, 'db, Engine, Types>;
    type Out = CreateTableUseCaseOutput;

    fn run_core(input: Self::In) -> ApllodbResult<Self::Out> {
        use apllodb_immutable_schema_engine_domain::entity::Entity;

        let vtable_repo = Types::VTableRepo::new(&input.tx);
        let version_repo = Types::VersionRepo::new(&input.tx);

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

        vtable_repo.create(&vtable)?;
        version_repo.create(&v1)?;

        Ok(CreateTableUseCaseOutput)
    }
}
