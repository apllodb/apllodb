use std::collections::{HashMap, HashSet};

use apllodb_shared_components::{
    ApllodbError, ApllodbErrorKind, ApllodbResult, ColumnName, CorrelationReference,
    FieldReference, FullFieldReference, RecordFieldRefSchema,
};
use apllodb_storage_engine_interface::ProjectionQuery;
use serde::{Deserialize, Serialize};

use crate::{
    entity::Entity,
    version::{active_versions::ActiveVersions, id::VersionId},
    vtable::VTable,
};

/// Has projected columns for each version in a VTable.
#[derive(Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct ProjectionResult {
    result_per_version: HashMap<VersionId, ProjectionResultInVersion>,
}

impl ProjectionResult {
    /// Calculate and construct ProjectionResult
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
                    .map(|cdt| cdt.column_name())
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
                .map(|cdt| cdt.column_name())
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

        Ok(Self { result_per_version })
    }

    /// Columns included in [ProjectionQuery](apllodb-shared-components::ProjectionQuery) and primary key for this version.
    pub fn pk_effective_projection(
        &self,
        version_id: &VersionId,
    ) -> ApllodbResult<&Vec<ColumnName>> {
        self.get_projection_core(version_id, |result_in_version| {
            &result_in_version.pk_effective
        })
    }

    /// Columns included in [ProjectionQuery](apllodb-shared-components::ProjectionQuery) but not in the primary key for this version.
    pub fn pk_void_projection(&self, version_id: &VersionId) -> ApllodbResult<&Vec<ColumnName>> {
        self.get_projection_core(version_id, |result_in_version| &result_in_version.pk_void)
    }

    /// Columns included in [ProjectionQuery](apllodb-shared-components::ProjectionQuery) and non-PK for this version.
    pub fn non_pk_effective_projection(
        &self,
        version_id: &VersionId,
    ) -> ApllodbResult<&Vec<ColumnName>> {
        self.get_projection_core(version_id, |result_in_version| {
            &result_in_version.non_pk_effective
        })
    }

    /// Columns included in [ProjectionQuery](apllodb-shared-components::ProjectionQuery) but not in the non-PK for this version.
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

impl From<ProjectionResult> for RecordFieldRefSchema {
    fn from(pr: ProjectionResult) -> Self {
        assert!(
            !pr.result_per_version.is_empty(),
            "at least 1 pr.result_per_version should exist for CREATEd table"
        );

        let (version_id, _) = pr.result_per_version.iter().next().unwrap();
        let table_name = version_id.vtable_id().table_name().clone();

        let mut all_column_names: Vec<ColumnName> = vec![];

        for (_, mut projection_result_in_version) in pr.result_per_version {
            all_column_names.append(&mut projection_result_in_version.pk_effective);
            all_column_names.append(&mut projection_result_in_version.pk_void);
            all_column_names.append(&mut projection_result_in_version.non_pk_effective);
            all_column_names.append(&mut projection_result_in_version.non_pk_void);
        }

        let ffrs: Vec<FullFieldReference> = all_column_names
            .into_iter()
            .map(|cn| {
                FullFieldReference::new(
                    CorrelationReference::TableNameVariant(table_name.clone()),
                    FieldReference::ColumnNameVariant(cn),
                )
            })
            .collect();

        RecordFieldRefSchema::new(ffrs)
    }
}
