use crate::SchemaIndex;

/// Fully-qualified names of parts of Schema.
pub trait SchemaName: Clone {
    /// Whether the index matches to this name.
    fn matches(&self, index: &SchemaIndex) -> bool;
}
