use super::version_number::VersionNumber;
use serde::{Deserialize, Serialize};
use crate::vtable::id::VTableId;

/// ID of VTable
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct VersionId {
    pub(in crate::version) vtable_id: VTableId,
    pub(in crate::version) version_number: VersionNumber,
}

impl VersionId {
    pub fn new(vtable_id: &VTableId, version_number: &VersionNumber) -> Self {
        Self {
            vtable_id: vtable_id.clone(),
            version_number: version_number.clone(),
        }
    }

    pub fn vtable_id(&self) -> &VTableId {
        &self.vtable_id
    }

    pub fn version_number(&self) -> &VersionNumber {
        &self.version_number
    }
}
