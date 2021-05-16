use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

use crate::SqlValue;

/// `Eq + Hash` hash key used for hash algorithms.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub struct SqlValueHashKey(u64);

impl From<&SqlValue> for SqlValueHashKey {
    fn from(sql_value: &SqlValue) -> Self {
        let mut hasher = DefaultHasher::new();
        sql_value.hash(&mut hasher);
        Self(hasher.finish())
    }
}
