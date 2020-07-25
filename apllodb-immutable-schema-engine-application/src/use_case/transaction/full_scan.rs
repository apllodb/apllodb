use crate::use_case::{UseCase, UseCaseInput, UseCaseOutput};
use apllodb_immutable_schema_engine_domain::{
    ImmutableSchemaRowIter, VTableRepository, VersionRepository,
};
use apllodb_immutable_schema_engine_domain::{ImmutableSchemaTx, VTableId, row::column::non_pk_column::filter_non_pk_column_names};
use apllodb_shared_components::{
    data_structure::{ColumnName, DatabaseName, TableName},
    error::ApllodbResult,
};

use std::{fmt::Debug, marker::PhantomData};

#[derive(Eq, PartialEq, Debug, new)]
pub struct FullScanUseCaseInput<'a, 'tx, 'db: 'tx, Tx: ImmutableSchemaTx<'tx, 'db>> {
    tx: &'tx Tx,

    database_name: &'a DatabaseName,
    table_name: &'a TableName,
    column_names: &'a [ColumnName],

    #[new(default)]
    _marker: PhantomData<&'db ()>,
}
impl<'a, 'tx, 'db: 'tx, Tx: ImmutableSchemaTx<'tx, 'db>> UseCaseInput
    for FullScanUseCaseInput<'a, 'tx, 'db, Tx>
{
    fn validate(&self) -> ApllodbResult<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct FullScanUseCaseOutput<'tx, 'db: 'tx, Tx: ImmutableSchemaTx<'tx, 'db>> {
    pub row_iter: ImmutableSchemaRowIter<
        <<Tx as ImmutableSchemaTx<'tx, 'db>>::VRepo as VersionRepository<'tx, 'db>>::VerRowIter,
    >,
}
impl<'tx, 'db: 'tx, Tx: ImmutableSchemaTx<'tx, 'db>> UseCaseOutput
    for FullScanUseCaseOutput<'tx, 'db, Tx>
{
}

pub struct FullScanUseCase<'a, 'tx, 'db: 'tx, Tx: ImmutableSchemaTx<'tx, 'db>> {
    _marker: PhantomData<&'a &'tx &'db Tx>,
}
impl<'a, 'tx, 'db: 'tx, Tx: ImmutableSchemaTx<'tx, 'db>> UseCase
    for FullScanUseCase<'a, 'tx, 'db, Tx>
{
    type In = FullScanUseCaseInput<'a, 'tx, 'db, Tx>;
    type Out = FullScanUseCaseOutput<'tx, 'db, Tx>;

    /// # Failures
    ///
    /// - [FeatureNotSupported](error/enum.ApllodbErrorKind.html#variant.FeatureNotSupported) when:
    ///   - any column_values' Expression is not a ConstantVariant.
    fn run_core(input: Self::In) -> ApllodbResult<Self::Out> {
        let vtable_repo = input.tx.vtable_repo();

        let vtable_id = VTableId::new(input.database_name, input.table_name);
        let vtable = vtable_repo.read(&vtable_id)?;
        let non_pk_column_names =
            filter_non_pk_column_names(input.column_names, &vtable.apk_column_names());

        let row_iter = vtable_repo.full_scan(&vtable_id, &non_pk_column_names)?;
        Ok(FullScanUseCaseOutput { row_iter })
    }
}
