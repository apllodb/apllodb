use std::fmt::Debug;

use apllodb_storage_engine_interface::StorageEngine;

use crate::{
    row_iter::version_row_iter::VersionRowIterator, version::repository::VersionRepository,
    vtable::repository::VTableRepository,
};

/// Types that must be implemented in an infrastructure layer.
pub trait ImmutableSchemaAbstractTypes<'tx, 'db: 'tx, Engine: StorageEngine<'db>>:
    Debug + Sized
{
    type VersionRowIter: VersionRowIterator;

    type VersionRepo: VersionRepository<'tx, 'db, Engine>;
    type VTableRepo: VTableRepository<'tx, 'db, Engine>;
}
