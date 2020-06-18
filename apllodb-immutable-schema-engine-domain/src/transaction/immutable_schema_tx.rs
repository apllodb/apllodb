use crate::{VTableRepository, VersionRepository};
use apllodb_shared_components::{
    data_structure::DatabaseName, error::ApllodbResult, traits::Database,
};
use std::fmt::Debug;

/// Operations a transaction implementation for Immutable Schema must have.
///
/// Meant to be called from implementations of [Transaction](foo.html) (logical transaction interface) internally as physical transaction.
pub trait ImmutableSchemaTx<'tx, 'db: 'tx>: Debug + Sized {
    type Db: Database + 'db;

    type VTRepo: VTableRepository<'tx, 'db, Tx = Self>;
    type VRepo: VersionRepository<'tx, 'db, Tx = Self>;

    fn begin(db: &'db mut Self::Db) -> ApllodbResult<Self>
    where
        Self: Sized;

    fn commit(self) -> ApllodbResult<()>
    where
        Self: Sized;

    fn abort(self) -> ApllodbResult<()>
    where
        Self: Sized;

    fn database_name(&self) -> &DatabaseName;

    fn vtable_repo(&'tx self) -> Self::VTRepo;
    fn version_repo(&'tx self) -> Self::VRepo;
}
