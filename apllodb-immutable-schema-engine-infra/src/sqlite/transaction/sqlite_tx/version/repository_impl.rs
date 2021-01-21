use super::dao::VersionDao;
use crate::{
    engine::ApllodbImmutableSchemaEngine,
    sqlite::transaction::sqlite_tx::{
        version_revision_resolver::VersionRevisionResolverImpl, SqliteTx,
    },
};
use apllodb_immutable_schema_engine_domain::{
    entity::Entity,
    row::pk::apparent_pk::ApparentPrimaryKey,
    version::{active_version::ActiveVersion, id::VersionId, repository::VersionRepository},
    version_revision_resolver::VersionRevisionResolver,
};
use apllodb_shared_components::ApllodbResult;
use apllodb_shared_components::{ColumnName, SqlValue};
use async_trait::async_trait;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug)]
pub struct VersionRepositoryImpl<'sqcn> {
    tx: Rc<RefCell<SqliteTx<'sqcn>>>,
}

impl<'sqcn> VersionRepositoryImpl<'sqcn> {
    pub fn new(tx: Rc<RefCell<SqliteTx<'sqcn>>>) -> Self {
        Self { tx }
    }
}

#[async_trait(?Send)]
impl<'sqcn> VersionRepository for VersionRepositoryImpl<'sqcn> {
    /// # Failures
    ///
    /// - [DuplicateTable](apllodb_shared_components::ApllodbErrorKind::DuplicateTable) when:
    ///   - Table `table_name` is already visible to this transaction.
    /// - Errors from [TableDao::create()](foobar.html).
    async fn create(&self, version: &ActiveVersion) -> ApllodbResult<()> {
        self.version_dao().create_table(&version).await?;
        Ok(())
    }

    async fn deactivate(&self, _version_id: &VersionId) -> ApllodbResult<()> {
        todo!()
    }

    async fn insert(
        &self,
        version_id: &VersionId,
        apparent_pk: ApparentPrimaryKey,
        column_values: &HashMap<ColumnName, SqlValue>,
    ) -> ApllodbResult<()> {
        let vrr_entry = self.vrr().register(version_id, apparent_pk).await?;

        self.version_dao()
            .insert(&version_id, vrr_entry.id(), &column_values)
            .await?;
        Ok(())
    }
}

impl<'sqcn> VersionRepositoryImpl<'sqcn> {
    fn vrr(&self) -> VersionRevisionResolverImpl<'sqcn> {
        VersionRevisionResolverImpl::new(self.tx.clone())
    }

    fn version_dao(&self) -> VersionDao<'sqcn> {
        VersionDao::new(self.tx.clone())
    }
}
