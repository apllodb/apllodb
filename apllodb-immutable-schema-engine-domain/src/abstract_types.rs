use std::fmt::Debug;

use apllodb_storage_engine_interface::StorageEngine;

use crate::{
    row_iter::{version_row_iter::VersionRowIterator, ImmutableSchemaRowIterator},
    version::repository::VersionRepository,
    vtable::repository::VTableRepository,
};

/// Types that must be implemented in an infrastructure layer.
pub trait ImmutableSchemaAbstractTypes<'tx, 'db: 'tx, Engine: StorageEngine>:
    Debug + Sized
{
    type VersionRowIter: VersionRowIterator;
    type ImmutableSchemaRowIter: ImmutableSchemaRowIterator<'tx, 'db, Engine, Self>;

    type VersionRepo: VersionRepository<'tx, 'db, Engine, Self>;
    type VTableRepo: VTableRepository<'tx, 'db, Engine, Self>;
}
