use crate::use_case::{TxUseCase, UseCaseInput, UseCaseOutput};

// use super::{
//     delete_all::{DeleteAllUseCase, DeleteAllUseCaseInput},
//     insert::{InsertUseCase, InsertUseCaseInput},
// };
use apllodb_immutable_schema_engine_domain::{
    abstract_types::ImmutableSchemaAbstractTypes,
    query::projection::ProjectionResult,
    row_iter::ImmutableSchemaRowIterator,
    vtable::{id::VTableId, repository::VTableRepository},
};
use apllodb_shared_components::{
    ApllodbError, ApllodbErrorKind, ApllodbResult, ColumnName, DatabaseName, Expression, SqlValue,
    SqlValues, TableName,
};
use apllodb_storage_engine_interface::ProjectionQuery;
// use apllodb_storage_engine_interface::ProjectionQuery;
use async_trait::async_trait;
use std::{collections::HashMap, fmt::Debug, marker::PhantomData};

use super::{
    delete_all::{DeleteAllUseCase, DeleteAllUseCaseInput},
    insert::{InsertUseCase, InsertUseCaseInput},
};

#[derive(PartialEq, Debug, new)]
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
                Expression::UnaryOperatorVariant(_, _) => {}
                Expression::UnresolvedFieldReferenceVariant(_) | Expression::BooleanExpressionVariant(_) => {
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

pub struct UpdateAllUseCase<'usecase, Types: ImmutableSchemaAbstractTypes> {
    _marker: PhantomData<(&'usecase (), Types)>,
}

#[async_trait(?Send)]
impl<'usecase, Types: ImmutableSchemaAbstractTypes> TxUseCase<Types>
    for UpdateAllUseCase<'usecase, Types>
{
    type In = UpdateAllUseCaseInput<'usecase>;
    type Out = UpdateAllUseCaseOutput;

    /// # Failures
    ///
    /// - [FeatureNotSupported](apllodb_shared_components::ApllodbErrorKind::FeatureNotSupported) when:
    ///   - any column_values' Expression is not a ConstantVariant.
    async fn run_core(
        vtable_repo: &Types::VTableRepo,
        version_repo: &Types::VersionRepo,
        mut input: Self::In,
    ) -> ApllodbResult<Self::Out> {
        let vtable_id = VTableId::new(input.database_name, input.table_name);
        let vtable = vtable_repo.read(&vtable_id).await?;

        let active_versions = vtable_repo.active_versions(&vtable).await?;

        // Fetch all columns of from all versions and update requested columns later.
        // FIXME Consider CoW to reduce disk usage (append only updated column to a new version).
        let projection_result: ProjectionResult =
            ProjectionResult::new(&vtable, active_versions, ProjectionQuery::All)?;
        let row_iter = vtable_repo.full_scan(&vtable, projection_result).await?;

        let new_columns_to_insert: Vec<ColumnName> = row_iter.schema().as_column_names().to_vec();
        let mut new_values_to_insert: Vec<SqlValues> = vec![];

        for row in row_iter {
            let col_vals_before = row.into_zipped();
            let mut vals_after: Vec<SqlValue> = Vec::new();

            for (column_name, val_before) in col_vals_before {
                let val_after = if let Some(expr) = input.column_values.remove(&column_name) {
                    if let Expression::ConstantVariant(sql_value) = expr {
                        sql_value
                    } else {
                        todo!("only ConstantVariant is acceptable for now")
                    }
                } else {
                    val_before
                };
                vals_after.push(val_after);
            }

            new_values_to_insert.push(SqlValues::new(vals_after));
        }

        // DELETE all
        let delete_all_usecase_input =
            DeleteAllUseCaseInput::new(input.database_name, input.table_name);
        let _ =
            DeleteAllUseCase::<'_, Types>::run(vtable_repo, version_repo, delete_all_usecase_input)
                .await?;

        // INSERT all
        let insert_usecase_input = InsertUseCaseInput::new(
            input.database_name,
            input.table_name,
            &new_columns_to_insert,
            new_values_to_insert,
        );
        let _ = InsertUseCase::<'_, Types>::run(vtable_repo, version_repo, insert_usecase_input)
            .await?;

        Ok(UpdateAllUseCaseOutput)
    }
}
