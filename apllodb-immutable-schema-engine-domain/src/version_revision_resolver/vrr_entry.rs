use crate::{
    row::pk::apparent_pk::ApparentPrimaryKey, row::pk::full_pk::revision::Revision, traits::Entity,
    version::id::VersionId,
};

use super::vrr_id::VRRId;
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct VRREntry<'a> {
    id: &'a VRRId,
    pk: &'a ApparentPrimaryKey,
    version_id: &'a VersionId,
    revision: &'a Revision,
}

impl<'a> Entity for VRREntry<'a> {
    type Id = VRRId;

    fn id(&self) -> &Self::Id {
        &self.id
    }
}
