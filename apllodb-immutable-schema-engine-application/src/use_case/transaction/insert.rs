use crate::use_case::{TxUseCase, UseCaseInput, UseCaseOutput};
use apllodb_immutable_schema_engine_domain::{
    abstract_types::ImmutableSchemaAbstractTypes,
    row::pk::apparent_pk::ApparentPrimaryKey,
    version::id::VersionId,
    version::repository::VersionRepository,
    vtable::{id::VTableId, repository::VTableRepository},
};
use apllodb_shared_components::{
    ApllodbResult, ColumnName, DatabaseName, SqlValue, SqlValues, TableName,
};
use async_trait::async_trait;
use std::{collections::HashMap, fmt::Debug, marker::PhantomData};

#[derive(PartialEq, Debug, new)]
pub struct InsertUseCaseInput<'usecase> {
    database_name: &'usecase DatabaseName,
    table_name: &'usecase TableName,
    columns: &'usecase [ColumnName],
    values: Vec<SqlValues>,
}
impl<'usecase> UseCaseInput for InsertUseCaseInput<'usecase> {
    fn validate(&self) -> ApllodbResult<()> {
        Ok(())
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct InsertUseCaseOutput;
impl UseCaseOutput for InsertUseCaseOutput {}

pub struct InsertUseCase<'usecase, Types: ImmutableSchemaAbstractTypes> {
    _marker: PhantomData<(&'usecase (), Types)>,
}

#[async_trait(?Send)]
impl<'usecase, Types: ImmutableSchemaAbstractTypes> TxUseCase<Types>
    for InsertUseCase<'usecase, Types>
{
    type In = InsertUseCaseInput<'usecase>;
    type Out = InsertUseCaseOutput;

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

        for sql_values in input.values {
            // Construct ApparentPrimaryKey
            let apk = ApparentPrimaryKey::from_table_pk_def(&vtable, &input.columns, &sql_values)?;

            let non_pk_col_vals: HashMap<ColumnName, SqlValue> = input
                .columns
                .into_iter()
                .cloned()
                .zip(sql_values)
                .filter_map(|(column_name, sql_value)| {
                    if apk.column_names().iter().any(|pk_cn| pk_cn == &column_name) {
                        None
                    } else {
                        Some((column_name.clone(), sql_value))
                    }
                })
                .collect();

            // Determine version to insert
            let active_versions = vtable_repo.active_versions(&vtable).await?;
            let version_to_insert = active_versions.version_to_insert(&non_pk_col_vals)?;
            let version_id = VersionId::new(&vtable_id, version_to_insert.number());

            version_repo
                .insert(&version_id, apk, &non_pk_col_vals)
                .await?;
        }

        Ok(InsertUseCaseOutput)
    }
}
