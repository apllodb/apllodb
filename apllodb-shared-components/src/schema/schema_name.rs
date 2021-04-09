use crate::Schema;

/// Fully-qualified names of parts of Schema.
pub trait SchemaName<S: Schema>: Clone {
    /// Whether the index matches to this name.
    fn matches(&self, index: &S::Index) -> bool;
}
