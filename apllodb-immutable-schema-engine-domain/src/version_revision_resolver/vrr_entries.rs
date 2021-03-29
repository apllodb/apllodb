use std::collections::{HashMap, VecDeque};

use crate::{
    abstract_types::ImmutableSchemaAbstractTypes, version::id::VersionId, vtable::id::VTableId,
};

use super::{vrr_entries_in_version::VrrEntriesInVersion, vrr_entry::VrrEntry};

/// Sequence of VrrEntry.
#[derive(Clone, PartialEq, Hash, Debug, new)]
pub struct VrrEntries<Types: ImmutableSchemaAbstractTypes> {
    vtable_id: VTableId,
    inner: VecDeque<VrrEntry<Types>>,
}

impl<Types: ImmutableSchemaAbstractTypes> VrrEntries<Types> {
    /// Order of VrrEntry is kept in each group.
    pub fn group_by_version_id(self) -> Vec<VrrEntriesInVersion<Types>> {
        let mut h: HashMap<VersionId, VecDeque<VrrEntry<Types>>> = HashMap::new();

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
            .map(|(version_id, es)| VrrEntriesInVersion::new(version_id, es))
            .collect()
    }

    pub fn vtable_id(&self) -> &VTableId {
        &self.vtable_id
    }
}

impl<Types: ImmutableSchemaAbstractTypes> Iterator for VrrEntries<Types> {
    type Item = VrrEntry<Types>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.pop_front()
    }
}
