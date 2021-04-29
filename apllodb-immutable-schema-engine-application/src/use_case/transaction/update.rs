use crate::use_case::{TxUseCase, UseCaseInput, UseCaseOutput};

// use super::{
//     delete_all::{DeleteAllUseCase, DeleteAllUseCaseInput},
//     insert::{InsertUseCase, InsertUseCaseInput},
// };
use apllodb_immutable_schema_engine_domain::{
    abstract_types::ImmutableSchemaAbstractTypes,
    row_projection_result::RowProjectionResult,
    row_selection_plan::RowSelectionPlan,
    vtable::{id::VTableId, repository::VTableRepository, VTable},
};
use apllodb_shared_components::{
    ApllodbError, ApllodbErrorKind, ApllodbResult, DatabaseName, Expression, SqlValue,
};
use apllodb_storage_engine_interface::{
    ColumnName, Row, RowProjectionQuery, RowSchema, Rows, TableName,
};
// use apllodb_storage_engine_interface::ProjectionQuery;
use async_trait::async_trait;
use std::{collections::HashMap, fmt::Debug, marker::PhantomData};

use super::{
    delete::{DeleteUseCase, DeleteUseCaseInput},
    insert::{InsertUseCase, InsertUseCaseInput},
};

#[derive(PartialEq, Debug, new)]
pub struct UpdateUseCaseInput<'usecase, Types: ImmutableSchemaAbstractTypes> {
    database_name: &'usecase DatabaseName,
    table_name: &'usecase TableName,
    column_values: HashMap<ColumnName, Expression>,
    selection: &'usecase RowSelectionPlan<Types>,
}
impl<'usecase, Types: ImmutableSchemaAbstractTypes> UseCaseInput
    for UpdateUseCaseInput<'usecase, Types>
{
    fn validate(&self) -> ApllodbResult<()> {
        self.validate_expression_type()?;
        Ok(())
    }
}
impl<'usecase, Types: ImmutableSchemaAbstractTypes> UpdateUseCaseInput<'usecase, Types> {
    fn validate_expression_type(&self) -> ApllodbResult<()> {
        for (column_name, expr) in &self.column_values {
            match expr {
                Expression::ConstantVariant(_) => {}
                Expression::UnaryOperatorVariant(_, _) => {}
                Expression::SchemaIndexVariant(_) | Expression::BooleanExpressionVariant(_) => {
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
pub struct UpdateUseCaseOutput;
impl UseCaseOutput for UpdateUseCaseOutput {}

pub struct UpdateUseCase<'usecase, Types: ImmutableSchemaAbstractTypes> {
    _marker: PhantomData<(&'usecase (), Types)>,
}

#[async_trait(?Send)]
impl<'usecase, Types: ImmutableSchemaAbstractTypes + 'usecase> TxUseCase<Types>
    for UpdateUseCase<'usecase, Types>
{
    type In = UpdateUseCaseInput<'usecase, Types>;
    type Out = UpdateUseCaseOutput;

    /// # Failures
    ///
    /// - [FeatureNotSupported](apllodb_shared_components::ApllodbErrorKind::FeatureNotSupported) when:
    ///   - any column_values' Expression is not a ConstantVariant.
    async fn run_core(
        vtable_repo: &Types::VTableRepo,
        version_repo: &Types::VersionRepo,
        input: Self::In,
    ) -> ApllodbResult<Self::Out> {
        let vtable_id = VTableId::new(input.database_name, input.table_name);
        let vtable = vtable_repo.read(&vtable_id).await?;

        let projection_result = Self::projection_result(vtable_repo, &vtable).await?;

        let rows = vtable_repo
            .select(&vtable, projection_result, input.selection)
            .await?;

        vtable_repo.delete(&vtable, input.selection).await?;

        Self::insert_updated_rows(
            vtable_repo,
            version_repo,
            &input.database_name,
            &input.table_name,
            rows,
            input.column_values,
        )
        .await?;

        Ok(UpdateUseCaseOutput)
    }
}

impl<'usecase, Types: ImmutableSchemaAbstractTypes> UpdateUseCase<'usecase, Types> {
    async fn insert_updated_rows(
        vtable_repo: &Types::VTableRepo,
        version_repo: &Types::VersionRepo,
        database_name: &DatabaseName,
        table_name: &TableName,
        rows_before: Rows,
        column_values_to_set: HashMap<ColumnName, Expression>,
    ) -> ApllodbResult<()> {
        let schema = rows_before.as_schema();
        let new_columns_to_insert = Self::new_columns_to_insert(schema);
        let new_rows_to_insert = Self::new_rows_to_insert(rows_before, column_values_to_set)?;

        let insert_usecase_input = InsertUseCaseInput::new(
            database_name,
            table_name,
            &new_columns_to_insert,
            new_rows_to_insert,
        );
        let _ = InsertUseCase::<'_, Types>::run(vtable_repo, version_repo, insert_usecase_input)
            .await?;

        Ok(())
    }

    async fn projection_result(
        vtable_repo: &Types::VTableRepo,
        vtable: &VTable,
    ) -> ApllodbResult<RowProjectionResult> {
        let active_versions = vtable_repo.active_versions(&vtable).await?;

        // Fetch all columns from all versions and update requested columns later.
        // FIXME Consider CoW to reduce disk usage (append only updated column to a new version).
        let projection_result =
            RowProjectionResult::new(&vtable, active_versions, &RowProjectionQuery::All)?;

        Ok(projection_result)
    }

    fn new_columns_to_insert(schema: &RowSchema) -> Vec<ColumnName> {
        schema
            .table_column_names()
            .into_iter()
            .map(|tc| tc.as_column_name().clone())
            .collect()
    }

    fn new_rows_to_insert(
        rows_before: Rows,
        mut column_values_to_set: HashMap<ColumnName, Expression>,
    ) -> ApllodbResult<Vec<Row>> {
        let mut ret: Vec<Row> = vec![];

        let schema = rows_before.as_schema().clone();

        for row in rows_before {
            let mut vals_after: Vec<SqlValue> = Vec::new();

            for (pos, tc) in schema.table_column_names_with_pos() {
                let column_name = tc.as_column_name();

                let val_after = if let Some(expr) = column_values_to_set.remove(&column_name) {
                    if let Expression::ConstantVariant(sql_value) = expr {
                        Ok(sql_value)
                    } else {
                        Err(ApllodbError::feature_not_supported(
                            "only ConstantVariant is acceptable for now",
                        ))
                    }
                } else {
                    let val_before = row.get_sql_value(pos)?;
                    Ok(val_before.clone())
                }?;
                vals_after.push(val_after);
            }

            ret.push(Row::new(vals_after));
        }

        Ok(ret)
    }
}
