pub mod revision;

use super::apparent_pk::ApparentPrimaryKey;
use apllodb_storage_engine_interface::ColumnName;
use revision::Revision;
use serde::{Deserialize, Serialize};

/// Primary key with revision.
/// Used for Immutable DML.
#[derive(Clone, PartialEq, Hash, Debug, Serialize, Deserialize, new)]
pub struct FullPrimaryKey {
    apparent_pk: ApparentPrimaryKey,
    revision: Revision,
}

impl FullPrimaryKey {
    pub fn apparent_pk(&self) -> &ApparentPrimaryKey {
        &self.apparent_pk
    }

    pub fn column_names(&self) -> &[ColumnName] {
        self.apparent_pk.column_names()
    }
}
