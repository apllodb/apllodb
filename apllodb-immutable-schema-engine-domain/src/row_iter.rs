pub mod version_row_iter;

use std::fmt::Debug;

use apllodb_storage_engine_interface::StorageEngine;

use crate::{abstract_types::ImmutableSchemaAbstractTypes, row::immutable_row::ImmutableRow};

/// Row iterator combining VersionRowIter from multiple versions.
pub trait ImmutableSchemaRowIterator<
    'repo,
    'db: 'repo,
    Engine: StorageEngine<'repo, 'db>,
    Types: ImmutableSchemaAbstractTypes<'repo, 'db, Engine>,
>: Iterator<Item = ImmutableRow> + Debug
{
    /// Chain iterators from multiple versions.
    fn chain(iters: impl IntoIterator<Item = Types::VersionRowIter>) -> Self;
}
