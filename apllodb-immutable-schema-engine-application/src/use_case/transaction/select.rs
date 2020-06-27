use crate::use_case::{UseCase, UseCaseInput, UseCaseOutput};
use apllodb_immutable_schema_engine_domain::{
    ImmutableSchemaRowIter, VersionId, VersionRepository,
};
use apllodb_immutable_schema_engine_domain::{ImmutableSchemaTx, VTableId};
use apllodb_shared_components::{
    data_structure::{ColumnName, DatabaseName, TableName},
    error::ApllodbResult,
};

use std::{fmt::Debug, marker::PhantomData};

#[derive(Eq, PartialEq, Debug, new)]
pub struct SelectUseCaseInput<'a, 'tx, 'db: 'tx, Tx: ImmutableSchemaTx<'tx, 'db>> {
    tx: &'tx Tx,

    database_name: &'a DatabaseName,
    table_name: &'a TableName,
    column_names: &'a [ColumnName],

    #[new(default)]
    _marker: PhantomData<&'db ()>,
}
impl<'a, 'tx, 'db: 'tx, Tx: ImmutableSchemaTx<'tx, 'db>> UseCaseInput
    for SelectUseCaseInput<'a, 'tx, 'db, Tx>
{
    fn validate(&self) -> ApllodbResult<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct SelectUseCaseOutput<'tx, 'db: 'tx, Tx: ImmutableSchemaTx<'tx, 'db>> {
    pub row_iter: ImmutableSchemaRowIter<
        <<Tx as ImmutableSchemaTx<'tx, 'db>>::VRepo as VersionRepository<'tx, 'db>>::VerRowIter,
    >,
}
impl<'tx, 'db: 'tx, Tx: ImmutableSchemaTx<'tx, 'db>> UseCaseOutput
    for SelectUseCaseOutput<'tx, 'db, Tx>
{
}

pub struct SelectUseCase<'a, 'tx, 'db: 'tx, Tx: ImmutableSchemaTx<'tx, 'db>> {
    _marker: PhantomData<&'a &'tx &'db Tx>,
}
impl<'a, 'tx, 'db: 'tx, Tx: ImmutableSchemaTx<'tx, 'db>> UseCase
    for SelectUseCase<'a, 'tx, 'db, Tx>
{
    type In = SelectUseCaseInput<'a, 'tx, 'db, Tx>;
    type Out = SelectUseCaseOutput<'tx, 'db, Tx>;

    /// # Failures
    ///
    /// - [FeatureNotSupported](error/enum.ApllodbErrorKind.html#variant.FeatureNotSupported) when:
    ///   - any column_values' Expression is not a ConstantVariant.
    fn run_core(input: Self::In) -> ApllodbResult<Self::Out> {
        let version_repo = input.tx.version_repo();

        let vtable_id = VTableId::new(input.database_name, input.table_name);

        // TODO このあたりの処理は見直す必要がある。
        // ActiveVersionだからといって、全てのレコードがほしいのではない。最新revisionだけほしい。
        // naviテーブルをなめて、各APK最新revisionの (VersionNumber, SurrogateId) を特定し、各バージョンに対してselectを投げる。
        // そう考えると、VersionDao::selectは、フルスキャンのインターフェイスは不要で、必ずSurrogateId列でselectionする。
        
        let active_versions = version_repo.active_versions(&vtable_id)?;
        let versions_to_select = active_versions.versions_to_select()?;

        let version_row_iters = versions_to_select
            .iter()
            .map(|v| {
                let version_id = VersionId::new(&vtable_id, v.number());
                version_repo.full_scan(&version_id, input.column_names)
            })
            .collect::<ApllodbResult<Vec<_>>>()?;

        let row_iter = ImmutableSchemaRowIter::chain(version_row_iters);
        Ok(SelectUseCaseOutput { row_iter })
    }
}
