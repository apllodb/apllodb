mod revision;

pub use revision::Revision;

use super::ApparentPrimaryKey;
use crate::row::column::pk_column::PKColumnName;
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

    pub fn column_names(&self) -> &[PKColumnName] {
        self.apparent_pk.column_names()
    }
}
