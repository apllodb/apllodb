use std::collections::VecDeque;

use crate::{abstract_types::ImmutableSchemaAbstractTypes, version::id::VersionId};

use super::{vrr_entries::VRREntries, vrr_entry::VRREntry};

/// Sequence of VRREntry in a specific Version.
#[derive(Clone, PartialEq, Hash, Debug)]
pub struct VRREntriesInVersion<Types: ImmutableSchemaAbstractTypes> {
    version_id: VersionId,
    vrr_entries: VRREntries<Types>,
}

impl<Types: ImmutableSchemaAbstractTypes> VRREntriesInVersion<Types> {
    pub(in crate::version_revision_resolver) fn new(
        version_id: VersionId,
        vrr_entries_in_version: VecDeque<VRREntry<Types>>,
    ) -> Self {
        let vtable_id = version_id.vtable_id().clone();
        Self {
            version_id,
            vrr_entries: VRREntries::new(vtable_id, vrr_entries_in_version),
        }
    }

    pub fn version_id(&self) -> &VersionId {
        &self.version_id
    }
}

impl<Types: ImmutableSchemaAbstractTypes> Iterator for VRREntriesInVersion<Types> {
    type Item = VRREntry<Types>;

    fn next(&mut self) -> Option<Self::Item> {
        self.vrr_entries.next()
    }
}
