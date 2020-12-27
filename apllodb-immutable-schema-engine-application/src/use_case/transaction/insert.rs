use crate::use_case::{TxUseCase, UseCaseInput, UseCaseOutput};
use apllodb_immutable_schema_engine_domain::{
    abstract_types::ImmutableSchemaAbstractTypes,
    row::pk::apparent_pk::ApparentPrimaryKey,
    version::id::VersionId,
    version::repository::VersionRepository,
    vtable::{id::VTableId, repository::VTableRepository},
};
use apllodb_shared_components::{
    ApllodbError, ApllodbErrorKind, ApllodbResult, ColumnName, DatabaseName, Expression, TableName,
};
use apllodb_storage_engine_interface::StorageEngine;
use std::{collections::HashMap, fmt::Debug, marker::PhantomData};

#[derive(Eq, PartialEq, Debug, new)]
pub struct InsertUseCaseInput<'usecase> {
    database_name: &'usecase DatabaseName,
    table_name: &'usecase TableName,
    column_values: HashMap<ColumnName, Expression>,
}
impl<'usecase> UseCaseInput for InsertUseCaseInput<'usecase> {
    fn validate(&self) -> ApllodbResult<()> {
        self.validate_expression_type()?;
        Ok(())
    }
}
impl<'usecase> InsertUseCaseInput<'usecase> {
    fn validate_expression_type(&self) -> ApllodbResult<()> {
        for (column_name, expr) in &self.column_values {
            match expr {
                Expression::ConstantVariant(_) => {}
                Expression::ColumnNameVariant(_) | Expression::BooleanExpressionVariant(_) => {
                    return Err(ApllodbError::new(ApllodbErrorKind::FeatureNotSupported,
                        format!("trying to insert `{:?}={:?}` while expr of `INSERT INTO ... VALUES (expr ...)`. `expr` can only be a constant", 
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
pub struct InsertUseCaseOutput;
impl UseCaseOutput for InsertUseCaseOutput {}

pub struct InsertUseCase<
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
    > TxUseCase<'usecase, 'db, Engine, Types> for InsertUseCase<'usecase, 'db, Engine, Types>
{
    type In = InsertUseCaseInput<'usecase>;
    type Out = InsertUseCaseOutput;

    /// # Failures
    ///
    /// - [FeatureNotSupported](apllodb_shared_components::ApllodbErrorKind::FeatureNotSupported) when:
    ///   - any column_values' Expression is not a ConstantVariant.
    fn run_core(
        vtable_repo: &Types::VTableRepo,
        version_repo: &Types::VersionRepo,
        input: Self::In,
    ) -> ApllodbResult<Self::Out> {
        let vtable_id = VTableId::new(input.database_name, input.table_name);
        let vtable = vtable_repo.read(&vtable_id)?;

        // Construct ApparentPrimaryKey
        let apk = ApparentPrimaryKey::from_table_and_column_values(&vtable, &input.column_values)?;

        // Filter Non-PK columns from column_values
        let non_pk_column_values: HashMap<ColumnName, Expression> = input
            .column_values
            .into_iter()
            .filter_map(|(column_name, expr)| {
                if apk
                    .column_names()
                    .iter()
                    .any(|pk_cn| pk_cn.as_str() == column_name.as_str())
                {
                    None
                } else {
                    Some((column_name, expr))
                }
            })
            .collect();

        // Determine version to insert
        let active_versions = vtable_repo.active_versions(&vtable)?;
        let version_to_insert = active_versions.version_to_insert(&non_pk_column_values)?;
        let version_id = VersionId::new(&vtable_id, version_to_insert.number());

        version_repo.insert(&version_id, apk, &non_pk_column_values)?;

        Ok(InsertUseCaseOutput)
    }
}
