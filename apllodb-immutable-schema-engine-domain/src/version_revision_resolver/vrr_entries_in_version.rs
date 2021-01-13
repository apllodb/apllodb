use std::collections::VecDeque;

use apllodb_storage_engine_interface::StorageEngine;

use crate::{abstract_types::ImmutableSchemaAbstractTypes, version::id::VersionId};

use super::{vrr_entries::VRREntries, vrr_entry::VRREntry};

/// Sequence of VRREntry in a specific Version.
#[derive(Clone, PartialEq, Hash, Debug)]
pub struct VRREntriesInVersion<
    'sess,
    Engine: StorageEngine<'sess>,
    Types: ImmutableSchemaAbstractTypes<'sess, Engine>,
> {
    version_id: VersionId,
    vrr_entries: VRREntries<'sess, Engine, Types>,
}

impl<'sess, Engine: StorageEngine<'sess>, Types: ImmutableSchemaAbstractTypes<'sess, Engine>>
    VRREntriesInVersion<'sess, Engine, Types>
{
    pub(in crate::version_revision_resolver) fn new(
        version_id: VersionId,
        vrr_entries_in_version: VecDeque<VRREntry<'sess, Engine, Types>>,
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

impl<'sess, Engine: StorageEngine<'sess>, Types: ImmutableSchemaAbstractTypes<'sess, Engine>>
    Iterator for VRREntriesInVersion<'sess, Engine, Types>
{
    type Item = VRREntry<'sess, Engine, Types>;

    fn next(&mut self) -> Option<Self::Item> {
        self.vrr_entries.next()
    }
}
