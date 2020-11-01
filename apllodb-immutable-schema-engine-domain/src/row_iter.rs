pub(crate) mod version_row_iter;

use std::fmt::Debug;

use version_row_iter::VersionRowIterator;

/// Row iterator combining VersionRowIter from multiple versions.
pub trait ImmutableSchemaRowIterator:
    Iterator<Item = <Self as ImmutableSchemaRowIterator>::I> + Debug
{
    type I: VersionRowIterator;

    /// Chain iterators from multiple versions.
    fn chain(iters: impl IntoIterator<Item = Self::I>) -> Self;

    fn next(&mut self) -> Option<Self::Item>;
}
