use super::{Version, VersionId};
use crate::entity::Entity;
use serde::{Deserialize, Serialize};

/// Inactive Version.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct InactiveVersion(Version);

impl Entity for InactiveVersion {
    type Id = VersionId;

    fn id(&self) -> &Self::Id {
        &self.0.id
    }
}
