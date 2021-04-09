use crate::Schema;

/// Fully-qualified names of parts of Schema.
pub trait SchemaName<S: Schema>: Clone {
    fn matches(&self, index: &S::Index) -> bool;
}
