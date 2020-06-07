use serde::{Deserialize, Serialize};

/// Used to get a value from a record.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct FieldIndex(String);

impl<S: Into<String>> From<S> for FieldIndex {
    fn from(col_name: S) -> Self {
        Self(col_name.into())
    }
}

// TODO フィールド番号, alias名 からもつくれるように
