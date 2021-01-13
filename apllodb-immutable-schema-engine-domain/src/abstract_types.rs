use std::fmt::Debug;

use apllodb_storage_engine_interface::StorageEngine;

use crate::{
    row_iter::{version_row_iter::VersionRowIterator, ImmutableSchemaRowIterator},
    version::repository::VersionRepository,
    version_revision_resolver::vrr_id::VRRId,
    vtable::repository::VTableRepository,
};

/// Types that must be implemented in an infrastructure layer.
pub trait ImmutableSchemaAbstractTypes<'sess, Engine: StorageEngine<'sess>>: Debug + Sized {
    type VRRId: VRRId;

    type ImmutableSchemaRowIter: ImmutableSchemaRowIterator<'sess, Engine, Self>;
    type VersionRowIter: VersionRowIterator;

    type VTableRepo: VTableRepository<'sess, Engine, Self>;
    type VersionRepo: VersionRepository<'sess, Engine>;
}
