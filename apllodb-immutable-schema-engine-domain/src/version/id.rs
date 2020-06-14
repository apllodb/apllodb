use crate::{vtable::VTableId, VersionNumber};
use serde::{Deserialize, Serialize};

/// ID of VTable
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct VersionId {
    pub(in crate::version) vtable_id: VTableId,
    pub(in crate::version) version_number: VersionNumber,
}

impl VersionId {
    pub(in crate::version) fn new(vtable_id: &VTableId, version_number: &VersionNumber) -> Self {
        Self {
            vtable_id: vtable_id.clone(),
            version_number: version_number.clone(),
        }
    }
}
