pub mod version_row_iter;

use std::fmt::Debug;

use crate::{abstract_types::AbstractTypes, row::immutable_row::ImmutableRow};

/// Row iterator combining VersionRowIter from multiple versions.
pub trait ImmutableSchemaRowIterator<'tx, 'db: 'tx, Types: AbstractTypes<'tx, 'db>>:
    Iterator<Item = ImmutableRow> + Debug
{
    /// Chain iterators from multiple versions.
    fn chain(iters: impl IntoIterator<Item = Types::VersionRowIter>) -> Self;
}
