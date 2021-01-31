use crate::use_case::{TxUseCase, UseCaseInput, UseCaseOutput};

use super::{
    delete_all::{DeleteAllUseCase, DeleteAllUseCaseInput},
    insert::{InsertUseCase, InsertUseCaseInput},
};
use apllodb_immutable_schema_engine_domain::{
    abstract_types::ImmutableSchemaAbstractTypes,
    query::projection::ProjectionResult,
    vtable::{id::VTableId, repository::VTableRepository},
};
use apllodb_shared_components::{
    ApllodbError, ApllodbErrorKind, ApllodbResult, ColumnName, ColumnReference, DatabaseName,
    Expression, FieldIndex, Record, RecordIterator, SqlValue, TableName,
};
use apllodb_storage_engine_interface::ProjectionQuery;
use async_trait::async_trait;
use std::{collections::HashMap, fmt::Debug, marker::PhantomData};

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

        // Fetch all columns of the latest version rows and update requested columns later.
        // FIXME Consider CoW to reduce disk usage (append only updated column to a new version).
        let projection_result: ProjectionResult =
            ProjectionResult::new(&vtable, active_versions, ProjectionQuery::All)?;
        let row_iter = vtable_repo.full_scan(&vtable, projection_result).await?;

        let mut new_col_vals_to_insert: Vec<HashMap<ColumnName, SqlValue>> = Vec::new();
        for row in row_iter {
            let col_vals_before = row.into_col_vals();
            let mut col_vals_after: HashMap<ColumnName, SqlValue> = HashMap::new();

            for (colref, val_before) in col_vals_before {
                let val_after =
                    if let Some(expr) = input.column_values.remove(colref.as_column_name()) {
                        if let Expression::ConstantVariant(sql_value) = expr {
                            sql_value
                        } else {
                            todo!("only ConstantVariant is acceptable for now")
                        }
                    } else {
                        val_before
                    };
                col_vals_after.insert(colref.as_column_name().clone(), val_after);
            }

            new_col_vals_to_insert.push(col_vals_after);
        }

        // DELETE all
        let delete_all_usecase_input =
            DeleteAllUseCaseInput::new(input.database_name, input.table_name);
        let _ =
            DeleteAllUseCase::<'_, Types>::run(vtable_repo, version_repo, delete_all_usecase_input)
                .await?;

        // INSERT all
        let records: Vec<Record> = new_col_vals_to_insert
            .into_iter()
            .map(|mut fields| {
                Record::new(
                    fields
                        .drain()
                        .map(|(cn, sql_value)| {
                            let colref = ColumnReference::new(vtable.table_name().clone(), cn);
                            let field = FieldIndex::InColumnReference(colref);
                            (field, sql_value)
                        })
                        .collect(),
                )
            })
            .collect();
        let records = RecordIterator::new(records);

        let insert_usecase_input =
            InsertUseCaseInput::new(input.database_name, input.table_name, records);
        let _ = InsertUseCase::<'_, Types>::run(vtable_repo, version_repo, insert_usecase_input)
            .await?;

        Ok(UpdateAllUseCaseOutput)
    }
}
