use std::fmt::Debug;

use crate::{
    row_iter::{version_row_iter::VersionRowIterator, ImmutableSchemaRowIterator},
    version::repository::VersionRepository,
    version_revision_resolver::vrr_id::VRRId,
    vtable::repository::VTableRepository,
};

/// Types that must be implemented in an infrastructure layer.
pub trait ImmutableSchemaAbstractTypes: Debug + Sized {
    type VRRId: VRRId;

    type ImmutableSchemaRowIter: ImmutableSchemaRowIterator<Self>;
    type VersionRowIter: VersionRowIterator;

    type VTableRepo: VTableRepository<Self>;
    type VersionRepo: VersionRepository;
}
