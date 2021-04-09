use std::fmt::Display;

/// Key to find RecordPos from a record / row.
///
/// Represented in a string either like "(prefix) . (attr)" or "(attr)".
pub trait SchemaIndex {
    fn new(prefix: Option<String>, attr: String) -> Self;

    fn prefix(&self) -> Option<&str>;

    fn attr(&self) -> &str;
}

impl<I> Display for I
where
    I: SchemaIndex,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = if let Some(p) = self.prefix() {
            format!("{}.", p)
        } else {
            "".to_string()
        };
        write!(f, "{}{}", prefix, self.attr())
    }
}

impl<I> From<&str> for I
where
    I: SchemaIndex,
{
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.split('.').collect();

        debug_assert!(!parts.is_empty());
        assert!(parts.len() <= 2, "too many dots (.) !");

        parts
            .iter()
            .for_each(|part| assert!(!part.is_empty(), "prefix nor attr must not be empty string"));

        let first = parts
            .get(0)
            .expect("must have at least 1 part")
            .trim()
            .to_string();
        let second = parts.get(1).map(|s| s.trim().to_string());

        if let Some(second) = second {
            Self::new(Some(first), second)
        } else {
            Self::new(None, first)
        }
    }
}
