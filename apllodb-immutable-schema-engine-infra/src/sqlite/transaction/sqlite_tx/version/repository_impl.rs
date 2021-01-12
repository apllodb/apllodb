use crate::{
    external_interface::ApllodbImmutableSchemaEngine,
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
use std::collections::HashMap;

use super::dao::VersionDao;

#[derive(Debug)]
pub struct VersionRepositoryImpl<'repo, 'sess: 'repo> {
    tx: &'repo SqliteTx<'sess>,
}

impl<'repo, 'sess> VersionRepositoryImpl<'repo, 'sess> {
    pub fn new(tx: &'repo SqliteTx<'sess>) -> Self {
        Self { tx }
    }
}

impl<'repo, 'sess: 'repo> VersionRepository<ApllodbImmutableSchemaEngine<'sess>>
    for VersionRepositoryImpl<'repo, 'sess>
{
    /// # Failures
    ///
    /// - [DuplicateTable](apllodb_shared_components::ApllodbErrorKind::DuplicateTable) when:
    ///   - Table `table_name` is already visible to this transaction.
    /// - Errors from [TableDao::create()](foobar.html).
    fn create(&self, version: &ActiveVersion) -> ApllodbResult<()> {
        self.version_dao().create_table(&version)?;
        Ok(())
    }

    fn deactivate(&self, _version_id: &VersionId) -> ApllodbResult<()> {
        todo!()
    }

    fn insert(
        &self,
        version_id: &VersionId,
        apparent_pk: ApparentPrimaryKey,
        column_values: &HashMap<ColumnName, SqlValue>,
    ) -> ApllodbResult<()> {
        let vrr_entry = self.vrr().register(version_id, apparent_pk)?;

        self.version_dao()
            .insert(&version_id, vrr_entry.id(), &column_values)?;
        Ok(())
    }
}

impl<'repo, 'sess: 'repo> VersionRepositoryImpl<'repo, 'sess> {
    fn vrr(&self) -> VersionRevisionResolverImpl {
        VersionRevisionResolverImpl::new(self.tx)
    }

    fn version_dao(&self) -> VersionDao<'repo, 'sess> {
        VersionDao::new(&self.tx)
    }
}
