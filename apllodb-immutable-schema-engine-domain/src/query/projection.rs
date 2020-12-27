use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
};

use apllodb_shared_components::{ApllodbError, ApllodbErrorKind, ApllodbResult, ColumnName};
use apllodb_storage_engine_interface::{ProjectionQuery, StorageEngine};
use serde::{Deserialize, Serialize};

use crate::{
    abstract_types::ImmutableSchemaAbstractTypes,
    entity::Entity,
    version::{active_versions::ActiveVersions, id::VersionId},
    vtable::VTable,
};

#[derive(Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct ProjectionResult<
    'prj,
    'db: 'prj,
    Engine: StorageEngine,
    Types: ImmutableSchemaAbstractTypes<'prj, 'db, Engine>,
> {
    result_per_version: HashMap<VersionId, ProjectionResultInVersion>,

    _marker: PhantomData<(&'prj &'db (), Engine, Types)>,
}
impl<
        'prj,
        'db: 'prj,
        Engine: StorageEngine,
        Types: ImmutableSchemaAbstractTypes<'prj, 'db, Engine>,
    > ProjectionResult<'prj, 'db, Engine, Types>
{
    pub fn new(
        vtable: &VTable,
        active_versions: ActiveVersions,
        query: ProjectionQuery,
    ) -> ApllodbResult<Self> {
        let pk_columns: HashSet<ColumnName> = vtable
            .table_wide_constraints()
            .pk_column_names()
            .into_iter()
            .collect();

        let versions_available_columns: HashSet<ColumnName> = active_versions
            .as_sorted_slice()
            .iter()
            .map(|active_version| {
                active_version
                    .column_data_types()
                    .iter()
                    .map(|cdt| cdt.column_ref().as_column_name())
            })
            .flatten()
            .cloned()
            .collect();

        let pk_query_columns: Vec<ColumnName> = match &query {
            ProjectionQuery::All => pk_columns.iter().cloned().collect(),
            ProjectionQuery::ColumnNames(cns) => cns
                .iter()
                .filter(|cn| pk_columns.contains(cn))
                .cloned()
                .collect(),
        };
        let non_pk_query_columns: Vec<ColumnName> = match &query {
            ProjectionQuery::All => versions_available_columns.iter().cloned().collect(),
            ProjectionQuery::ColumnNames(cns) => cns
                .iter()
                .filter(|cn| versions_available_columns.contains(cn))
                .cloned()
                .collect(),
        };

        // check UndefinedColumn
        {
            let available_columns: Vec<&ColumnName> = pk_columns
                .iter()
                .chain(versions_available_columns.iter())
                .collect();

            for q_cn in pk_query_columns.iter().chain(non_pk_query_columns.iter()) {
                if !available_columns.contains(&q_cn) {
                    return Err(ApllodbError::new(
                        ApllodbErrorKind::UndefinedColumn,
                        format!("undefined column `{:?}` is queried", q_cn),
                        None,
                    ));
                }
            }
        }

        let (pk_effective_columns, pk_void_columns): (Vec<ColumnName>, Vec<ColumnName>) =
            pk_query_columns
                .iter()
                .cloned()
                .partition(|q_cn| pk_columns.contains(q_cn));

        let mut result_per_version: HashMap<VersionId, ProjectionResultInVersion> = HashMap::new();
        for active_version in active_versions.as_sorted_slice() {
            let version_id = active_version.id();

            let version_columns: HashSet<&ColumnName> = active_version
                .column_data_types()
                .iter()
                .map(|cdt| cdt.column_ref().as_column_name())
                .collect();

            let (non_pk_effective_columns, non_pk_void_columns): (
                Vec<ColumnName>,
                Vec<ColumnName>,
            ) = non_pk_query_columns
                .iter()
                .cloned()
                .partition(|q_cn| version_columns.contains(q_cn));

            let result_in_version = ProjectionResultInVersion {
                pk_effective: pk_effective_columns.clone(),
                pk_void: pk_void_columns.clone(),
                non_pk_effective: non_pk_effective_columns,
                non_pk_void: non_pk_void_columns,
            };

            result_per_version.insert(version_id.clone(), result_in_version);
        }

        Ok(Self {
            result_per_version,
            _marker: PhantomData::default(),
        })
    }

    pub fn pk_effective_projection(
        &self,
        version_id: &VersionId,
    ) -> ApllodbResult<&Vec<ColumnName>> {
        self.get_projection_core(version_id, |result_in_version| {
            &result_in_version.pk_effective
        })
    }

    pub fn pk_void_projection(&self, version_id: &VersionId) -> ApllodbResult<&Vec<ColumnName>> {
        self.get_projection_core(version_id, |result_in_version| &result_in_version.pk_void)
    }

    pub fn non_pk_effective_projection(
        &self,
        version_id: &VersionId,
    ) -> ApllodbResult<&Vec<ColumnName>> {
        self.get_projection_core(version_id, |result_in_version| {
            &result_in_version.non_pk_effective
        })
    }

    pub fn non_pk_void_projection(
        &self,
        version_id: &VersionId,
    ) -> ApllodbResult<&Vec<ColumnName>> {
        self.get_projection_core(version_id, |result_in_version| {
            &result_in_version.non_pk_void
        })
    }

    fn get_projection_core<F: FnOnce(&ProjectionResultInVersion) -> &Vec<ColumnName>>(
        &self,
        version_id: &VersionId,
        columns_from_result_in_version: F,
    ) -> ApllodbResult<&Vec<ColumnName>> {
        self.result_per_version
            .get(version_id)
            .map(columns_from_result_in_version)
            .ok_or_else(|| {
                ApllodbError::new(
                    ApllodbErrorKind::InvalidVersion,
                    format!("invalid version `{:?}` is queried", version_id),
                    None,
                )
            })
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Serialize, Deserialize)]
struct ProjectionResultInVersion {
    pk_effective: Vec<ColumnName>,
    pk_void: Vec<ColumnName>,

    non_pk_effective: Vec<ColumnName>,
    non_pk_void: Vec<ColumnName>,
}
