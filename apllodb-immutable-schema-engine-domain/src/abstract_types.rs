use std::fmt::Debug;

use apllodb_storage_engine_interface::StorageEngine;

use crate::{
    row_iter::version_row_iter::VersionRowIterator, transaction::ImmutableSchemaTransaction,
    version::repository::VersionRepository, vtable::repository::VTableRepository,
};

/// Types that must be implemented in an infrastructure layer.
pub trait ImmutableSchemaAbstractTypes<'tx, 'db: 'tx, Engine: StorageEngine<'db>>:
    Debug + Sized
{
    type VersionRowIter: VersionRowIterator;

    type ImmutableSchemaTx: ImmutableSchemaTransaction<'tx, 'db, Engine, Self>;

    type VersionRepo: VersionRepository<'tx, 'db, Engine, Self>;
    type VTableRepo: VTableRepository<'tx, 'db, Engine, Self>;
}
