mod version_row_iter;

pub use version_row_iter::VersionRowIter;

use apllodb_shared_components::error::ApllodbResult;
use apllodb_storage_engine_interface::Row;
use std::collections::VecDeque;

/// Row iterator combining VersionRowIter from multiple versions.
#[derive(Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct ImmutableSchemaRowIter<I: VersionRowIter>(VecDeque<I>);

impl<I: VersionRowIter> ImmutableSchemaRowIter<I> {
    /// Chain iterators from multiple versions.
    pub fn chain(iters: impl IntoIterator<Item = I>) -> Self {
        Self(iters.into_iter().collect())
    }
}

impl<I: VersionRowIter> Iterator for ImmutableSchemaRowIter<I> {
    type Item = ApllodbResult<Row>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
