mod revision;

pub use revision::Revision;

use super::ApparentPrimaryKey;
use apllodb_shared_components::data_structure::ColumnName;
use serde::{Deserialize, Serialize};

/// Primary key with revision.
/// Used for Immutable DML.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, new)]
pub struct FullPrimaryKey {
    apparent_pk: ApparentPrimaryKey,
    revision: Revision,
}

impl FullPrimaryKey {
    pub fn apparent_pk(&self) -> &ApparentPrimaryKey {
        &self.apparent_pk
    }

    pub fn column_names(&self) -> &[ColumnName] {
        use apllodb_storage_engine_interface::PrimaryKey;

        self.apparent_pk.column_names()
    }
}
