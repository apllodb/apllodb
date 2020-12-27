use crate::use_case::{TxUseCase, UseCaseInput, UseCaseOutput};
use apllodb_immutable_schema_engine_domain::abstract_types::ImmutableSchemaAbstractTypes;
use apllodb_immutable_schema_engine_domain::{
    query::projection::ProjectionResult,
    vtable::{id::VTableId, repository::VTableRepository},
};
use apllodb_shared_components::{ApllodbResult, DatabaseName, TableName};
use apllodb_storage_engine_interface::{ProjectionQuery, StorageEngine};

use std::{fmt::Debug, marker::PhantomData};

#[derive(Eq, PartialEq, Debug, new)]
pub struct FullScanUseCaseInput<'usecase> {
    database_name: &'usecase DatabaseName,
    table_name: &'usecase TableName,
    projection: ProjectionQuery,
}
impl<'usecase> UseCaseInput for FullScanUseCaseInput<'usecase> {
    fn validate(&self) -> ApllodbResult<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct FullScanUseCaseOutput<
    'usecase,
    'db: 'usecase,
    Engine: StorageEngine,
    Types: ImmutableSchemaAbstractTypes<'usecase, 'db, Engine>,
> {
    pub row_iter: Types::ImmutableSchemaRowIter,
}
impl<
        'usecase,
        'db: 'usecase,
        Engine: StorageEngine,
        Types: ImmutableSchemaAbstractTypes<'usecase, 'db, Engine>,
    > UseCaseOutput for FullScanUseCaseOutput<'usecase, 'db, Engine, Types>
{
}

pub struct FullScanUseCase<
    'usecase,
    'db: 'usecase,
    Engine: StorageEngine,
    Types: ImmutableSchemaAbstractTypes<'usecase, 'db, Engine>,
> {
    _marker: PhantomData<(&'usecase &'db (), Engine, Types)>,
}
impl<
        'usecase,
        'db: 'usecase,
        Engine: StorageEngine,
        Types: ImmutableSchemaAbstractTypes<'usecase, 'db, Engine>,
    > TxUseCase<'usecase, 'db, Engine, Types> for FullScanUseCase<'usecase, 'db, Engine, Types>
{
    type In = FullScanUseCaseInput<'usecase>;
    type Out = FullScanUseCaseOutput<'usecase, 'db, Engine, Types>;

    /// # Failures
    ///
    /// - [FeatureNotSupported](apllodb_shared_components::ApllodbErrorKind::FeatureNotSupported) when:
    ///   - any column_values' Expression is not a ConstantVariant.
    fn run_core(
        vtable_repo: &Types::VTableRepo,
        _version_repo: &Types::VersionRepo,
        input: Self::In,
    ) -> ApllodbResult<Self::Out> {
        let vtable_id = VTableId::new(input.database_name, input.table_name);
        let vtable = vtable_repo.read(&vtable_id)?;

        let active_versions = vtable_repo.active_versions(&vtable)?;

        let projection_result: ProjectionResult<'_, 'db, Engine, Types> =
            ProjectionResult::new(&vtable, active_versions, input.projection)?;
        let row_iter = vtable_repo.full_scan(&vtable, projection_result)?;
        Ok(FullScanUseCaseOutput { row_iter })
    }
}
