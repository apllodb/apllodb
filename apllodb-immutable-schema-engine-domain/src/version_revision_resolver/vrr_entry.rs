use crate::{
    row::pk::apparent_pk::ApparentPrimaryKey, row::pk::full_pk::revision::Revision, traits::Entity,
    version::id::VersionId,
};

use super::vrr_id::VRRId;
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct VRREntry {
    id: VRRId,
    pk: ApparentPrimaryKey,
    version_id: VersionId,
    revision: Revision,
}

impl<'a> Entity for VRREntry {
    type Id = VRRId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}
