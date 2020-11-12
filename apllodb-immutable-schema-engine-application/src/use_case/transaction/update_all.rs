use crate::use_case::{UseCase, UseCaseInput, UseCaseOutput};
use apllodb_immutable_schema_engine_domain::abstract_types::ImmutableSchemaAbstractTypes;
use apllodb_shared_components::{
    data_structure::{ColumnName, DatabaseName, Expression, TableName},
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use apllodb_storage_engine_interface::StorageEngine;
use std::{collections::HashMap, fmt::Debug, marker::PhantomData};

#[derive(Eq, PartialEq, Debug, new)]
pub struct UpdateAllUseCaseInput<
    'a,
    'tx: 'a,
    'db: 'tx,
    Engine: StorageEngine<'db>,
    Types: ImmutableSchemaAbstractTypes<'tx, 'db, Engine>,
> {
    tx: &'a Types::ImmutableSchemaTx,
    database_name: &'a DatabaseName,
    table_name: &'a TableName,
    column_values: &'a HashMap<ColumnName, Expression>,

    #[new(default)]
    _marker: PhantomData<&'tx &'db ()>,
}
impl<
        'a,
        'tx: 'a,
        'db: 'tx,
        Engine: StorageEngine<'db>,
        Types: ImmutableSchemaAbstractTypes<'tx, 'db, Engine>,
    > UseCaseInput for UpdateAllUseCaseInput<'a, 'tx, 'db, Engine, Types>
{
    fn validate(&self) -> ApllodbResult<()> {
        self.validate_expression_type()?;
        Ok(())
    }
}
impl<
        'a,
        'tx: 'a,
        'db: 'tx,
        Engine: StorageEngine<'db>,
        Types: ImmutableSchemaAbstractTypes<'tx, 'db, Engine>,
    > UpdateAllUseCaseInput<'a, 'tx, 'db, Engine, Types>
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
    'a,
    'tx: 'a,
    'db: 'tx,
    Engine: StorageEngine<'db>,
    Types: ImmutableSchemaAbstractTypes<'tx, 'db, Engine>,
> {
    _marker: PhantomData<(&'a &'tx &'db (), Types, Engine)>,
}
impl<
        'a,
        'tx: 'a,
        'db: 'tx,
        Engine: StorageEngine<'db> + 'a,
        Types: ImmutableSchemaAbstractTypes<'tx, 'db, Engine>,
    > UseCase for UpdateAllUseCase<'a, 'tx, 'db, Engine, Types>
{
    type In = UpdateAllUseCaseInput<'a, 'tx, 'db, Engine, Types>;
    type Out = UpdateAllUseCaseOutput;

    /// # Failures
    ///
    /// - [FeatureNotSupported](error/enum.ApllodbErrorKind.html#variant.FeatureNotSupported) when:
    ///   - any column_values' Expression is not a ConstantVariant.
    fn run_core(_input: Self::In) -> ApllodbResult<Self::Out> {
        todo!()

        // let vtable_repo = input.tx.vtable_repo();
        // let version_repo = input.tx.version_repo();

        // let vtable_id = VTableId::new(input.database_name, input.table_name);
        // let vtable = vtable_repo.read(&vtable_id)?;

        // // Fetch all columns of the latest version rows and update requested columns later.
        // // TODO Consider CoW to reduce disk usage (append only updated column to a new version).
        // let row_iter = vtable_repo.full_scan_all_columns(&vtable_id)?;

        // // Construct ApparentPrimaryKey
        // // FIXME INSERTではないので、必要なvalueがあるとは限らない！！
        // let apk = ApparentPrimaryKey::from_table_and_column_values(&vtable, input.column_values)?;

        // // Filter Non-PK columns from column_values
        // let non_pk_column_values: HashMap<NonPKColumnName, Expression> = input
        //     .column_values
        //     .clone()
        //     .into_iter()
        //     .filter_map(|(column_name, expr)| {
        //         if apk
        //             .column_names()
        //             .iter()
        //             .any(|pk_cn| pk_cn.as_str() == column_name.as_str())
        //         {
        //             None
        //         } else {
        //             Some((NonPKColumnName::from(column_name), expr))
        //         }
        //     })
        //     .collect();

        // // Determine version to update
        // let active_versions = vtable_repo.active_versions(&vtable)?;
        // let version_to_update = active_versions.version_to_update(&non_pk_column_values)?;
        // let version_id = VersionId::new(&vtable_id, version_to_update.number());

        // version_repo.update(&version_id, apk, &non_pk_column_values)?;

        // Ok(UpdateAllUseCaseOutput)
    }
}
