use std::{cell::RefCell, rc::Rc};

use super::vtable_metadata_dao::VTableMetadataDao;
use crate::sqlite::{
    rows::chain_rows::ChainRows,
    sqlite_types::{RowSelectionPlan, SqliteTypes, VrrEntries},
    transaction::sqlite_tx::{
        version::dao::{version_dao::VersionDao, version_metadata_dao::VersionMetadataDao},
        version_revision_resolver::VersionRevisionResolverImpl,
        SqliteTx,
    },
};
use apllodb_immutable_schema_engine_domain::{
    entity::Entity,
    row::pk::apparent_pk::ApparentPrimaryKey,
    row_projection_result::RowProjectionResult,
    version::active_versions::ActiveVersions,
    version_revision_resolver::VersionRevisionResolver,
    vtable::repository::VTableRepository,
    vtable::{id::VTableId, VTable},
};
use apllodb_shared_components::{
    ApllodbError, ApllodbErrorKind, ApllodbResult, BooleanExpression, ComparisonFunction,
    Expression, SqlValue,
};
use apllodb_storage_engine_interface::{
    Row, RowSchema, RowSelectionQuery, Rows, SingleTableCondition,
};
use async_trait::async_trait;

#[derive(Debug)]
pub struct VTableRepositoryImpl {
    tx: Rc<RefCell<SqliteTx>>,
}

impl VTableRepositoryImpl {
    pub(crate) fn new(tx: Rc<RefCell<SqliteTx>>) -> Self {
        Self { tx }
    }
}

#[async_trait(?Send)]
impl VTableRepository<SqliteTypes> for VTableRepositoryImpl {
    /// # Failures
    ///
    /// - [DuplicateTable](apllodb_shared_components::ApllodbErrorKind::DuplicateTable) when:
    ///   - Table `table_name` is already visible to this transaction.
    /// - Errors from [TableDao::create()](foobar.html).
    async fn create(&self, vtable: &VTable) -> ApllodbResult<()> {
        self.vtable_metadata_dao().insert(&vtable).await?;
        self.vrr().create_table(&vtable).await?;
        Ok(())
    }

    /// # Failures
    ///
    /// - [IoError](apllodb_shared_components::ApllodbErrorKind::IoError) when:
    ///   - sqlx raises an error.
    /// - [UndefinedTable](apllodb_shared_components::ApllodbErrorKind::UndefinedTable) when:
    ///   - Table `table_name` is not visible to this transaction.
    async fn read(&self, vtable_id: &VTableId) -> ApllodbResult<VTable> {
        self.vtable_metadata_dao().select(&vtable_id).await
    }

    /// # Failures
    ///
    /// - [UndefinedTable](apllodb_shared_components::ApllodbErrorKind::UndefinedTable) when:
    ///   - Table `table_name` is not visible to this transaction.
    /// - [IoError](apllodb_shared_components::ApllodbErrorKind::IoError) when:
    ///   - sqlx raises an error.
    async fn update(&self, _vtable: &VTable) -> ApllodbResult<()> {
        // TODO update VTable on TableWideConstraints change.
        Ok(())
    }

    async fn plan_selection(
        &self,
        vtable: &VTable,
        selection_query: RowSelectionQuery,
    ) -> ApllodbResult<RowSelectionPlan> {
        match selection_query {
            RowSelectionQuery::FullScan => Ok(RowSelectionPlan::FullScan),
            RowSelectionQuery::Condition(c) => self.plan_selection_from_condition(vtable, c).await,
        }
    }

    async fn active_versions(&self, vtable: &VTable) -> ApllodbResult<ActiveVersions> {
        let active_versions = self
            .version_metadata_dao()
            .select_active_versions(vtable.id())
            .await?;
        Ok(ActiveVersions::from(active_versions))
    }

    async fn probe_vrr_entries(
        &self,
        vrr_entries: VrrEntries,
        projection: RowProjectionResult,
    ) -> ApllodbResult<Rows> {
        let vtable = self
            .vtable_metadata_dao()
            .select(&vrr_entries.vtable_id())
            .await?;

        let mut all_ver_rows = Vec::<Rows>::new();

        for vrr_entries_in_version in vrr_entries.group_by_version_id() {
            let version = self
                .version_metadata_dao()
                .select_active_version(&vtable.id(), vrr_entries_in_version.version_id())
                .await?;

            let ver_rows = self
                .version_dao()
                .probe_in_version(&version, vrr_entries_in_version, &projection)
                .await?;

            all_ver_rows.push(ver_rows);
        }

        if all_ver_rows.is_empty() {
            Ok(Rows::new(RowSchema::from(projection), Vec::<Row>::new()))
        } else {
            let rows = ChainRows::chain(all_ver_rows);
            Ok(rows)
        }
    }

    fn vrr(&self) -> VersionRevisionResolverImpl {
        VersionRevisionResolverImpl::new(self.tx.clone())
    }
}

impl VTableRepositoryImpl {
    fn vtable_metadata_dao(&self) -> VTableMetadataDao {
        VTableMetadataDao::new(self.tx.clone())
    }

    fn version_dao(&self) -> VersionDao {
        VersionDao::new(self.tx.clone())
    }

    fn version_metadata_dao(&self) -> VersionMetadataDao {
        VersionMetadataDao::new(self.tx.clone())
    }

    async fn plan_selection_from_condition(
        &self,
        vtable: &VTable,
        condition: SingleTableCondition,
    ) -> ApllodbResult<RowSelectionPlan> {
        match self.try_condition_into_apk(vtable, &condition) {
            Ok(apk) => {
                let vrr_entries = self.vrr().probe(vtable.id(), vec![apk]).await?;
                Ok(RowSelectionPlan::VrrProbe(vrr_entries))
            }
            Err(e) => match e.kind() {
                ApllodbErrorKind::FeatureNotSupported | ApllodbErrorKind::UndefinedPrimaryKey => {
                    Err(ApllodbError::feature_not_supported("for storage-engine selection, only expression like `pk_col = 123` is supported currently"))
                }
                _ => Err(e)
            }
        }
    }

    /// # Failures
    ///
    /// - [UndefinedPrimaryKey](apllodb_shared_components::ApllodbErrorKind::UndefinedPrimaryKey) when:
    ///   - `condition` is not like: `pk_col = 123`
    ///   - TODO support: `pk_col1 = 123 AND pk_col2 = 456`
    /// - [FeatureNotSupported](apllodb_shared_components::ApllodbErrorKind::FeatureNotSupported) when:
    ///   - PK contains multiple columns.
    ///   - `condition` is like: `pk_col1 = 123 AND pk_col2 = 456`
    fn try_condition_into_apk(
        &self,
        vtable: &VTable,
        // FIXME no clone of SqlValue inside (pass ownership and return it on error?)
        condition: &SingleTableCondition,
    ) -> ApllodbResult<ApparentPrimaryKey> {
        let pk_column_names = vtable.table_wide_constraints().pk_column_names();
        let pk_column_name = if pk_column_names.len() == 1 {
            Ok(pk_column_names.first().expect("length checked"))
        } else {
            Err(ApllodbError::feature_not_supported(
                "storage-engine selection with compound PK is not supported currently",
            ))
        }?;

        let err = ApllodbError::new(ApllodbErrorKind::UndefinedPrimaryKey, "", None);

        let expr = condition.as_expression();
        match expr {
            Expression::BooleanExpressionVariant(BooleanExpression::ComparisonFunctionVariant(
                cf,
            )) => match cf {
                ComparisonFunction::EqualVariant { left, right } => {
                    match (left.as_ref(), right.as_ref()) {
                        (
                            Expression::ConstantVariant(sql_value),
                            Expression::SchemaIndexVariant(index),
                        )
                        | (
                            Expression::SchemaIndexVariant(index),
                            Expression::ConstantVariant(sql_value),
                        ) => {
                            if index.attr() == pk_column_name.as_str() {
                                if let SqlValue::NotNull(nn_sql_value) = sql_value {
                                    Ok(ApparentPrimaryKey::new(
                                        vtable.table_name().clone(),
                                        vec![pk_column_name.clone()],
                                        vec![nn_sql_value.clone()],
                                    ))
                                } else {
                                    Err(err)
                                }
                            } else {
                                Err(err)
                            }
                        }
                        _ => Err(err),
                    }
                }
            },
            _ => Err(err),
        }
    }
}
