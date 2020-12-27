use crate::use_case::{TxUseCase, UseCaseInput, UseCaseOutput};

use apllodb_immutable_schema_engine_domain::{
    abstract_types::ImmutableSchemaAbstractTypes,
    query::projection::ProjectionResult,
    vtable::{id::VTableId, repository::VTableRepository},
};
use apllodb_shared_components::{
    ApllodbError, ApllodbErrorKind, ApllodbResult, ColumnName, DatabaseName, Expression, TableName,
};
use apllodb_storage_engine_interface::{ProjectionQuery, StorageEngine};
use std::{collections::HashMap, fmt::Debug, marker::PhantomData};

use super::{
    delete_all::{DeleteAllUseCase, DeleteAllUseCaseInput},
    insert::{InsertUseCase, InsertUseCaseInput},
};

#[derive(Eq, PartialEq, Debug, new)]
pub struct UpdateAllUseCaseInput<'usecase> {
    database_name: &'usecase DatabaseName,
    table_name: &'usecase TableName,
    column_values: HashMap<ColumnName, Expression>,
}
impl<'usecase> UseCaseInput for UpdateAllUseCaseInput<'usecase> {
    fn validate(&self) -> ApllodbResult<()> {
        self.validate_expression_type()?;
        Ok(())
    }
}
impl<'usecase> UpdateAllUseCaseInput<'usecase> {
    fn validate_expression_type(&self) -> ApllodbResult<()> {
        for (column_name, expr) in &self.column_values {
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
    > TxUseCase<'usecase, 'db, Engine, Types> for UpdateAllUseCase<'usecase, 'db, Engine, Types>
{
    type In = UpdateAllUseCaseInput<'usecase>;
    type Out = UpdateAllUseCaseOutput;

    /// # Failures
    ///
    /// - [FeatureNotSupported](apllodb_shared_components::ApllodbErrorKind::FeatureNotSupported) when:
    ///   - any column_values' Expression is not a ConstantVariant.
    fn run_core(
        vtable_repo: &Types::VTableRepo,
        version_repo: &Types::VersionRepo,
        mut input: Self::In,
    ) -> ApllodbResult<Self::Out> {
        let vtable_id = VTableId::new(input.database_name, input.table_name);
        let vtable = vtable_repo.read(&vtable_id)?;

        let active_versions = vtable_repo.active_versions(&vtable)?;

        // Fetch all columns of the latest version rows and update requested columns later.
        // FIXME Consider CoW to reduce disk usage (append only updated column to a new version).
        let projection_result: ProjectionResult<'_, 'db, Engine, Types> =
            ProjectionResult::new(&vtable, active_versions, ProjectionQuery::All)?;
        let row_iter = vtable_repo.full_scan(&vtable, projection_result)?;

        let mut new_col_vals_to_insert: Vec<HashMap<ColumnName, Expression>> = Vec::new();
        for row in row_iter {
            let col_vals_before = row.into_col_vals();
            let mut col_vals_after: HashMap<ColumnName, Expression> = HashMap::new();

            for (colref, val_before) in col_vals_before {
                let expr_after =
                    if let Some(expr_after) = input.column_values.remove(colref.as_column_name()) {
                        expr_after
                    } else {
                        Expression::from(&val_before)
                    };
                col_vals_after.insert(colref.as_column_name().clone(), expr_after);
            }

            new_col_vals_to_insert.push(col_vals_after);
        }

        // DELETE all
        let delete_all_usecase_input =
            DeleteAllUseCaseInput::new(input.database_name, input.table_name);
        let _ = DeleteAllUseCase::<'_, '_, Engine, Types>::run(
            vtable_repo,
            version_repo,
            delete_all_usecase_input,
        )?;

        // INSERT
        for col_vals in new_col_vals_to_insert {
            let insert_usecase_input =
                InsertUseCaseInput::new(input.database_name, input.table_name, col_vals);
            let _ = InsertUseCase::<'_, '_, Engine, Types>::run(
                vtable_repo,
                version_repo,
                insert_usecase_input,
            )?;
        }

        Ok(UpdateAllUseCaseOutput)
    }
}
