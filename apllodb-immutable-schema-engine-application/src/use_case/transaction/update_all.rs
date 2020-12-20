use crate::use_case::{UseCase, UseCaseInput, UseCaseOutput};

use apllodb_immutable_schema_engine_domain::{
    abstract_types::ImmutableSchemaAbstractTypes,
    query::projection::ProjectionResult,
    version::{repository::VersionRepository},
    vtable::{id::VTableId, repository::VTableRepository},
};
use apllodb_shared_components::{
    data_structure::{ColumnName, DatabaseName, Expression, TableName},
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use apllodb_storage_engine_interface::{ProjectionQuery, StorageEngine};
use std::{collections::HashMap, fmt::Debug, marker::PhantomData};

#[derive(Eq, PartialEq, Debug, new)]
pub struct UpdateAllUseCaseInput<
    'usecase,
    'db: 'usecase,
    Engine: StorageEngine<'usecase, 'db>,
    Types: ImmutableSchemaAbstractTypes<'usecase, 'db, Engine>,
> {
    tx: &'usecase Engine::Tx,
    database_name: &'usecase DatabaseName,
    table_name: &'usecase TableName,
    column_values: &'usecase HashMap<ColumnName, Expression>,

    #[new(default)]
    _marker: PhantomData<(&'db (), Types)>,
}
impl<
        'usecase,
        'db: 'usecase,
        Engine: StorageEngine<'usecase, 'db>,
        Types: ImmutableSchemaAbstractTypes<'usecase, 'db, Engine>,
    > UseCaseInput for UpdateAllUseCaseInput<'usecase, 'db, Engine, Types>
{
    fn validate(&self) -> ApllodbResult<()> {
        self.validate_expression_type()?;
        Ok(())
    }
}
impl<
        'usecase,
        'db: 'usecase,
        Engine: StorageEngine<'usecase, 'db>,
        Types: ImmutableSchemaAbstractTypes<'usecase, 'db, Engine>,
    > UpdateAllUseCaseInput<'usecase, 'db, Engine, Types>
{
    fn validate_expression_type(&self) -> ApllodbResult<()> {
        for (column_name, expr) in self.column_values {
            match expr {
                Expression::ConstantVariant(_) => {}
                Expression::ColumnNameVariant(_) | Expression::BooleanExpressionVariant(_) => {
                    return Err(ApllodbError::new(ApllodbErrorKind::FeatureNotSupported,
                        format!("trying to UpdateAll `{:?}={:?}` while expr of `UpdateAll INTO ... VALUES (expr ...)`. `expr` can only be a constant", 
                        column_name, expr
                    ),
                    None
                    ))
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct UpdateAllUseCaseOutput;
impl UseCaseOutput for UpdateAllUseCaseOutput {}

pub struct UpdateAllUseCase<
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
    > UseCase for UpdateAllUseCase<'usecase, 'db, Engine, Types>
{
    type In = UpdateAllUseCaseInput<'usecase, 'db, Engine, Types>;
    type Out = UpdateAllUseCaseOutput;

    /// # Failures
    ///
    /// - [FeatureNotSupported](error/enum.ApllodbErrorKind.html#variant.FeatureNotSupported) when:
    ///   - any column_values' Expression is not a ConstantVariant.
    fn run_core(input: Self::In) -> ApllodbResult<Self::Out> {
        let vtable_repo = Types::VTableRepo::new(&input.tx);
        let version_repo = Types::VersionRepo::new(&input.tx);

        let vtable_id = VTableId::new(input.database_name, input.table_name);
        let vtable = vtable_repo.read(&vtable_id)?;

        let active_versions = vtable_repo.active_versions(&vtable)?;

        // Fetch all columns of the latest version rows and update requested columns later.
        // FIXME Consider CoW to reduce disk usage (append only updated column to a new version).
        let projection_result: ProjectionResult<'_, 'db, Engine, Types> =
            ProjectionResult::new(input.tx, &vtable, ProjectionQuery::All)?;
        let row_iter = vtable_repo.full_scan(&vtable, projection_result)?;

        for row in row_iter {
            // TODO PK: ApparentPrimaryKey, non-PK: HashMap<ColumnName, Expression> に各 row を分ける

            // let version_to_update = active_versions.version_to_insert(&non_pk_column_values)?;
            // let version_id = VersionId::new(&vtable_id, version_to_insert.number());
            // version_repo.insert(&version_id, apk, &non_pk_column_values)?;
        }

        Ok(UpdateAllUseCaseOutput)
    }
}
