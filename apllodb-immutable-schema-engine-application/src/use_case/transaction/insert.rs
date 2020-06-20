use crate::use_case::{UseCase, UseCaseInput, UseCaseOutput};
use apllodb_immutable_schema_engine_domain::{ImmutableSchemaTx, VTableId};
use apllodb_immutable_schema_engine_domain::{VersionId, VersionRepository};
use apllodb_shared_components::{
    data_structure::{ColumnName, DatabaseName, Expression, TableName},
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
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
                Expression::ColumnNameVariant(_) => {
                    return Err(ApllodbError::new(ApllodbErrorKind::FeatureNotSupported,
                        format!("trying to insert `{:?}={:?}` while expr of `INSERT INTO ... VALUES (expr ...)` can only be a constant", 
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
        let version_repo = input.tx.version_repo();

        let vtable_id = VTableId::new(input.database_name, input.table_name);
        let active_versions = version_repo.active_versions(&vtable_id)?;
        let version_to_insert = active_versions.version_to_insert(input.column_values)?;
        let version_id = VersionId::new(&vtable_id, version_to_insert.number());

        version_repo.insert(&version_id, input.column_values)?;

        Ok(InsertUseCaseOutput)
    }
}
