use std::fmt::Debug;

use apllodb_storage_engine_interface::StorageEngine;

use crate::{
    row_iter::{version_row_iter::VersionRowIterator, ImmutableSchemaRowIterator},
    transaction::ImmutableSchemaTransaction,
    version::repository::VersionRepository,
    vtable::repository::VTableRepository,
};

/// Types that must be implemented in an infrastructure layer.
pub trait ImmutableSchemaAbstractTypes<Engine: StorageEngine>: Debug + Sized {
    type VersionRowIter: VersionRowIterator;
    type ImmutableSchemaRowIter: ImmutableSchemaRowIterator<Engine, Self>;

    type ImmutableSchemaTx: ImmutableSchemaTransaction<Engine, Self>;

    type VersionRepo: VersionRepository<Engine, Self>;
    type VTableRepo: VTableRepository<Engine, Self>;
}
