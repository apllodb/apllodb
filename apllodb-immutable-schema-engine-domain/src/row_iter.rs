pub(crate) mod version_row_iter;

use std::fmt::Debug;

use version_row_iter::VersionRowIter;

/// Row iterator combining VersionRowIter from multiple versions.
pub trait ImmutableSchemaRowIter:
    Iterator<Item = <Self as ImmutableSchemaRowIter>::I> + Debug
{
    type I: VersionRowIter;

    /// Chain iterators from multiple versions.
    fn chain(iters: impl IntoIterator<Item = Self::I>) -> Self;

    fn next(&mut self) -> Option<Self::Item>;
}
