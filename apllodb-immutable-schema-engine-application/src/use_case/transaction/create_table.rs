use crate::use_case::{UseCase, UseCaseInput, UseCaseOutput};
use apllodb_immutable_schema_engine_domain::{ActiveVersion, ImmutableSchemaTx, VTable};
use apllodb_shared_components::{
    data_structure::{ColumnDefinition, DatabaseName, TableConstraints, TableName},
    error::ApllodbResult,
};
use std::{fmt::Debug, marker::PhantomData};

#[derive(Eq, PartialEq, Hash, Debug, new)]
pub struct CreateTableUseCaseInput<'a, 'db: 'a, Tx: ImmutableSchemaTx<'db>> {
    pub tx: &'a mut Tx,
    pub database_name: &'a DatabaseName,
    pub table_name: &'a TableName,
    pub table_constraints: &'a TableConstraints,
    pub column_definitions: &'a [ColumnDefinition],

    #[new(default)]
    _marker: PhantomData<&'db ()>,
}
impl<'a, 'db: 'a, Tx: ImmutableSchemaTx<'db>> UseCaseInput
    for CreateTableUseCaseInput<'a, 'db, Tx>
{
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct CreateTableUseCaseOutput;
impl UseCaseOutput for CreateTableUseCaseOutput {}

pub struct CreateTableUseCase<'a, 'db: 'a, Tx: ImmutableSchemaTx<'db>> {
    _marker: PhantomData<&'a &'db Tx>,
}
impl<'a, 'db: 'a, Tx: ImmutableSchemaTx<'db>> UseCase for CreateTableUseCase<'a, 'db, Tx> {
    type In = CreateTableUseCaseInput<'a, 'db, Tx>;
    type Out = CreateTableUseCaseOutput;

    fn run_core(input: Self::In) -> ApllodbResult<Self::Out> {
        use apllodb_immutable_schema_engine_domain::Entity;

        let vtable = VTable::new(
            input.database_name,
            input.table_name,
            input.table_constraints,
            input.column_definitions,
        )?;

        let v1 = ActiveVersion::initial(
            vtable.id(),
            input.column_definitions,
            input.table_constraints,
        )?;

        input.tx.create_vtable(&vtable)?;
        input.tx.create_version(&v1)?;

        Ok(CreateTableUseCaseOutput)
    }
}
