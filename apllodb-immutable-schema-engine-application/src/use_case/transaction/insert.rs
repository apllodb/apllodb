use crate::use_case::{UseCase, UseCaseInput, UseCaseOutput};
use apllodb_immutable_schema_engine_domain::{
    abstract_types::ImmutableSchemaAbstractTypes,
    row::{
        column::non_pk_column::column_name::NonPKColumnName, pk::apparent_pk::ApparentPrimaryKey,
    },
    transaction::ImmutableSchemaTx,
    version::{id::VersionId, repository::VersionRepository},
    vtable::{id::VTableId, repository::VTableRepository},
};
use apllodb_shared_components::{
    data_structure::{ColumnName, DatabaseName, Expression, TableName},
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use std::{collections::HashMap, fmt::Debug, marker::PhantomData};

#[derive(Eq, PartialEq, Debug, new)]
pub struct InsertUseCaseInput<'a, 'tx, 'db: 'tx, Engine: StorageEngine<'tx, 'db>, Types: ImmutableSchemaAbstractTypes<'tx, 'db>> {
    tx: &'tx Engine::Tx,

    database_name: &'a DatabaseName,
    table_name: &'a TableName,
    column_values: &'a HashMap<ColumnName, Expression>,

    #[new(default)]
    _marker: PhantomData<&'db ()>,
}
impl<'a, 'tx, 'db: 'tx, Engine: StorageEngine<'tx, 'db>, Types: ImmutableSchemaAbstractTypes<'tx, 'db>> UseCaseInput
    for InsertUseCaseInput<'a, 'tx, 'db, Types>
{
    fn validate(&self) -> ApllodbResult<()> {
        self.validate_expression_type()?;
        Ok(())
    }
}
impl<'a, 'tx, 'db: 'tx, Engine: StorageEngine<'tx, 'db>, Types: ImmutableSchemaAbstractTypes<'tx, 'db>> InsertUseCaseInput<'a, 'tx, 'db, Types> {
    fn validate_expression_type(&self) -> ApllodbResult<()> {
        for (column_name, expr) in self.column_values {
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

pub struct InsertUseCase<'a, 'tx, 'db: 'tx, Engine: StorageEngine<'tx, 'db>, Types: ImmutableSchemaAbstractTypes<'tx, 'db>> {
    _marker: PhantomData<&'a &'tx &'db Types>,
}
impl<'a, 'tx, 'db: 'tx, Engine: StorageEngine<'tx, 'db>, Types: ImmutableSchemaAbstractTypes<'tx, 'db>> UseCase
    for InsertUseCase<'a, 'tx, 'db, Types>
{
    type In = InsertUseCaseInput<'a, 'tx, 'db, Types>;
    type Out = InsertUseCaseOutput;

    /// # Failures
    ///
    /// - [FeatureNotSupported](error/enum.ApllodbErrorKind.html#variant.FeatureNotSupported) when:
    ///   - any column_values' Expression is not a ConstantVariant.
    fn run_core(input: Self::In) -> ApllodbResult<Self::Out> {
        let vtable_repo = input.tx.vtable_repo();
        let version_repo = input.tx.version_repo();

        let vtable_id = VTableId::new(input.database_name, input.table_name);
        let vtable = vtable_repo.read(&vtable_id)?;

        // Construct ApparentPrimaryKey
        let apk = ApparentPrimaryKey::from_table_and_column_values(&vtable, input.column_values)?;

        // Filter Non-PK columns from column_values
        let non_pk_column_values: HashMap<NonPKColumnName, Expression> = input
            .column_values
            .clone()
            .into_iter()
            .filter_map(|(column_name, expr)| {
                if apk
                    .column_names()
                    .iter()
                    .any(|pk_cn| pk_cn.as_str() == column_name.as_str())
                {
                    None
                } else {
                    Some((NonPKColumnName::from(column_name), expr))
                }
            })
            .collect();

        // Determine version to insert
        let active_versions = vtable_repo.active_versions(&vtable)?;
        let version_to_insert = active_versions.version_to_insert(&non_pk_column_values)?;
        let version_id = VersionId::new(&vtable_id, version_to_insert.number());

        // rowで会話するようにしたい。そうすれば、updateでもrowを作ってからversion_repoに同じ用は話しかけられる
        version_repo.insert(&version_id, apk, &non_pk_column_values)?;

        Ok(InsertUseCaseOutput)
    }
}
