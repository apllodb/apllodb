use std::collections::{HashMap, VecDeque};

use apllodb_storage_engine_interface::StorageEngine;

use crate::{
    abstract_types::ImmutableSchemaAbstractTypes, version::id::VersionId, vtable::id::VTableId,
};

use super::{vrr_entries_in_version::VRREntriesInVersion, vrr_entry::VRREntry};

/// Sequence of VRREntry.
/// Must have at least 1 entry.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct VRREntries<
    'vrr,
    'db: 'vrr,
    Engine: StorageEngine<'vrr, 'db>,
    Types: ImmutableSchemaAbstractTypes<'vrr, 'db, Engine>,
>(VecDeque<VRREntry<'vrr, 'db, Engine, Types>>);

impl<
        'vrr,
        'db: 'vrr,
        Engine: StorageEngine<'vrr, 'db>,
        Types: ImmutableSchemaAbstractTypes<'vrr, 'db, Engine>,
    > VRREntries<'vrr, 'db, Engine, Types>
{
    pub(in crate::version_revision_resolver) fn new(
        inner: VecDeque<VRREntry<'vrr, 'db, Engine, Types>>,
    ) -> Self {
        assert!(
            !inner.is_empty(),
            "VRREntries must have at least 1 element."
        );
        Self(inner)
    }

    /// Order of VRREntry is kept in each group.
    pub fn group_by_version_id(
        self,
    ) -> Vec<(VersionId, VRREntriesInVersion<'vrr, 'db, Engine, Types>)> {
        let mut h: HashMap<VersionId, VecDeque<VRREntry<'vrr, 'db, Engine, Types>>> =
            HashMap::new();

        for e in self.0 {
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
            .map(|(version_id, es)| {
                let vrr_entries_in_version = VRREntriesInVersion::new(es);
                (version_id, vrr_entries_in_version)
            })
            .collect()
    }

    pub fn vtable_id(&self) -> VTableId {
        let e = self.0.front().expect("must have at least 1 element");
        e.version_id.vtable_id().clone()
    }
}

impl<
        'vrr,
        'db: 'vrr,
        Engine: StorageEngine<'vrr, 'db>,
        Types: ImmutableSchemaAbstractTypes<'vrr, 'db, Engine>,
    > Iterator for VRREntries<'vrr, 'db, Engine, Types>
{
    type Item = VRREntry<'vrr, 'db, Engine, Types>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}
