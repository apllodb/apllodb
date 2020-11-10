use apllodb_shared_components::{data_structure::DatabaseName, error::ApllodbResult};
use apllodb_storage_engine_interface::StorageEngine;
use std::fmt::Debug;

use crate::abstract_types::ImmutableSchemaAbstractTypes;

/// Operations a transaction implementation for Immutable Schema must have.
///
/// Meant to be called from implementations of [Transaction](foo.html) (logical transaction interface) internally as physical transaction.
pub trait ImmutableSchemaTx<
    'tx,
    'db: 'tx,
    Engine: StorageEngine<'tx, 'db>,
    Types: ImmutableSchemaAbstractTypes<'tx, 'db, Engine>,
>: Debug + Sized
{
    fn id(&self) -> &Engine::TID;

    fn begin(db: &'db mut Engine::Db) -> ApllodbResult<Self>
    where
        Self: Sized;

    fn commit(self) -> ApllodbResult<()>
    where
        Self: Sized;

    fn abort(self) -> ApllodbResult<()>
    where
        Self: Sized;

    fn database_name(&self) -> &DatabaseName;

    fn vtable_repo(&'tx self) -> Types::VTableRepo;
    fn version_repo(&'tx self) -> Types::VersionRepo;
}
