use crate::use_case::{UseCase, UseCaseInput, UseCaseOutput};
use apllodb_immutable_schema_engine_domain::vtable::{id::VTableId, repository::VTableRepository};
use apllodb_immutable_schema_engine_domain::{
    row::column::non_pk_column::filter_non_pk_column_names,
};
use apllodb_shared_components::{
    data_structure::{ColumnName, DatabaseName, TableName},
    error::ApllodbResult,
};
use apllodb_storage_engine_interface::StorageEngine;

use std::{fmt::Debug, marker::PhantomData};

#[derive(Eq, PartialEq, Debug, new)]
pub struct FullScanUseCaseInput<'a, 'tx: 'a, 'db: 'tx, Engine: StorageEngine<'db>> {
    tx: &'a Engine::Tx,
    database_name: &'a DatabaseName,
    table_name: &'a TableName,
    column_names: &'a [ColumnName],

    #[new(default)]
    _marker: PhantomData<&'tx &'db ()>,
}
impl<'a, 'tx: 'a, 'db: 'tx, Engine: StorageEngine<'db>> UseCaseInput
    for FullScanUseCaseInput<'a, 'tx, 'db, Engine>
{
    fn validate(&self) -> ApllodbResult<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct FullScanUseCaseOutput<'db, Engine: StorageEngine<'db>> {
    pub row_iter: Engine::RowIter,
}
impl<'db, Engine: StorageEngine<'db>> UseCaseOutput for FullScanUseCaseOutput<'db, Engine> {}

pub struct FullScanUseCase<'a, 'tx: 'a, 'db: 'tx, Engine: StorageEngine<'db>> {
    _marker: PhantomData<(&'a &'tx &'db (), Engine)>,
}
impl<'a, 'tx: 'a, 'db: 'tx, Engine: StorageEngine<'db>> UseCase
    for FullScanUseCase<'a, 'tx, 'db, Engine>
{
    type In = FullScanUseCaseInput<'a, 'tx, 'db, Engine>;
    type Out = FullScanUseCaseOutput<'db, Engine>;

    /// # Failures
    ///
    /// - [FeatureNotSupported](error/enum.ApllodbErrorKind.html#variant.FeatureNotSupported) when:
    ///   - any column_values' Expression is not a ConstantVariant.
    fn run_core(input: Self::In) -> ApllodbResult<Self::Out> {
        let vtable_repo = input.tx.vtable_repo();

        let vtable_id = VTableId::new(input.database_name, input.table_name);
        let vtable = vtable_repo.read(&vtable_id)?;
        let non_pk_column_names = filter_non_pk_column_names(
            input.column_names,
            &vtable.table_wide_constraints().pk_column_names(),
        );

        let row_iter = vtable_repo.full_scan(&vtable_id, &non_pk_column_names)?;
        Ok(FullScanUseCaseOutput { row_iter })
    }
}
