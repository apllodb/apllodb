use crate::use_case::{UseCase, UseCaseInput, UseCaseOutput};

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
pub struct UpdateAllUseCaseInput<
    'usecase,
    'db: 'usecase,
    Engine: StorageEngine<'usecase, 'db>,
    Types: ImmutableSchemaAbstractTypes<'usecase, 'db, Engine>,
> {
    tx: &'usecase Engine::Tx,
    database_name: &'usecase DatabaseName,
    table_name: &'usecase TableName,
    column_values: HashMap<ColumnName, Expression>,

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
    /// - [FeatureNotSupported](apllodb_shared_components::ApllodbErrorKind::FeatureNotSupported) when:
    ///   - any column_values' Expression is not a ConstantVariant.
    fn run_core(mut input: Self::In) -> ApllodbResult<Self::Out> {
        let vtable_repo = Types::VTableRepo::new(&input.tx);

        let vtable_id = VTableId::new(input.database_name, input.table_name);
        let vtable = vtable_repo.read(&vtable_id)?;

        // Fetch all columns of the latest version rows and update requested columns later.
        // FIXME Consider CoW to reduce disk usage (append only updated column to a new version).
        let projection_result: ProjectionResult<'_, 'db, Engine, Types> =
            ProjectionResult::new(input.tx, &vtable, ProjectionQuery::All)?;
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
        let delete_all_usecase_input: DeleteAllUseCaseInput<'_, 'db, Engine, Types> =
            DeleteAllUseCaseInput::new(input.tx, input.database_name, input.table_name);
        let _ = DeleteAllUseCase::run(delete_all_usecase_input)?;

        // INSERT
        for col_vals in new_col_vals_to_insert {
            let insert_usecase_input: InsertUseCaseInput<'_, 'db, Engine, Types> =
                InsertUseCaseInput::new(input.tx, input.database_name, input.table_name, col_vals);
            let _ = InsertUseCase::run(insert_usecase_input)?;
        }

        Ok(UpdateAllUseCaseOutput)
    }
}
