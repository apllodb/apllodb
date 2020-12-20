use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
};

use apllodb_shared_components::{
    data_structure::ColumnName,
    error::{ApllodbError, ApllodbErrorKind, ApllodbResult},
};
use apllodb_storage_engine_interface::StorageEngine;
use serde::{Deserialize, Serialize};

use crate::{
    abstract_types::ImmutableSchemaAbstractTypes,
    entity::Entity,
    version::id::VersionId,
    vtable::{repository::VTableRepository, VTable},
};

/// Projection query for single table column references.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum ProjectionQuery {
    All,
    ColumnNames(Vec<ColumnName>),
}

pub struct ProjectionResult<
    'prj,
    'db: 'prj,
    Engine: StorageEngine<'prj, 'db>,
    Types: ImmutableSchemaAbstractTypes<'prj, 'db, Engine>,
> {
    result_per_version: HashMap<VersionId, ProjectionResultInVersion>,

    _marker: PhantomData<(&'prj &'db (), Engine, Types)>,
}
impl<
        'prj,
        'db: 'prj,
        Engine: StorageEngine<'prj, 'db>,
        Types: ImmutableSchemaAbstractTypes<'prj, 'db, Engine>,
    > ProjectionResult<'prj, 'db, Engine, Types>
{
    pub fn new(
        tx: &'prj Engine::Tx,
        vtable: VTable,
        query: ProjectionQuery,
    ) -> ApllodbResult<Self> {
        let vtable_repo = Types::VTableRepo::new(tx);
        let active_versions = vtable_repo.active_versions(&vtable)?;

        let pk_columns: HashSet<ColumnName> = vtable
            .table_wide_constraints()
            .pk_column_names()
            .into_iter()
            .collect();

        let versions_available_columns_with_pk: HashSet<ColumnName> = active_versions
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
            .chain(pk_columns)
            .collect();

        let query_columns: Vec<ColumnName> = match query {
            ProjectionQuery::All => versions_available_columns_with_pk.iter().cloned().collect(),
            ProjectionQuery::ColumnNames(cn) => cn,
        };

        // check UndefinedColumn
        for q_cn in &query_columns {
            if !versions_available_columns_with_pk.contains(q_cn) {
                return Err(ApllodbError::new(
                    ApllodbErrorKind::UndefinedColumn,
                    format!("undefined column `{}` is queried", q_cn),
                    None,
                ));
            }
        }

        let mut result_per_version: HashMap<VersionId, ProjectionResultInVersion> = HashMap::new();
        for active_version in active_versions.as_sorted_slice() {
            let version_id = active_version.id();

            let version_columns: HashSet<&ColumnName> = active_version
                .column_data_types()
                .iter()
                .map(|cdt| cdt.column_ref().as_column_name())
                .collect();

            let (effective_columns, void_columns): (Vec<ColumnName>, Vec<ColumnName>) =
                query_columns
                    .iter()
                    .cloned()
                    .partition(|q_cn| version_columns.contains(q_cn));

            let result_in_version = ProjectionResultInVersion {
                effective: effective_columns,
                void: void_columns,
            };

            result_per_version.insert(version_id.clone(), result_in_version);
        }

        Ok(Self {
            result_per_version,
            _marker: PhantomData::default(),
        })
    }

    pub fn effective_projection(&self, version_id: &VersionId) -> ApllodbResult<&Vec<ColumnName>> {
        self.result_per_version
            .get(version_id)
            .map(|result_in_version| &result_in_version.effective)
            .ok_or_else(|| {
                ApllodbError::new(
                    ApllodbErrorKind::InvalidVersion,
                    format!("invalid version `{:?}` is queried", version_id),
                    None,
                )
            })
    }

    pub fn void_projection(&self, version_id: &VersionId) -> ApllodbResult<&Vec<ColumnName>> {
        self.result_per_version
            .get(version_id)
            .map(|result_in_version| &result_in_version.void)
            .ok_or_else(|| {
                ApllodbError::new(
                    ApllodbErrorKind::InvalidVersion,
                    format!("invalid version `{:?}` is queried", version_id),
                    None,
                )
            })
    }
}

struct ProjectionResultInVersion {
    effective: Vec<ColumnName>,
    void: Vec<ColumnName>,
}
