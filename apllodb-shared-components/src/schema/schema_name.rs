use crate::SchemaIndex;

/// Fully-qualified names of parts of Schema.
pub trait SchemaName: Clone {
    /// Whether the index matches to this name.
    fn matches(&self, index: &SchemaIndex) -> bool {
        match index.prefix() {
            Some(prefix) => self._prefix_attr_match(prefix, index.attr()),
            None => self._attr_matches(index.attr()),
        }
    }

    /// Whether the attr part of index matches to this name.
    fn _attr_matches(&self, attr: &str) -> bool;

    /// Whether both the attr part and prefix part of index match to this name.
    fn _prefix_attr_match(&self, prefix: &str, attr: &str) -> bool;
}
