use serde::{Deserialize, Serialize};

/// ORDER BY (ASC | DESC)
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Serialize, Deserialize)]
pub enum Ordering {
    /// Ascending order
    Asc,
    /// Descending order
    Desc,
}
