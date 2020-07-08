use crate::use_case::{UseCase, UseCaseInput, UseCaseOutput};
use apllodb_immutable_schema_engine_domain::{
    ApparentPrimaryKey, ApparentPrimaryKeyColumnNames, ImmutableSchemaTx, NonPKColumnName,
    VTableId, VTableRepository, VersionId, VersionRepository,
};
use apllodb_shared_components::{
    data_structure::{ColumnName, DatabaseName, Expression, SqlValue, TableName},
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use apllodb_storage_engine_interface::PrimaryKey;
use std::{collections::HashMap, fmt::Debug, marker::PhantomData};

#[derive(Eq, PartialEq, Debug, new)]
pub struct InsertUseCaseInput<'a, 'tx, 'db: 'tx, Tx: ImmutableSchemaTx<'tx, 'db>> {
    tx: &'tx Tx,

    database_name: &'a DatabaseName,
    table_name: &'a TableName,
    column_values: &'a HashMap<ColumnName, Expression>,

    #[new(default)]
    _marker: PhantomData<&'db ()>,
}
impl<'a, 'tx, 'db: 'tx, Tx: ImmutableSchemaTx<'tx, 'db>> UseCaseInput
    for InsertUseCaseInput<'a, 'tx, 'db, Tx>
{
    fn validate(&self) -> ApllodbResult<()> {
        self.validate_expression_type()?;
        Ok(())
    }
}
impl<'a, 'tx, 'db: 'tx, Tx: ImmutableSchemaTx<'tx, 'db>> InsertUseCaseInput<'a, 'tx, 'db, Tx> {
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

pub struct InsertUseCase<'a, 'tx, 'db: 'tx, Tx: ImmutableSchemaTx<'tx, 'db>> {
    _marker: PhantomData<&'a &'tx &'db Tx>,
}
impl<'a, 'tx, 'db: 'tx, Tx: ImmutableSchemaTx<'tx, 'db>> UseCase
    for InsertUseCase<'a, 'tx, 'db, Tx>
{
    type In = InsertUseCaseInput<'a, 'tx, 'db, Tx>;
    type Out = InsertUseCaseOutput;

    /// # Failures
    ///
    /// - [FeatureNotSupported](error/enum.ApllodbErrorKind.html#variant.FeatureNotSupported) when:
    ///   - any column_values' Expression is not a ConstantVariant.
    fn run_core(input: Self::In) -> ApllodbResult<Self::Out> {
        let vtable_repo = input.tx.vtable_repo();
        let version_repo = input.tx.version_repo();

        // Construct ApparentPrimaryKey
        let vtable_id = VTableId::new(input.database_name, input.table_name);
        let vtable = vtable_repo.read(&vtable_id)?;
        let apk_cdts = vtable
            .table_wide_constraints()
            .apparent_pk_column_data_types();
        let apk_column_names = apk_cdts
            .iter()
            .map(|cdt| cdt.column_name())
            .cloned()
            .collect::<Vec<ColumnName>>();
        let apk_sql_values = apk_cdts
            .iter()
            .map(|cdt| {
                let expr = input.column_values.get(cdt.column_name()).ok_or_else(|| {
                    ApllodbError::new(
                        ApllodbErrorKind::NotNullViolation,
                        format!(
                            "primary key column `{}` must be specified when INSERTing into table `{}`",
                            cdt.column_name(),
                            vtable.table_name()
                        ),
                        None,
                    )
                })?;
                SqlValue::try_from(expr, cdt.data_type())
            })
            .collect::<ApllodbResult<Vec<SqlValue>>>()?;
        let apk = ApparentPrimaryKey::new(
            ApparentPrimaryKeyColumnNames::new(apk_column_names),
            apk_sql_values,
        );

        // Filter Non-PK columns
        let non_pk_column_values: HashMap<NonPKColumnName, Expression> = input
            .column_values
            .clone()
            .into_iter()
            .filter_map(|(column_name, expr)| {
                if apk.column_names().contains(&column_name) {
                    None
                } else {
                    Some((NonPKColumnName::new(column_name), expr))
                }
            })
            .collect();

        // Determine version to insert
        let active_versions = vtable_repo.active_versions(&vtable)?;
        let version_to_insert = active_versions.version_to_insert(input.column_values)?;
        let version_id = VersionId::new(&vtable_id, version_to_insert.number());

        version_repo.insert(&version_id, apk, &non_pk_column_values)?;

        Ok(InsertUseCaseOutput)
    }
}
