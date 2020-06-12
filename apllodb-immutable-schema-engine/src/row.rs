pub(crate) trait ImmutableSchemaRowIter: Iterator {
    /// Chain iterators from multiple versions.
    fn chain(iters: Vec<Self>) -> Self
    where
        Self: Sized;
}
