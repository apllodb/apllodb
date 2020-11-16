use std::collections::VecDeque;

use apllodb_storage_engine_interface::StorageEngine;

use crate::{abstract_types::ImmutableSchemaAbstractTypes, version::id::VersionId};

use super::{vrr_entries::VRREntries, vrr_entry::VRREntry};

/// Sequence of VRREntry in a specific Version.
/// Must have at least 1 entry.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct VRREntriesInVersion<
    'vrr,
    'db: 'vrr,
    Engine: StorageEngine<'vrr, 'db>,
    Types: ImmutableSchemaAbstractTypes<'vrr, 'db, Engine>,
> {
    version_id: VersionId,
    vrr_entries: VRREntries<'vrr, 'db, Engine, Types>,
}

impl<
        'vrr,
        'db: 'vrr,
        Engine: StorageEngine<'vrr, 'db>,
        Types: ImmutableSchemaAbstractTypes<'vrr, 'db, Engine>,
    > VRREntriesInVersion<'vrr, 'db, Engine, Types>
{
    pub(in crate::version_revision_resolver) fn new(
        vrr_entries_in_version: VecDeque<VRREntry<'vrr, 'db, Engine, Types>>,
    ) -> Self {
        assert!(!vrr_entries_in_version.is_empty(),);
        Self {
            version_id: vrr_entries_in_version
                .front()
                .expect("VRREntriesInVersion must have at least 1 element.")
                .version_id
                .clone(),
            vrr_entries: VRREntries::new(vrr_entries_in_version),
        }
    }

    pub fn version_id(&self) -> &VersionId {
        &self.version_id
    }
}

impl<
        'vrr,
        'db: 'vrr,
        Engine: StorageEngine<'vrr, 'db>,
        Types: ImmutableSchemaAbstractTypes<'vrr, 'db, Engine>,
    > Iterator for VRREntriesInVersion<'vrr, 'db, Engine, Types>
{
    type Item = VRREntry<'vrr, 'db, Engine, Types>;

    fn next(&mut self) -> Option<Self::Item> {
        self.vrr_entries.next()
    }
}
