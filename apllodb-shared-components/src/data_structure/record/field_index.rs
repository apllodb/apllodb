use serde::{Deserialize, Serialize};

/// Used to get a value from a record.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct FieldIndex(String);

impl<S: Into<String>> From<S> for FieldIndex {
    fn from(field_name: S) -> Self {
        Self(field_name.into())
    }
}
