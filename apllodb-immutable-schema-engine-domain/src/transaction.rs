use apllodb_shared_components::{data_structure::DatabaseName, error::ApllodbResult};
use apllodb_storage_engine_interface::{StorageEngine, Transaction};
use std::fmt::Debug;

use crate::abstract_types::ImmutableSchemaAbstractTypes;

/// Operations a transaction implementation for Immutable Schema must have.
///
/// Meant to be called from implementations of [Transaction](foo.html) (logical transaction interface) internally as physical transaction.
pub trait ImmutableSchemaTransaction<
    'tx,
    'db: 'tx,
    Engine: StorageEngine<'db>,
    Types: ImmutableSchemaAbstractTypes<'tx, 'db, Engine>,
>: Transaction<'db, Engine> + Sized + 'tx
{
    fn vtable_repo(&'tx self) -> Types::VTableRepo;
    fn version_repo(&'tx self) -> Types::VersionRepo;
}
