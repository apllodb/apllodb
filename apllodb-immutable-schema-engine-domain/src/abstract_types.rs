use std::fmt::Debug;

use apllodb_shared_components::traits::Database;
use apllodb_storage_engine_interface::TransactionId;

use crate::{
    row_iter::{version_row_iter::VersionRowIterator, ImmutableSchemaRowIterator},
    transaction::ImmutableSchemaTx,
    version::repository::VersionRepository,
    vtable::repository::VTableRepository,
};

/// Types that must be implemented in an infrastructure layer.
pub trait ImmutableSchemaAbstractTypes<'tx, 'db: 'tx>: Debug + Sized + 'db {
    type VersionRowIter: VersionRowIterator;
    type ImmutableSchemaRowIter: ImmutableSchemaRowIterator<'tx, 'db, Self>;

    type TID: TransactionId;
    type Tx: ImmutableSchemaTx<'tx, 'db, Self>;
    type Db: Database + 'db;

    type VersionRepo: VersionRepository<'tx, 'db, Self>;
    type VTableRepo: VTableRepository<'tx, 'db, Self>;
}
