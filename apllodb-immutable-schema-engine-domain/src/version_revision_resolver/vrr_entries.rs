use std::collections::{HashMap, VecDeque};

use apllodb_storage_engine_interface::StorageEngine;

use crate::{
    abstract_types::ImmutableSchemaAbstractTypes, version::id::VersionId, vtable::id::VTableId,
};

use super::{vrr_entries_in_version::VRREntriesInVersion, vrr_entry::VRREntry};

/// Sequence of VRREntry.
#[derive(Clone, PartialEq, Hash, Debug, new)]
pub struct VRREntries<Engine: StorageEngine, Types: ImmutableSchemaAbstractTypes<Engine>> {
    vtable_id: VTableId,
    inner: VecDeque<VRREntry<Engine, Types>>,
}

impl<'vrr, 'db: 'vrr, Engine: StorageEngine, Types: ImmutableSchemaAbstractTypes<Engine>>
    VRREntries<Engine, Types>
{
    /// Order of VRREntry is kept in each group.
    pub fn group_by_version_id(self) -> Vec<VRREntriesInVersion<Engine, Types>> {
        let mut h: HashMap<VersionId, VecDeque<VRREntry<Engine, Types>>> = HashMap::new();

        for e in self.inner {
            let version_id = &e.version_id;
            h.entry(version_id.clone())
                .and_modify(|entries| {
                    let e = e.clone(); // don't hold e's ownership for or_insert_with.
                    entries.push_back(e);
                })
                .or_insert_with(move || {
                    let mut v = VecDeque::new();
                    v.push_back(e);
                    v
                });
        }

        h.into_iter()
            .map(|(version_id, es)| VRREntriesInVersion::new(version_id, es))
            .collect()
    }

    pub fn vtable_id(&self) -> &VTableId {
        &self.vtable_id
    }
}

impl<'vrr, 'db: 'vrr, Engine: StorageEngine, Types: ImmutableSchemaAbstractTypes<Engine>> Iterator
    for VRREntries<Engine, Types>
{
    type Item = VRREntry<Engine, Types>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.pop_front()
    }
}
