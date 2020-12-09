use crate::use_case::{UseCase, UseCaseInput, UseCaseOutput};
use apllodb_immutable_schema_engine_domain::vtable::{id::VTableId, repository::VTableRepository};
use apllodb_immutable_schema_engine_domain::{
    abstract_types::ImmutableSchemaAbstractTypes, row::column::filter_non_pk_column_names,
};
use apllodb_shared_components::{
    data_structure::{ColumnName, DatabaseName, TableName},
    error::ApllodbResult,
};
use apllodb_storage_engine_interface::StorageEngine;

use std::{fmt::Debug, marker::PhantomData};

#[derive(Eq, PartialEq, Debug, new)]
pub struct FullScanUseCaseInput<
    'usecase,
    'db: 'usecase,
    Engine: StorageEngine<'usecase, 'db>,
    Types: ImmutableSchemaAbstractTypes<'usecase, 'db, Engine>,
> {
    tx: &'usecase Engine::Tx,
    database_name: &'usecase DatabaseName,
    table_name: &'usecase TableName,
    column_names: &'usecase [ColumnName],

    #[new(default)]
    _marker: PhantomData<(&'db (), Types)>,
}
impl<
        'usecase,
        'db: 'usecase,
        Engine: StorageEngine<'usecase, 'db>,
        Types: ImmutableSchemaAbstractTypes<'usecase, 'db, Engine>,
    > UseCaseInput for FullScanUseCaseInput<'usecase, 'db, Engine, Types>
{
    fn validate(&self) -> ApllodbResult<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct FullScanUseCaseOutput<'usecase, 'db: 'usecase, Engine: StorageEngine<'usecase, 'db>> {
    pub row_iter: Engine::RowIter,
}
impl<'usecase, 'db: 'usecase, Engine: StorageEngine<'usecase, 'db>> UseCaseOutput
    for FullScanUseCaseOutput<'usecase, 'db, Engine>
{
}

pub struct FullScanUseCase<
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
    > UseCase for FullScanUseCase<'usecase, 'db, Engine, Types>
{
    type In = FullScanUseCaseInput<'usecase, 'db, Engine, Types>;
    type Out = FullScanUseCaseOutput<'usecase, 'db, Engine>;

    /// # Failures
    ///
    /// - [FeatureNotSupported](error/enum.ApllodbErrorKind.html#variant.FeatureNotSupported) when:
    ///   - any column_values' Expression is not a ConstantVariant.
    fn run_core(input: Self::In) -> ApllodbResult<Self::Out> {
        let vtable_repo = Types::VTableRepo::new(&input.tx);

        let vtable_id = VTableId::new(input.database_name, input.table_name);
        let vtable = vtable_repo.read(&vtable_id)?;

        // TODO ここで PK projection を弾いてはいけない。
        let non_pk_column_names = filter_non_pk_column_names(
            input.column_names,
            &vtable.table_wide_constraints().pk_column_names(),
        );

        let row_iter = vtable_repo.full_scan(&vtable, &non_pk_column_names)?;
        Ok(FullScanUseCaseOutput { row_iter })
    }
}
