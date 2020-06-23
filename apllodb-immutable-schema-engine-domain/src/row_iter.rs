mod version_row_iter;

pub use version_row_iter::VersionRowIter;

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
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let ver_row_iter = self.0.front_mut()?;
        ver_row_iter.next().or_else(|| {
            let _ = self
                .0
                .remove(0)
                .expect("ver_row_iter exists so self.0 has first element");
            self.next()
        })
    }
}
