use std::collections::HashMap;

use crate::{version::id::VersionId, vtable::id::VTableId};

use super::vrr_entry::VRREntry;

/// Sequence of VRREntry.
/// Must have at least 1 entry.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct VRREntries(Vec<VRREntry>);

impl VRREntries {
    fn new(inner: Vec<VRREntry>) -> Self {
        assert!(
            !inner.is_empty(),
            "VRREntries must have at least 1 element."
        );
        Self(inner)
    }

    /// Order of VRREntry is kept in each group.
    pub fn group_by_version_id(self) -> Vec<(VersionId, Self)> {
        let mut h: HashMap<VersionId, Vec<VRREntry>> = HashMap::new();

        for e in self.0 {
            let version_id = &e.version_id;
            h.entry(version_id.clone())
                .and_modify(|entries| {
                    let e = e.clone(); // don't hold r's ownership for or_insert_with.
                    entries.push(e);
                })
                .or_insert_with(move || vec![e]);
        }

        h.into_iter()
            .map(|(version_id, es)| (version_id, Self::new(es)))
            .collect()
    }

    pub fn vtable_id(&self) -> VTableId {
        let e = self.0.first().expect("must have at least 1 element");
        e.version_id.vtable_id().clone()
    }
}
