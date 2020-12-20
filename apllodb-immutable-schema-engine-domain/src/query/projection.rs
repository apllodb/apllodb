use apllodb_shared_components::data_structure::ColumnName;
use serde::{Deserialize, Serialize};

/// Projection query for single table column references.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum ProjectionInQuery {
    All,
    ColumnNames(Vec<ColumnName>),
}

pub struct ProjectionResultInVersion {
    effective: Vec<ColumnName>,
    void: Vec<ColumnName>,
}
impl ProjectionResultInVersion {
    pub fn effective_projection(&self) -> &[ColumnName] {
        &self.effective
    }
    
    pub fn void_projection(&self) -> &[ColumnName] {
        &self.void
    }
}
