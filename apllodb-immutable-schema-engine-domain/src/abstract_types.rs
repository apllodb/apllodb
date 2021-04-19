use std::fmt::Debug;

use crate::{
    version::repository::VersionRepository, version_revision_resolver::vrr_id::VrrId,
    vtable::repository::VTableRepository,
};

/// Types that must be implemented in an infrastructure layer.
pub trait ImmutableSchemaAbstractTypes: Debug + Sized {
    type VrrId: VrrId;

    type VTableRepo: VTableRepository<Self>;
    type VersionRepo: VersionRepository;
}
