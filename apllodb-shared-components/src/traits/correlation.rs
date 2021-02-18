use crate::CorrelationName;

/// One of:
/// - table name or its alias
/// - sub-query's alias
pub(crate) trait Correlation {
    fn is_named(&self, correlation_name: &CorrelationName) -> bool;
}
