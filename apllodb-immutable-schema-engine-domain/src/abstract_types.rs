use std::fmt::Debug;

use apllodb_storage_engine_interface::StorageEngine;

use crate::{
    row_iter::{version_row_iter::VersionRowIterator, ImmutableSchemaRowIterator},
    version::repository::VersionRepository,
    version_revision_resolver::vrr_id::VRRId,
    vtable::repository::VTableRepository,
};

/// Types that must be implemented in an infrastructure layer.
pub trait ImmutableSchemaAbstractTypes<'repo, 'db: 'repo, Engine: StorageEngine<'repo, 'db>>:
    Debug + Sized
{
    type VRRId: VRRId;

    type ImmutableSchemaRowIter: ImmutableSchemaRowIterator<'repo, 'db, Engine, Self>;
    type VersionRowIter: VersionRowIterator;

    type VTableRepo: VTableRepository<'repo, 'db, Engine, Self>;
    type VersionRepo: VersionRepository<'repo, 'db, Engine>;
}
