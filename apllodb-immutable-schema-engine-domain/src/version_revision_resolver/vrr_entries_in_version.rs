use std::collections::VecDeque;

use apllodb_storage_engine_interface::StorageEngine;

use crate::{abstract_types::ImmutableSchemaAbstractTypes, version::id::VersionId};

use super::{vrr_entries::VRREntries, vrr_entry::VRREntry};

/// Sequence of VRREntry in a specific Version.
#[derive(Clone, PartialEq, Hash, Debug)]
pub struct VRREntriesInVersion<
    'vrr,
    'db: 'vrr,
    Engine: StorageEngine,
    Types: ImmutableSchemaAbstractTypes<'vrr, 'db, Engine>,
> {
    version_id: VersionId,
    vrr_entries: VRREntries<'vrr, 'db, Engine, Types>,
}

impl<
        'vrr,
        'db: 'vrr,
        Engine: StorageEngine,
        Types: ImmutableSchemaAbstractTypes<'vrr, 'db, Engine>,
    > VRREntriesInVersion<'vrr, 'db, Engine, Types>
{
    pub(in crate::version_revision_resolver) fn new(
        version_id: VersionId,
        vrr_entries_in_version: VecDeque<VRREntry<'vrr, 'db, Engine, Types>>,
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

impl<
        'vrr,
        'db: 'vrr,
        Engine: StorageEngine,
        Types: ImmutableSchemaAbstractTypes<'vrr, 'db, Engine>,
    > Iterator for VRREntriesInVersion<'vrr, 'db, Engine, Types>
{
    type Item = VRREntry<'vrr, 'db, Engine, Types>;

    fn next(&mut self) -> Option<Self::Item> {
        self.vrr_entries.next()
    }
}
