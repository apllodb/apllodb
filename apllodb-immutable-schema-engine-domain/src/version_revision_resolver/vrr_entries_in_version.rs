use std::collections::VecDeque;

use crate::{abstract_types::ImmutableSchemaAbstractTypes, version::id::VersionId};

use super::{vrr_entries::VrrEntries, vrr_entry::VrrEntry};

/// Sequence of VrrEntry in a specific Version.
#[derive(Clone, PartialEq, Hash, Debug)]
pub struct VrrEntriesInVersion<Types: ImmutableSchemaAbstractTypes> {
    version_id: VersionId,
    vrr_entries: VrrEntries<Types>,
}

impl<Types: ImmutableSchemaAbstractTypes> VrrEntriesInVersion<Types> {
    pub(in crate::version_revision_resolver) fn new(
        version_id: VersionId,
        vrr_entries_in_version: VecDeque<VrrEntry<Types>>,
    ) -> Self {
        let vtable_id = version_id.vtable_id().clone();
        Self {
            version_id,
            vrr_entries: VrrEntries::new(vtable_id, vrr_entries_in_version),
        }
    }

    pub fn version_id(&self) -> &VersionId {
        &self.version_id
    }
}

impl<Types: ImmutableSchemaAbstractTypes> Iterator for VrrEntriesInVersion<Types> {
    type Item = VrrEntry<Types>;

    fn next(&mut self) -> Option<Self::Item> {
        self.vrr_entries.next()
    }
}
